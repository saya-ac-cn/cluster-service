package record

// iot 用法见：https://www.cnblogs.com/nulige/p/10199722.html

// 操作类型
const (
	// 系统相关的0X1----
	USER_LOGIN  = "0X10001"
	USER_LOGOUT = "0X10002"
	// 用户相关的0X2----
	USER_SET_PASSWORD = "0X20001"
	// 产品相关的0X2----
	CREATE_PRODUCT = "0X30001"
	UPDATE_PRODUCT = "0X30002"
	DELETE_PRODUCT = "0X30003"
)
