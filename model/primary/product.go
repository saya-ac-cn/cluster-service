package primary

import (
	"github.com/sirupsen/logrus"
	"github.com/wenj91/gobatis"
	"saya-cloud/model"
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

func QueryProduct(param Product) ([]*Product, int) {
	products := make([]*Product, 0)
	err := model.PrimaryDataSource.Select("ProductMapper.queryProduct", param)(&products)
	if err != nil {
		logrus.Warn("查询产品异常:", err)
		return nil, response.ERROR
	}
	return products, response.SUCCSE
}

// 添加产品信息
func CreateProduct(tx *gobatis.TX, product Product) int {
	_, err := tx.Update("ProductMapper.insertProduct", &product)
	if err != nil {
		logrus.Warn("添加产品异常:", err)
		return response.ERROR
	}
	return response.SUCCSE
}

// 修改产品信息
func UpdateProduct(tx *gobatis.TX, product Product) int {
	_, err := tx.Update("ProductMapper.updateProduct", &product)
	if err != nil {
		logrus.Warn("修改产品异常:", err)
		return response.ERROR
	}
	return response.SUCCSE
}
