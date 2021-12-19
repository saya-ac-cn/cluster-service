package v1

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"notes-cloud/model"
	"notes-cloud/utils/response"
)

// GetArt 查询文章列表
func GetArt(c *gin.Context) {
	data, code := model.GetArt()
	c.JSON(http.StatusOK, gin.H{
		"status":  code,
		"data":    data,
		"message": response.GetMsg(code),
	})
}
