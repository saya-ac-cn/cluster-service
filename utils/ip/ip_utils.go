package ip

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net"
	"net/http"
	"saya-cloud/constant"
	"strconv"
	"strings"
)

/**
 * 高德ip城市定位工具类
 * http接口调用：https://blog.csdn.net/jxwBlog/article/details/111190517
 */

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

/**
 * 检查ip格式
 */
func checkIp(ipStr string) bool {
	address := net.ParseIP(ipStr)
	if address == nil {
		fmt.Println("ip地址格式不正确")
		return false
	} else {
		return true
	}
}

/**
 * ip to int64
 */
func inetAton(ipStr string) int64 {
	bits := strings.Split(ipStr, ".")

	b0, _ := strconv.Atoi(bits[0])
	b1, _ := strconv.Atoi(bits[1])
	b2, _ := strconv.Atoi(bits[2])
	b3, _ := strconv.Atoi(bits[3])

	var sum int64

	sum += int64(b0) << 24
	sum += int64(b1) << 16
	sum += int64(b2) << 8
	sum += int64(b3)

	return sum
}

/**
 * int64 to IP
 */
func inetNtoa(ipnr int64) net.IP {
	var bytes [4]byte
	bytes[0] = byte(ipnr & 0xFF)
	bytes[1] = byte((ipnr >> 8) & 0xFF)
	bytes[2] = byte((ipnr >> 16) & 0xFF)
	bytes[3] = byte((ipnr >> 24) & 0xFF)
	return net.IPv4(bytes[3], bytes[2], bytes[1], bytes[0])
}

/**
 * 检查ip是否为内网
 */
func isInnerIp(ipStr string) bool {
	if !checkIp(ipStr) {
		// ip格式不对，强制判定为内网
		return true
	}
	inputIpNum := inetAton(ipStr)
	innerIpA := inetAton("10.255.255.255")
	innerIpB := inetAton("172.16.255.255")
	innerIpC := inetAton("192.168.255.255")
	innerIpD := inetAton("100.64.255.255")
	innerIpF := inetAton("127.255.255.255")

	return inputIpNum>>24 == innerIpA>>24 || inputIpNum>>20 == innerIpB>>20 ||
		inputIpNum>>16 == innerIpC>>16 || inputIpNum>>22 == innerIpD>>22 ||
		inputIpNum>>24 == innerIpF>>24
}

func GetLocation(ip string) *IpCityInfo {
	var result = IpCityInfo{Ip: ip, Province: "未知", City: "未知", Location: "未知"}
	if isInnerIp(ip) {
		return &result
	}
	var url bytes.Buffer
	url.WriteString(constant.AmapConfigData.Url)
	url.WriteString("?key=")
	url.WriteString(constant.AmapConfigData.Key)
	url.WriteString("&ip=")
	url.WriteString(ip)
	url.WriteString("&type=")
	url.WriteString("4")
	response, err := http.Get(url.String())
	if err != nil {
		return &result
	}
	defer response.Body.Close()
	body, _ := ioutil.ReadAll(response.Body)
	fmt.Printf(string(body))
	_ = json.Unmarshal(body, &result)
	return &result
}
