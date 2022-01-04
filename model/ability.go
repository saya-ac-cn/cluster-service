package model

/**
 * 标准物理量
 */

type Ability struct {
	Id         int    `field:"id" json:"id"`
	ProductId  int    `field:"product_id" json:"productId"`
	Name       string `field:"name" json:"name"`
	Property   string `field:"property" json:"property"`
	StandardId int    `field:"standard_id" json:"standardId"`
	Type       int    `field:"type" json:"type"`
	Scope      string `field:"scope" json:"scope"`
	RwFlag     int    `field:"rw_flag" json:"rwFlag"`
}
