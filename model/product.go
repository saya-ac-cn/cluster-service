package model

import (
	"github.com/sirupsen/logrus"
	"saya-cloud/utils/response"
)

/**
 * 产品
 */

type Product struct {
	Id     int    `field:"id" json:"id"`
	Name   string `field:"name" json:"name"`
	Status int    `field:"status" json:"status"`
}

func QueryProduct()(*Product,int)  {
	var product *Product
	// 查询启用的产品
	param := Product{Status: 1}
	err := PrimaryDataSource.Select("UserMapper.queryOneByUser", param)(&product)
	if err != nil {
		logrus.Warn("查询产品异常:", err)
		return nil, response.ERROR
	}
	return product, response.SUCCSE
}