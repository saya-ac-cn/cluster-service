package v1

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"saya-cloud/model"
	"saya-cloud/utils/record"
	"saya-cloud/utils/response"
	"strconv"
)

/**
 * 产品api接口，用于产品的CRUD操作
 */

func GetProductList(c *gin.Context) {
	// 查询启用的产品
	param := model.Product{Status: 1}
	products, code := model.QueryProduct(param)
	if code != response.SUCCSE {
		// 系统异常
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(code))
		return
	}
	c.JSON(http.StatusOK, gin.H{
		"status":  code,
		"data":    products,
		"message": response.GetMsg(code),
	})
}

func AddProduct(c *gin.Context) {
	var formData model.Product
	_ = c.ShouldBindJSON(&formData)
	if "" == formData.Name {
		// 缺少参数
		c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(response.NOT_PARAMETER))
		return
	}
	// 校验名字是否存在
	param := model.Product{Status: 1, Name: formData.Name}
	products, code := model.QueryProduct(param)
	if code != response.SUCCSE {
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(code))
		return
	}
	if len(products) > 0 {
		c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(response.DATA_REPEAT))
		return
	}
	// 添加时默认启用
	formData.Status = 1
	// 执行添加
	tx, _ := model.PrimaryDataSource.Begin()
	defer tx.Rollback()
	result := model.CreateProduct(tx, formData)
	model.RecordLog(tx, c, record.CREATE_PRODUCT)
	tx.Commit()
	if response.SUCCSE != result {
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(result))
		return
	}
	c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(result))
}

func EditProduct(c *gin.Context) {
	var formData model.Product
	_ = c.ShouldBindJSON(&formData)
	if "" == formData.Name {
		// 缺少参数
		c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(response.NOT_PARAMETER))
		return
	}
	// 校验名字是否存在
	param := model.Product{Status: 1, Name: formData.Name}
	products, code := model.QueryProduct(param)
	if code != response.SUCCSE {
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(code))
		return
	}
	if len(products) > 0 && products[0].Id != formData.Id {
		c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(response.DATA_REPEAT))
		return
	}
	// 执行修改
	tx, _ := model.PrimaryDataSource.Begin()
	defer tx.Rollback()
	result := model.UpdateProduct(tx, formData)
	model.RecordLog(tx, c, record.UPDATE_PRODUCT)
	tx.Commit()
	if response.SUCCSE != result {
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(result))
		return
	}
	c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(result))
}

func RemoveProduct(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, response.GenerateErrorResponseByCode(response.ERROR))
		return
	}
	param := model.Product{Status: 2, Id: id}
	// 执行逻辑删除
	tx, _ := model.PrimaryDataSource.Begin()
	defer tx.Rollback()
	result := model.UpdateProduct(tx, param)
	model.RecordLog(tx, c, record.DELETE_PRODUCT)
	tx.Commit()
	if response.SUCCSE != result {
		c.JSON(http.StatusInternalServerError, response.GenerateErrorResponseByCode(result))
		return
	}
	c.JSON(http.StatusOK, response.GenerateErrorResponseByCode(result))
}
