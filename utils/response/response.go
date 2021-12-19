package response

const (
	SUCCSE = 200
	ERROR  = 500

	// code= 1000... 用户模块的错误
	ERROR_USERNAME_USED    = 1001
	ERROR_PASSWORD_WRONG   = 1002
	ERROR_USER_NOT_EXIST   = 1003
	ERROR_TOKEN_EXIST      = 1004
	ERROR_TOKEN_RUNTIME    = 1005
	ERROR_TOKEN_WRONG      = 1006
	ERROR_TOKEN_TYPE_WRONG = 1007
	ERROR_USER_NO_RIGHT    = 1008
)

var codeMsg = map[int]string{
	SUCCSE:                 "OK",
	ERROR:                  "FAIL",
	ERROR_USERNAME_USED:    "用户名已存在！",
	ERROR_PASSWORD_WRONG:   "密码错误",
	ERROR_USER_NOT_EXIST:   "用户不存在",
	ERROR_TOKEN_EXIST:      "TOKEN不存在,请重新登陆",
	ERROR_TOKEN_RUNTIME:    "TOKEN已过期,请重新登陆",
	ERROR_TOKEN_WRONG:      "TOKEN不正确,请重新登陆",
	ERROR_TOKEN_TYPE_WRONG: "TOKEN格式错误,请重新登陆",
	ERROR_USER_NO_RIGHT:    "该用户无权限",
}

func GetMsg(code int) string {
	return codeMsg[code]
}

type Response struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
	// 定义空接口以支持泛型
	Data interface{} `json:"data"`
}

func GenerateResponse(code int, msg string, data interface{}) *Response {
	return &Response{Code: code, Message: msg, Data: data}
}

func GenerateSuccessResponse(data interface{}) *Response {
	return &Response{Code: SUCCSE, Message: codeMsg[SUCCSE], Data: data}
}

func GenerateSuccessEmptyResponse() *Response {
	return &Response{Code: SUCCSE, Message: codeMsg[SUCCSE]}
}

func GenerateErrorResponse(code int, msg string) *Response {
	return &Response{Code: code, Message: msg}
}

func GenerateErrorResponseByCode(code int) *Response {
	return &Response{Code: code, Message: codeMsg[code]}
}
