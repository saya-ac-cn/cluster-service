package main

import (
	"saya-cloud/constant"
	"saya-cloud/handle"
	"saya-cloud/model"
	"saya-cloud/routes"
)

/**
 * 项目启动入口方法！！！
 * 整个项目的环境配置相关移步initializing.init()方法
 */

// TODO init方法全局优先，最先加载
func init() {
	// 加载配置文件到内存变量中
	constant.InitVariable()
}

func main() {
	// 初始化数据库
	model.InitDataSource()
	// 初始化mqtt
	handle.InitMqttClient()
	// 引入路由组件
	routes.InitRouter()
}
