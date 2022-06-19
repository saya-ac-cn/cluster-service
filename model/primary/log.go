package primary

import (
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
	"github.com/wenj91/gobatis"
	"saya-cloud/utils/ip"
)

type Log struct {
	Id       int64  `field:"id" json:"id"`
	User     string `field:"user" json:"user"`
	Type     string `field:"type" json:"type"`
	Ip       string `field:"ip" json:"ip"`
	City     string `field:"city" json:"city"`
	Location string `field:"location" json:"location"`
	Date     string `field:"date" json:"date"`
}

/**
 * 记录日志
 */
func RecordLog(tx *gobatis.TX, c *gin.Context, code string) {
	location := ip.GetLocation(c.ClientIP())
	log := Log{
		User:     "Pandora",
		Type:     code,
		Ip:       location.Ip,
		City:     location.Location,
		Location: "-",
	}
	_, _, err := tx.Insert("LogMapper.insertLog", log)
	if err != nil {
		logrus.Warn("记录日志异常:", err)
	}
}

/**
 * 批量记录日志
 */
func BatchRecordLog(tx *gobatis.TX, c *gin.Context, code string) {
	location := ip.GetLocation(c.ClientIP())
	log1 := Log{
		User:     "Pandora",
		Type:     code,
		Ip:       location.Ip,
		City:     location.Location,
		Location: "-",
	}
	log2 := Log{
		User:     "Shmily",
		Type:     code,
		Ip:       location.Ip,
		City:     location.Location,
		Location: "-",
	}
	logs := make([]Log, 0)
	logs = append(logs, log1)
	logs = append(logs, log2)
	_, _, err := tx.Insert("LogMapper.batchInsertLog", map[string]interface{}{
		"list": logs,
	})
	if err != nil {
		logrus.Warn("记录日志异常:", err)
	}
}
