package config

/**
 * 整个项目环境的唯一初始方法
 */

// TODO init方法全局优先，最先加载
func init() {
	// 加载配置文件到内存变量中
	loadVariable()
	// 初始化日志脚手架
	setupLogger()
}
