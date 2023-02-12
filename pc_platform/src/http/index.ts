import axios, {AxiosInstance, AxiosError, AxiosRequestConfig, AxiosResponse, CreateAxiosDefaults} from "axios";
import { AxiosCanceler } from "./axios-cancel";
import { checkStatus } from "./check-status";
import {openNotificationWithIcon} from "@/utils/window";
import {doRefreshToken} from "@/http/token";
import {openLoginWindow} from "@/windows/actions";



enum ResultEnum {
    SUCCESS = 200,
    ERROR = 500,
    // 会话过期状态码
    OVERDUE = 401,
    // 请求超时时间
    TIMEOUT = 60000,
    TYPE = "success"
}


interface Result {
    code: number;
    message: string;
}

// * 请求响应参数(包含data)
interface ResultData<T = any> extends Result {
    data?: T;
}


const axiosCanceler = new AxiosCanceler();

const config = {
    // 默认地址请求地址，可在 .env 开头文件中修改
    baseURL: import.meta.env.VITE_APP_BASE_API as string,
    // 设置超时时间（10s）
    timeout: ResultEnum.TIMEOUT as number,
    // 跨域时候允许携带凭证
    withCredentials: true
};

class RequestHttp {
    service: AxiosInstance;
    access_token: string | null | unknown;

    constructor(config) {
        // 实例化axios
        this.service = axios.create(config);

        /**
         * @description 请求拦截器
         */
        this.service.interceptors.request.use(
            (config: AxiosRequestConfig) => {
                axiosCanceler.addPending(config);
                // 需要添加的token 自行设置
                if (this.access_token && '/backend/login' !== config.url){
                    config.headers.access_token = this.access_token;
                }
                return config;
            },
            (error: AxiosError) => {
                return Promise.reject(error);
            }
        );

        /**
         * @description 响应拦截器
         */
        this.service.interceptors.response.use(
            (response: AxiosResponse) => {
                const {data, config} = response;
                // * 在请求结束后，移除本次请求
                axiosCanceler.removePending(config);
                // 会话过期操作
                if (data.code == ResultEnum.OVERDUE) {
                    openNotificationWithIcon("error", "错误提示", '您已经长时间未操作，请重新登录！');
                    openLoginWindow()
                    return Promise.reject(data);
                }
                // * 全局错误信息拦截（防止下载文件得时候返回数据流，没有code，直接报错）
                // if (data.code && data.code !== ResultEnum.SUCCESS) {
                //     return Promise.reject(data);
                // }
                // * 成功请求
                return data;
            },
            async (error: AxiosError) => {
                const {response} = error;
                // 根据响应的错误状态码，做不同的处理
                if (response) return checkStatus(response.status);
                // 服务器结果都没有返回(可能服务器错误可能客户端断网)，断网处理:可以跳转到断网页面
                if (!window.navigator.onLine) return
                return Promise.reject(error);
            }
        );
    }

    // * 常用请求方法封装
    async get<T>(url: string, params?: any, _object = {}): Promise<AxiosResponse<any>> {
        this.access_token = await doRefreshToken(url);
        //console.log(this.access_token);
        return this.service.get(url, {params, ..._object});
    }
    async post<T>(url: string, params?: object, _object = {}): Promise<AxiosResponse<any>> {
        // 判断是否需要刷新token
        this.access_token = await doRefreshToken(url);
        //console.log(this.access_token);
        return this.service.post(url, params, _object);
    }
    async put<T>(url: string, params?: object, _object = {}): Promise<AxiosResponse<any>> {
        this.access_token = await doRefreshToken(url);
        //console.log(this.access_token);
        return this.service.put(url, params, _object);
    }
    async delete<T>(url: string, params?: any, _object = {}): Promise<AxiosResponse<any>> {
        this.access_token = await doRefreshToken(url);
        //console.log(this.access_token);
        return this.service.delete(url, {params, ..._object});
    }
}

export default new RequestHttp(config);