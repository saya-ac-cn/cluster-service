/**
 * 变量操作js工具类
 */
import moment from "moment";

/**
 * 判断对象是否为空
 * 入参 data
 * 返回 为空返回true
 */
export const isEmptyObject = (data) => {
    // 手写实现的判断一个对象{}是否为空对象，没有任何属性 非空返回false
    for (let item in data)
        return false;
    return true;
};

/**
 * 货币格式化
 * @param s 要格式化的数字
 * @param n 保留几位小数
 * @returns {string}
 */
export const formatMoney = (s, n=2) => {
    n = typeof(n)==="undefined"?2:n;
    n = n > 0 && n <= 20 ? n : 2;
    s = parseFloat((s + "").replace(/[^\d.-]/g, "")).toFixed(n) + "";
    let l = s.split(".")[0].split("").reverse(),
        r = s.split(".")[1];
    let t= "";
    for (let i = 0; i < l.length; i++) {
        t += l[i] + ((i + 1) % 3 === 0 && (i + 1) !== l.length ? "," : "");
    }
    return t.split("").reverse().join("") + "." + r;
};

/**
 * 深拷贝函数  接收目标target参数
 * @param target
 * @returns {Object|Array|{}}
 */
export const deepClone = (target) => {
    // 定义一个变量
    let result;
    // 如果当前需要深拷贝的是一个对象的话
    if (typeof target === 'object') {
        // 如果是一个数组的话
        if (Array.isArray(target)) {
            result = []; // 将result赋值为一个数组，并且执行遍历
            for (let i in target) {
                // 递归克隆数组中的每一项
                result.push(deepClone(target[i]))
            }
            // 判断如果当前的值是null的话；直接赋值为null
        } else if(target===null) {
            result = null;
            // 判断如果当前的值是一个RegExp对象的话，直接赋值
        } else if(target.constructor===RegExp){
            result = target;
        }else {
            // 否则是普通对象，直接for in循环，递归赋值对象的所有值
            result = {};
            for (let i in target) {
                result[i] = deepClone(target[i]);
            }
        }
        // 如果不是对象的话，就是基本数据类型，那么直接赋值
    } else {
        result = target;
    }
    // 返回最终结果
    return result;
};

// 只能选择今天以前的日期
export const disabledDate = (current) => {
    // Can not select days before today and today
    return current && current > moment().endOf('day');
};

// 只能选择本月及其以前的月份
export const disabledMonth = (current) => {
    return current && current > moment().endOf('month');
};

// 通过组织提取用户
export const extractUserName = (organize,account) => {
    if (null === account || '' === account){
        return null;
    }
    if (isEmptyObject(organize)){
        return account;
    }
    return organize[account]
}

// 返回默认值
export const returnDefaultValue = (value) => {
    if (null === value || '' === value){
        return '未设置';
    }else {
        return value;
    }
};

//说明：javascript的加法结果会有误差，在两个浮点数相加的时候会比较明显。这个函数返回较为精确的加法结果。
//调用：accAdd(arg1,arg2)
//返回值：arg1加上arg2的精确结果
export const accAdd = (arg1,arg2) => {
    let r1,r2,m;
    try{r1=arg1.toString().split(".")[1].length}catch(e){r1=0}
    try{r2=arg2.toString().split(".")[1].length}catch(e){r2=0}
    m=Math.pow(10,Math.max(r1,r2))
    return (arg1*m+arg2*m)/m
}