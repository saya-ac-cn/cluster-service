package model

/**
 * 标准物理量
 */

type StandardUnit struct {
	Id     int    `field:"id" json:"id"`
	Name   string `field:"name" json:"name"`
	Symbol string `field:"symbol" json:"symbol"`
}
