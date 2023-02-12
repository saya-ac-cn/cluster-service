/**
 * 进行local数据存储管理的工具模块
 */
import store from "store"

const LOGIN_KEY:string = 'login'
const USER_KEY:string = 'user'
const ACCESS_KEY:string = 'access_token'
const PLAN_KEY:string = 'plan'
const LOG_KEY:string = 'log'
const ORGANIZE_KEY:string = 'organize'

export default {
    USER_KEY:USER_KEY,ACCESS_KEY:ACCESS_KEY,PLAN_KEY:PLAN_KEY,LOG_KEY:LOG_KEY,ORGANIZE_KEY:ORGANIZE_KEY, LOGIN_KEY: LOGIN_KEY,

    /**
     * 保存
     * @param key 键
     * @param val 值
     */
    add(key:string,val:any) {
        store.set(key, val)
    },

    /**
     *  读取
     * @param key 键
     */
    get(key:string) {
        return store.get(key)
    },

    /**
     * 删除
     * @param key 键
     */
    remove(key:string) {
        store.remove(key)
    },

    /**
     * 删除所有
     */
    removeAll(){
        store.remove(USER_KEY)
        store.remove(ACCESS_KEY)
        store.remove(PLAN_KEY)
        store.remove(LOG_KEY)
        store.remove(ORGANIZE_KEY)
    }

}