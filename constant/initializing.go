package constant

import (
	"fmt"
	"gopkg.in/ini.v1"
	"io/ioutil"
	"os"
)

/**
 * 整个项目环境的唯一初始方法
 */
func InitVariable() {
	// 加载配置文件到内存变量中
	loadVariable()
	// 初始化日志脚手架
	setupLogger()
}

/**
 * 加载配置文件到内存中
 */
func loadVariable() {
	file, err := ini.Load("constant/config.ini")
	if err != nil {
		fmt.Println("配置文件读取错误，请检查文件路径:", err)
		os.Exit(-1)
	}
	loadServer(file.Section("server"), ServerConfigData)
	loadAmap(file.Section("amap"), AmapConfigData)
	loadDataSource(file.Section("primary_datasource"), PrimaryDataSourceConfigData)
	loadDataSource(file.Section("second_datasource"), SecondDataSourceConfigData)
}

/**
 * 加载高德相关的配置
 */
func loadServer(section *ini.Section, serverConfigData *ServerConfig) {
	serverConfigData.Mode = section.Key("AppMode").MustString("debug")
	serverConfigData.Port = section.Key("HttpPort").MustString(":3000")
	serverConfigData.JwtKey = section.Key("JwtKey").MustString("89js82js72")
	serverConfigData.LogLevel = section.Key("LogLevel").MustString("warn")
}

/**
 * 加载高德应用相关的配置
 */
func loadAmap(section *ini.Section, amapConfigData *AmapConfig) {
	amapConfigData.Url = section.Key("AmapUrl").MustString("https://restapi.amap.com/v5/ip")
	amapConfigData.Key = section.Key("AmapKey").MustString("f9e1683d880fca390a916581322e5f0d")
}

/**
 * 加载数据库相关配置
 */
func loadDataSource(section *ini.Section, dataSourceConfigData *DataSourceConfig) {
	dataSourceConfigData.DataSource = section.Key("Datasource").MustString("primary")
	dataSourceConfigData.DriverName = section.Key("DriverName").MustString("mysql")
	dataSourceConfigData.DataSourceName = section.Key("DataSourceName").MustString("root:123456@tcp(127.0.0.1:3306)/test?charset=utf8")
	dataSourceConfigData.MaxLifeTime = section.Key("MaxLifeTime").MustInt(120)
	dataSourceConfigData.MaxOpenConns = section.Key("MaxOpenConns").MustInt(10)
	dataSourceConfigData.MaxIdleConns = section.Key("MaxIdleConns").MustInt(5)
	dataSourceConfigData.ShowSql = section.Key("ShowSql").MustBool(false)
	mapperPath := section.Key("MapperPath").MustString("./model/")
	dirList, e := ioutil.ReadDir(mapperPath)
	if e == nil {
		for _, v := range dirList {
			// 判断文件是否以.xml结尾
			if v.IsDir() {
				continue
			}
			if v.Name()[len(v.Name())-4:] == ".xml" {
				dataSourceConfigData.Mappers = append(dataSourceConfigData.Mappers, mapperPath+v.Name())
			}
		}
	} else {
		fmt.Println(fmt.Sprintf("read %s path error:%s", mapperPath, e.Error()))
		return
	}
}
