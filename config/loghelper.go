package config

import (
	"fmt"
	retalog "github.com/lestrrat-go/file-rotatelogs"
	"github.com/rifflock/lfshook"
	"github.com/sirupsen/logrus"
	"os"
	"path"
	"time"
)

var Logger *logrus.Logger

/**
 * 日志脚手架初始化（警告：本方法只能被初始化一次）
 */
func setupLogger() {
	logFilePath := "logs"
	logFileName := "system.log"
	// 日志文件
	fileName := path.Join(logFilePath, logFileName)
	// 写入文件
	src, err := os.OpenFile(fileName, os.O_APPEND|os.O_WRONLY, os.ModeAppend)
	if err != nil {
		fmt.Println("读取配置文件失败：", err)
	}
	// 实例化
	logger := logrus.New()
	// 设置输出
	logger.Out = src
	// 设置日志级别
	level, err := logrus.ParseLevel(LogLevel)
	if err != nil {
		fmt.Println("无效的日志级别参数：", err)
		level = logrus.WarnLevel
	}
	logger.SetLevel(level)

	// 设置 rotatelogs
	logWriter, err := retalog.New(
		// 分割后的文件名称
		fileName+".%Y%m%d.log",
		// 生成软链，指向最新日志文件
		retalog.WithLinkName(fileName),
		// 设置最大保存时间(7天)
		retalog.WithMaxAge(7*24*time.Hour),
		// 设置日志切割时间间隔(1天)
		retalog.WithRotationTime(24*time.Hour),
	)

	writeMap := lfshook.WriterMap{
		logrus.InfoLevel:  logWriter,
		logrus.FatalLevel: logWriter,
		logrus.DebugLevel: logWriter,
		logrus.WarnLevel:  logWriter,
		logrus.ErrorLevel: logWriter,
		logrus.PanicLevel: logWriter,
	}

	lfHook := lfshook.NewHook(writeMap, &logrus.JSONFormatter{
		TimestampFormat: "2006-01-02 15:04:05",
	})

	// 新增 Hook
	logger.AddHook(lfHook)
	Logger = logger
}
