package model

import (
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"github.com/wenj91/gobatis"
	"saya-cloud/constant"
	"sync"
)

var (
	PrimaryDataSource *gobatis.DB
	//primaryDataSourceOnce sync.Once
)

func InitDataSource() {
	var primaryDataSourceOnce sync.Once
	initDataSource(primaryDataSourceOnce, constant.PrimaryDataSourceConfigData, PrimaryDataSource)
}

func initDataSource(lock sync.Once, dataSourceConfigData *constant.DataSourceConfig, datasource *gobatis.DB) {
	lock.Do(func() {
		fmt.Println("init " + dataSourceConfigData.DataSource + " instance...")
		// 初始化db
		ds1 := gobatis.NewDataSourceBuilder().
			DataSource(dataSourceConfigData.DataSource).
			DriverName(dataSourceConfigData.DriverName).
			DataSourceName(dataSourceConfigData.DataSourceName).
			MaxLifeTime(dataSourceConfigData.MaxLifeTime).
			MaxOpenConns(dataSourceConfigData.MaxOpenConns).
			MaxIdleConns(dataSourceConfigData.MaxIdleConns).
			Build()
		option := gobatis.NewDSOption().
			DS([]*gobatis.DataSource{ds1}).
			Mappers(dataSourceConfigData.Mappers).
			ShowSQL(dataSourceConfigData.ShowSql)
		gobatis.Init(option)
		// 获取数据源，参数为数据源名称，如：ds1
		datasource = gobatis.Get(dataSourceConfigData.DataSource)
	})
	//return PrimaryDataSource
}
