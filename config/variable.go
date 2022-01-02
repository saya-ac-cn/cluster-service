package config

import (
	"fmt"
	"gopkg.in/ini.v1"
	"os"
)

var (
	AppMode  string
	HttpPort string
	JwtKey   string
	LogLevel string

	AmapUrl string
	AmapKey string

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
 * 加载配置文件到内存中
 */
func loadVariable() {
	file, err := ini.Load("config/config.ini")
	if err != nil {
		fmt.Println("配置文件读取错误，请检查文件路径:", err)
		os.Exit(-1)
	}
	loadServer(file.Section("server"))
	loadAmap(file.Section("amap"))
	loadDataBase(file.Section("database"))
	loadPrimaryDB(file.Section("primary_database"))
}

/**
 * 加载高德相关的配置
 */
func loadServer(section *ini.Section) {
	AppMode = section.Key("AppMode").MustString("debug")
	HttpPort = section.Key("HttpPort").MustString(":3000")
	JwtKey = section.Key("JwtKey").MustString("89js82js72")
	LogLevel = section.Key("LogLevel").MustString("warn")
}

/**
 * 加载应用相关的配置
 */
func loadAmap(section *ini.Section) {
	AmapUrl = section.Key("AmapUrl").MustString("https://restapi.amap.com/v5/ip")
	AmapKey = section.Key("AmapKey").MustString("f9e1683d880fca390a916581322e5f0d")
}

/**
 * 加载数据库通用配置
Debugger disconnected unexpectedly
 */
func loadDataBase(section *ini.Section) {
	ShowSql = section.Key("ShowSql").MustBool(false)
	Mappers = section.Key("Mappers").Strings(",")
}

/**
 * 加载主数据库配置
 */
func loadPrimaryDB(section *ini.Section) {
	PrimaryDatasource = section.Key("Datasource").MustString("primary")
	PrimaryDriverName = section.Key("DriverName").MustString("mysql")
	PrimaryDataSourceName = section.Key("DataSourceName").MustString("root:123456@tcp(127.0.0.1:3306)/test?charset=utf8")
	PrimaryMaxLifeTime = section.Key("MaxLifeTime").MustInt(120)
	PrimaryMaxOpenConns = section.Key("MaxOpenConns").MustInt(10)
	PrimaryMaxIdleConns = section.Key("MaxIdleConns").MustInt(5)
}
