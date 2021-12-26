package main

import (
	"saya-cloud/handle"
	"saya-cloud/model"
	"saya-cloud/routes"
)

/**
 * 项目启动入口方法！！！
 * 整个项目的环境配置相关移步initializing.init()方法
 */
func main() {
	// 初始化数据库
	model.InitPrimaryDataSource()
	// 初始化mqtt
	handle.InitMqttClient()
	// 引入路由组件
	routes.InitRouter()
}
