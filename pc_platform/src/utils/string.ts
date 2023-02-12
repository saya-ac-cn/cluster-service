// 字符串工具类

/**
 * 清除输入框的空格
 * @returns {*}
 * @param value
 */
export const clearTrimValueEvent = (value:string) => {
    return value.replace(/\s+/g, '');
}