package model

import (
	"github.com/sirupsen/logrus"
	"notes-cloud/utils/response"
)

type Article struct {
	Id           int64  `field:"id" json:"id"`
	CreatedAt    int    `field:"created_at" json:"createdAt"`
	UpdatedAt    string `field:"updated_at" json:"updatedAt"`
	DeletedAt    string `field:"deleted_at" json:"deletedAt"`
	Title        string `field:"title" json:"title"`
	Cid          int    `field:"cid" json:"cid"`
	Desc         string `field:"desc" json:"desc"`
	Content      string `field:"content" json:"content"`
	Img          string `field:"img" json:"img"`
	CommentCount int    `field:"comment_count" json:"commentCount"`
	ReadCount    int    `field:"read_count" json:"readCount"`
}

// GetArt 查询文章列表
func GetArt() ([]*Article, int) {
	articleList := make([]*Article, 0)
	// ArticleMapper xml中的namespace
	err := PrimaryDataSource.Select("ArticleMapper.queryAllLog", map[string]interface{}{})(&articleList)
	if err != nil {
		logrus.Warn("查询文章异常:", err)
		return nil, response.ERROR
	}
	return articleList, response.SUCCSE
}
