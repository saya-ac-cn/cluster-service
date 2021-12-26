package handle

import (
	"fmt"
	"github.com/eclipse/paho.mqtt.golang"
	"time"
)

// https://blog.csdn.net/yunyin_link/article/details/114408967
/**
 * 全局默认的订阅回调函数，收到消息后会执行
 * 这个写法和下面的等同
 */
var defaultReceiverHandle mqtt.MessageHandler = func(client mqtt.Client, message mqtt.Message) {
	fmt.Printf("Topic:%s\n", message.Topic())
	fmt.Printf("Message:%s\n", message.Payload())
}

/**
 * 订阅回调函数，收到消息后会执行
 */
func receiverHandle(client mqtt.Client, message mqtt.Message) {
	fmt.Printf("Topic:%s\n", message.Topic())
	fmt.Printf("Message:%s\n", message.Payload())
}

func InitMqttClient() {
	options := mqtt.NewClientOptions()
	options.AddBroker("tcp://1.15.81.148:1883")
	options.SetClientID("esp32-005")
	// 设置鉴权信息
	options.SetUsername("iC3466YGUm4KZbZb")
	options.SetPassword("123456")
	options.SetKeepAlive(5 * time.Second)
	options.SetPingTimeout(5 * time.Second)
	options.SetCleanSession(false)
	// 设置默认的订阅回调函数
	//options.SetDefaultPublishHandler(defaultReceiverHandle)
	client := mqtt.NewClient(options)
	// 建立连接
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}
	// 订阅主题
	if token := client.Subscribe("/iot/go/ack/#", 2, receiverHandle); token.Wait() && token.Error() != nil {
		fmt.Println(token.Error())
	}
	// 发布消息
	// if token := client.Publish("/iot/ack/esp32-01",2,false,"来自go-client的会话消息");token.Wait() && token.Error() != nil {
	// 	fmt.Println(token.Error())
	// }
}
