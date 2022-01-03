package model

import (
	"github.com/sirupsen/logrus"
	"github.com/wenj91/gobatis"
	_ "github.com/wenj91/gobatis"
	_ "log"
	"saya-cloud/utils/response"
)

type User struct {
	User       string `field:"user" json:"user"`
	Password   string `field:"password" json:"password"`
	Email      string `field:"email" json:"email"`
	Phone      int    `field:"phone" json:"phone"`
	UpdateTime int    `field:"update_time" json:"updateTime"`
}

// 获取用户信息
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

// 修改用户信息
func UpdateUserInfo(tx *gobatis.TX, user User) int {
	_, err := tx.Update("UserMapper.updateUserInfo", &user)
	if err != nil {
		logrus.Warn("修改用户异常:", err)
		return response.ERROR
	}
	return response.SUCCSE
}
