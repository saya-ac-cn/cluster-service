import axios, {AxiosRequestConfig} from 'axios'
import {refreshTokenApi} from './api'
import Storage from '@/utils/storage'
import jwt_decode from "jwt-decode";
import {openNotificationWithIcon} from "@/utils/window";
import {checkStatus} from "@/http/check-status";
import {openLoginWindow} from '@/windows/actions'



// 刷新token
export const doRefreshToken = (url: string) => {
    if ('/backend/login' === url){
        Storage.removeAll();
        return new Promise((resolve, reject) => {resolve(null)})
    }

    return new Promise((resolve, reject) => {
        // 当前缓存中的token
        let access_token = Storage.get(Storage.ACCESS_KEY)
        if(access_token){
            // 解码jwt
            let token = jwt_decode(access_token)
            // 抓取jwt创建时间
            const exp = token.exp
            const current = parseInt(String(new Date().valueOf() / 1000))
            const distant = current - exp
            // 若是local storage 中有值，
            // 1、如果已经过期（token创建时间已经过了30分钟），则直接清除所有的缓存，并重定向到登录页面
            // 2、如果邻近过期（token创建时间已经过了25分钟，但是小于30分钟），则刷新一次
            // 3、其它情况（token创建时间已经过的时间不足25分钟）
            if (distant > 1800){
                openNotificationWithIcon("error", "错误提示", '您已经长时间未操作，请重新登录！');
                Storage.removeAll();
                openLoginWindow();
                reject('您已经长时间未操作，请重新登录！')
            }else if (distant > 1500){
                const _config = { headers: { 'access_token':access_token} }
                // const defaults: AxiosRequestConfig = {
                //     timeout: 0,
                //     headers: {
                //         access_token: access_token
                //     }
                // };
                //defaults.headers = { headers: { 'access_token':access_token} }
                axios.post(refreshTokenApi, {}, _config).then(response => {
                    Storage.add(Storage.ACCESS_KEY,response.data.data)
                    access_token = response.data.data
                    resolve(access_token)
                },error => {
                    const { response: { status, data: { msg = '服务器发生错误' } }} = error;
                    // 根据响应的错误状态码，做不同的处理
                    if (status === 401) {
                        Storage.removeAll();
                        openNotificationWithIcon("error", "错误提示", '您已经长时间未操作，请重新登录！');
                        openLoginWindow()
                    }else{
                        checkStatus(status);
                    }
                    reject('您已经离线！')
                }).catch(error => {
                    openNotificationWithIcon("error", "请求出错了", error.message);
                    reject(error)
                });
            }else{
                // 空转，不用刷新
            }
        }
        resolve(access_token)
    })
}