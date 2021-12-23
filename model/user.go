package model

import (
	"github.com/sirupsen/logrus"
	_ "github.com/wenj91/gobatis"
	_ "log"
	"saya-cloud/utils/response"
)

type User struct {
	// 用户名
	User string `field:"user" json:"user"`
	// 密码
	Password string `field:"password" json:"password"`
	// 性别
	Sex int `field:"sex" json:"sex"`
	// qq号
	Qq string `field:"qq" json:"qq"`
	// 邮箱
	Email string `field:"email" json:"email"`
	// 电话号码
	Phone int `field:"phone" json:"phone"`
	// 生日
	Birthday string `field:"birthday" json:"birthday"`
	// 故乡
	Hometown string `field:"hometown" json:"hometown"`
	// 签名
	Autograph int `field:"autograph" json:"autograph"`
	// 头像地址
	Logo string `field:"logo" json:"logo"`
	// 设置的背景（外键）
	Background int `field:"background" json:"background"`
	// 修改时间
	UpdateTime int `field:"updateTime" json:"updateTime"`
}

// GetUserByAccount 后台登录验证
func GetUserByAccount(account string) (*User, int) {
	var user *User
	// 根据传入实体查询对象
	param := User{User: account}
	err := PrimaryDataSource.Select("UserMapper.queryOneByUser", param)(&user)
	if err != nil {
		logrus.Warn("查询用户异常:", err)
		return nil, response.ERROR
	}
	return user, response.SUCCSE
}
