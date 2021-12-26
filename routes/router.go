package routes

import (
	"github.com/gin-gonic/gin"
	v1 "saya-cloud/api/v1"
	"saya-cloud/config"
	"saya-cloud/middleware"
)

func InitRouter() {
	gin.SetMode(config.AppMode)
	r := gin.New()
	r.Use(middleware.Log())
	r.Use(gin.Recovery())
	r.Use(middleware.Cors())

	/*
		后台管理路由接口
	*/
	auth := r.Group("api/v1")
	auth.Use(middleware.JwtToken())
	{
		auth.GET("admin/article", v1.GetArt)
		//auth.POST("article/add", v1.AddArticle)
		//auth.PUT("article/:id", v1.EditArt)
		//auth.DELETE("article/:id", v1.DeleteArt)
	}

	/*
		前端展示页面接口
	*/
	router := r.Group("api/v1")
	{
		// 登录控制模块
		router.POST("login", v1.Login)
		//router.GET("article", v1.GetArt)
	}

	_ = r.Run(config.HttpPort)

}
