package main

import (
	"github.com/sirupsen/logrus"
	"notes-cloud/model"
	"notes-cloud/routes"
	"notes-cloud/utils"
)


// 全局最先加载的方法utils.init
func main() {
	// 初始化数据库
	model.InitPrimaryDataSource()
	// 初始化日志脚手架
	utils.SetupLogger()
	(utils.Logger).WithFields(logrus.Fields{
		"animal": "walrus",
	}).Error("服务正在启动")
	// 引入路由组件
	routes.InitRouter()
}