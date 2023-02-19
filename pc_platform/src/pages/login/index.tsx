import {useState,useEffect} from "react";
import { Form, Input, Button, Checkbox } from 'antd';
import "./index.less"
import {clearTrimValueEvent} from "@/utils/string"
import Storage from '@/utils/storage'
import {openNotificationWithIcon} from "@/utils/window";
import { appWindow } from '@tauri-apps/api/window'
import {loginApi,ownOrganizeUserApi} from "@/http/api"
import { invoke } from '@tauri-apps/api/tauri'
import {openStageWindow} from "@/windows/actions";
import {isEmptyObject} from "@/utils/var";
/**
 * 表单布局
 */
const layout = {
    labelCol: { span: 8 },
    wrapperCol: { span: 16 },
};
const tailLayout = {
    wrapperCol: { offset: 8, span: 16 },
};

const Login = () => {

    const [loinForm] = Form.useForm();
    const [user, setUser] = useState({account:'',password:'',remember:false});
    const [loading, setLoading] = useState(false);

    useEffect(()=>{
        checkLogin()
        rememberMeData()
    },[])

    const checkLogin = () => {
        // 缓存数据缺一不可
        const access_token = Storage.get(Storage.ACCESS_KEY)
        if (isEmptyObject(access_token)){
            return
        }
        const user = Storage.get(Storage.USER_KEY)
        if (isEmptyObject(user)){
            return
        }
        const plan = Storage.get(Storage.PLAN_KEY)
        if (isEmptyObject(plan)){
            return
        }
        const log = Storage.get(Storage.LOG_KEY)
        if (isEmptyObject(log)){
            return
        }
        const organize = Storage.get(Storage.ORGANIZE_KEY)
        if (isEmptyObject(organize)){
            return
        }
        openStageWindow()
    }

    /**
     * 读取记住我的信息
     */
    const rememberMeData = () => {
        const remember = Storage.get(Storage.LOGIN_KEY)
        if (!remember){
            return
        }
        const user = {account:remember.account,password:'',remember:false}
        setUser(user)
        loinForm.setFieldsValue(user);
    }

    const onFinish = () => {
        setLoading(true)
        loinForm.validateFields().then((values) => {
            loginHandle(values)
        }).catch((info) => {
            setLoading(false);
            console.log('表单校验不通过:', info);
        });
    };

    const loginHandle = async (values:any) => {
        let loginParam = {account: values.account, password: values.password};
        // invoke('login',{arg: loginParam}).then((message) => console.log(message))
        //     .catch((error) => console.error('error',error))
        const result = await loginApi(loginParam).catch(()=>{setLoading(false)});
        setLoading(false);
        let {code, data} = result;
        if (code === 0) {
           let {access_token,log,plan,user} = data
           // 保存到local中
           Storage.add(Storage.ACCESS_KEY,access_token)
           Storage.add(Storage.USER_KEY,user)
           Storage.add(Storage.PLAN_KEY,plan)
           Storage.add(Storage.LOG_KEY,log)
           // 获取组织用户列表信息
           await getOwnOrganizeUser()
           if (values.remember){
               const cache = { account: values.account}
               Storage.add(Storage.LOGIN_KEY,cache)
           }
           openStageWindow()
        } else if (code === 5) {
           openNotificationWithIcon("error", "错误提示", '请输入用户名和密码');
        } else {
           openNotificationWithIcon("error", "错误提示", '用户名或密码错误');
        }
    }

    // 关闭
    const handleAppClose = async() => {
        await appWindow.hide()
    }

    /**
     * 获取自己所在组织下的用户
     */
    const getOwnOrganizeUser = async () => {
        // 发异步ajax请求, 获取数据
        const {msg, code, data} = await ownOrganizeUserApi()
        if (code === 0) {
            let organize = {};
            for (let index in data) {
                const item = data[index]
                organize[item.account] = item.name
            }
            Storage.add(Storage.ORGANIZE_KEY,organize)
        } else {
            openNotificationWithIcon("error", "错误提示", msg);
        }
    }

    return (
        <div style={{backgroundImage: `url('/picture/login/login_background.png')`}} className='login-page'>
            <div data-tauri-drag-region className='window-title'>
                <a onClick={handleAppClose} className='light red'/>
                <a className='light yellow'/>
                <a className='light green'/>
            </div>
            <Form {...layout} name="login" form={loinForm}>
                <h2 className="title">统一身份认证入口</h2>
                <Form.Item label="账号" name="account" initialValue={user.account} getValueFromEvent={(e) => clearTrimValueEvent(e.target.value)} rules={[{ required: true, message: '请输入账号!' }]}>
                    <Input maxLength={32}/>
                </Form.Item>

                <Form.Item label="密码" name="password" initialValue={user.password} getValueFromEvent={(e) => clearTrimValueEvent(e.target.value)} rules={[{ required: true, message: '请输入密码!' }]}>
                    <Input.Password maxLength={32}/>
                </Form.Item>

                <Form.Item {...tailLayout} name="remember" valuePropName="checked" initialValue={user.remember}>
                    <Checkbox>记住我</Checkbox>
                </Form.Item>

                <Form.Item {...tailLayout}>
                    <Button type="primary" htmlType="submit" loading={loading} onClick={()=>onFinish()}>
                        登录
                    </Button>
                </Form.Item>
            </Form>
        </div>
    )
}

export default Login
