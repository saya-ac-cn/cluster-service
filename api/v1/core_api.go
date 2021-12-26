package v1

import (
	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"
	"net/http"
	"saya-cloud/middleware"
	"saya-cloud/model"
	"saya-cloud/utils/encrypt"
	"saya-cloud/utils/response"
	"time"
)

/**
 * 核心api接口，用于系统相关的操作，比如登录和退出等
 */

// Login 后台登陆
func Login(c *gin.Context) {
	var formData model.User
	_ = c.ShouldBindJSON(&formData)
	user, code := model.GetUserByAccount(formData.User)
	if code != response.SUCCSE {
		// 系统异常
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(code))
		return
	}
	if user == nil {
		// 用户不存在
		c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(response.ERROR_USER_NOT_EXIST))
		return
	}
	checkPwdResult := encrypt.ComparePasswords(user.Password, []byte(formData.Password))
	if checkPwdResult == false {
		// 密码错误
		c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(response.ERROR_PASSWORD_WRONG))
		return
	}
	user.Password = ""
	// 开启事务示例
	tx, _ := model.PrimaryDataSource.Begin()
	defer tx.Rollback()
	//model.RecordLog(tx, c, "100002")
	model.BatchRecordLog(tx, c, "100002")
	tx.Commit()
	setToken(c, *user)
}

// token生成函数
func setToken(c *gin.Context, user model.User) {
	j := middleware.NewJWT()
	claims := middleware.MyClaims{
		Username: user.User,
		StandardClaims: jwt.StandardClaims{
			NotBefore: time.Now().Unix() - 100,
			ExpiresAt: time.Now().Unix() + 7200,
			Issuer:    "Saya-Cloud",
		},
	}
	token, err := j.CreateToken(claims)
	if err != nil {
		// 生成token失败
		c.JSON(http.StatusOK, gin.H{
			"status":  response.ERROR,
			"message": response.GetMsg(response.ERROR),
			"token":   token,
		})
	}
	c.JSON(http.StatusOK, gin.H{
		"status":  200,
		"data":    user,
		"message": response.GetMsg(response.SUCCSE),
		"token":   token,
	})
	return
}
