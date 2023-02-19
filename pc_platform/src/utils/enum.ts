// 枚举工具类
/**
 * 转义计划提醒执行频率
 * @param key
 * @returns {*}
 */
export const getPlanHowOftenExecute = (key:number) => {
    const array = ['一次性','天','周','月','年'];
    if (key < 1 || key > array.length){
        return ''
    }
    return array[key-1];
};