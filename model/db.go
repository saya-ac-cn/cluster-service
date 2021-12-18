package model

import (
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"github.com/wenj91/gobatis"
	"notes-cloud/utils"
	"sync"
)

var (
	PrimaryDataSource *gobatis.DB
	primaryDataSourceOnce     sync.Once
)

func InitPrimaryDataSource() *gobatis.DB {
	primaryDataSourceOnce.Do(func() {
		fmt.Println("init primaryDataSource instance...")
		// 初始化db
		ds1 := gobatis.NewDataSourceBuilder().
			DataSource(utils.PrimaryDatasource).
			DriverName(utils.PrimaryDriverName).
			DataSourceName(utils.PrimaryDataSourceName).
			MaxLifeTime(utils.PrimaryMaxLifeTime).
			MaxOpenConns(utils.PrimaryMaxOpenConns).
			MaxIdleConns(utils.PrimaryMaxIdleConns).
			Build()

		option := gobatis.NewDSOption().
			DS([]*gobatis.DataSource{ds1}).
			Mappers(utils.Mappers).
			ShowSQL(utils.ShowSql)
		gobatis.Init(option)
		// 获取数据源，参数为数据源名称，如：ds1
		PrimaryDataSource = gobatis.Get(utils.PrimaryDatasource)
	})
	return PrimaryDataSource
}