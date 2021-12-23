package model

import (
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"github.com/wenj91/gobatis"
	"saya-cloud/config"
	"sync"
)

var (
	PrimaryDataSource     *gobatis.DB
	primaryDataSourceOnce sync.Once
)

func InitPrimaryDataSource() *gobatis.DB {
	primaryDataSourceOnce.Do(func() {
		fmt.Println("init primaryDataSource instance...")
		// 初始化db
		ds1 := gobatis.NewDataSourceBuilder().
			DataSource(config.PrimaryDatasource).
			DriverName(config.PrimaryDriverName).
			DataSourceName(config.PrimaryDataSourceName).
			MaxLifeTime(config.PrimaryMaxLifeTime).
			MaxOpenConns(config.PrimaryMaxOpenConns).
			MaxIdleConns(config.PrimaryMaxIdleConns).
			Build()

		option := gobatis.NewDSOption().
			DS([]*gobatis.DataSource{ds1}).
			Mappers(config.Mappers).
			ShowSQL(config.ShowSql)
		gobatis.Init(option)
		// 获取数据源，参数为数据源名称，如：ds1
		PrimaryDataSource = gobatis.Get(config.PrimaryDatasource)
	})
	return PrimaryDataSource
}
