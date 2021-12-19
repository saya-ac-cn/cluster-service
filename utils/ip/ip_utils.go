package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
)

/**
 * 高德ip城市定位工具类
 * http接口调用：https://blog.csdn.net/jxwBlog/article/details/111190517
 */
type IpCityInfo struct {
	Status   string `json:"status"`
	Info     string `json:"info"`
	Infocode string `json:"infocode"`
	Country  string `json:"country"`
	Province string `json:"province"`
	City     string `json:"city"`
	District string `json:"district"`
	Isp      string `json:"isp"`
	Location string `json:"location"`
	Ip       string `json:"ip"`
}

func main() {
	var url bytes.Buffer
	url.WriteString("https://restapi.amap.com/v5/ip")
	url.WriteString("?key=")
	url.WriteString("f9e1683d880fca390a916581322e5f0d")
	url.WriteString("&ip=")
	url.WriteString("182.139.65.97")
	url.WriteString("&type=")
	url.WriteString("4")
	response, err := http.Get(url.String())
	if err != nil {
		return
	}
	defer response.Body.Close()
	body, _ := ioutil.ReadAll(response.Body)
	fmt.Printf(string(body))
	var result IpCityInfo
	_ = json.Unmarshal(body, &result)
	fmt.Printf("%#v", result)
}
