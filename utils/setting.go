package utils

import (
	"fmt"
	"gopkg.in/ini.v1"
	"os"
)

var (
	AppMode  string
	HttpPort string
	JwtKey   string

	ShowSql bool
	Mappers []string

	PrimaryDatasource     string
	PrimaryDriverName     string
	PrimaryDataSourceName string
	PrimaryMaxLifeTime    int
	PrimaryMaxOpenConns   int
	PrimaryMaxIdleConns   int
)

/**
 * 存放系统相关必要的配置参数
 */

// TODO 配置文件的读取最先加载
func init() {
	file, err := ini.Load("config/config.ini")
	if err != nil {
		fmt.Println("配置文件读取错误，请检查文件路径:", err)
		os.Exit(-1)
	}
	LoadServer(file.Section("server"))
	LoadDataBase(file.Section("database"))
	LoadPrimaryDB(file.Section("primary_database"))
}

// LoadServer 应用相关的配置
func LoadServer(section *ini.Section) {
	AppMode = section.Key("AppMode").MustString("debug")
	HttpPort = section.Key("HttpPort").MustString(":3000")
	JwtKey = section.Key("JwtKey").MustString("89js82js72")
}

// LoadDataBase 加载数据库通用配置
func LoadDataBase(section *ini.Section)  {
	ShowSql = section.Key("showSql").MustBool(false)
	Mappers = section.Key("mappers").Strings(",")
}

func LoadPrimaryDB(section *ini.Section) {
	PrimaryDatasource = section.Key("datasource").MustString("primary")
	PrimaryDriverName = section.Key("driverName").MustString("mysql")
	PrimaryDataSourceName = section.Key("dataSourceName").MustString("root:123456@tcp(127.0.0.1:3306)/test?charset=utf8")
	PrimaryMaxLifeTime = section.Key("maxLifeTime").MustInt(120)
	PrimaryMaxOpenConns = section.Key("maxOpenConns").MustInt(10)
	PrimaryMaxIdleConns = section.Key("maxIdleConns").MustInt(5)
}