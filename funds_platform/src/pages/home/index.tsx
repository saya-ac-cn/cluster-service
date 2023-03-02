import React from "react";
import { Descriptions,Breadcrumb,Button, DatePicker, Form, Input ,Radio,Space } from 'antd';
import { CheckOutlined,SearchOutlined } from '@ant-design/icons';

import './index.less'

const { RangePicker } = DatePicker;
const Home = () => {

    const onFinish = (values: any) => {
        console.log('Success:', values);
    };

    const onFinishFailed = (errorInfo: any) => {
        console.log('Failed:', errorInfo);
    };

    return (
        <div className="funds-div">
            <div className="funds-top">
                <span className="page-name">复盘推演</span>
                <Form className="search-form" layout='inline' onFinish={onFinish} onFinishFailed={onFinishFailed} autoComplete="off">
                    <Form.Item label="基金代码" name="fundCode" rules={[
                        { required: true, message: '基金代码不能为空!' },
                        { max: 10, message: '基金代码格式有误!' },
                        {
                            pattern: new RegExp(/^[0-9a-zA-Z]{6,}$/, "g") , /* 以数字、大小写字母开头，至少有6位*/
                            message: '基金代码格式有误'
                        }
                    ]}>
                        <Input maxLength={10}/>
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit">
                            <SearchOutlined />查询
                        </Button>
                    </Form.Item>
                </Form>
            </div>

            <div className="funds-bottom">
                <div className="funds-data">
                    <Descriptions title="基金详情" className='search-fund'>
                        <Descriptions.Item label="基金名称">1810000000</Descriptions.Item>
                        <Descriptions.Item label="净值日期">Hangzhou, Zhejiang</Descriptions.Item>
                        <Descriptions.Item label="当日净值">empty</Descriptions.Item>
                        <Descriptions.Item label="估算净值">No. 18</Descriptions.Item>
                        <Descriptions.Item label="估算涨跌">No. 18</Descriptions.Item>
                        <Descriptions.Item label="估值时间">No. 18</Descriptions.Item>
                    </Descriptions>
                    <div className="funds-container">
                        <div className="fund-setting">
                            <Form name="fund-form" labelCol={{ span: 5 }}  wrapperCol={{ span: 16 }}>
                                <Form.Item name="radio-button" label="涨跌类型" rules={[{ required: true, message: '基金代码不能为空!' }]}>
                                    <Radio.Group>
                                        <Radio.Button value="a">按百分比</Radio.Button>
                                        <Radio.Button value="b">按差价</Radio.Button>
                                    </Radio.Group>
                                    <Space direction="vertical" size="middle" className='fund-trade'>
                                        <div>每上涨<Input placeholder="1" className='fund-shares'/>%，买入<Input placeholder="1000" className='fund-shares'/>份</div>
                                        <div>每下跌<Input placeholder="1" className='fund-shares'/>%，卖出<Input placeholder="1000" className='fund-shares'/>份</div>
                                    </Space>
                                </Form.Item>
                                <Form.Item name="range-picker" label="时间范围" rules={[{ type: 'array' as const, required: true, message: '请选择时间范围!' }]}>
                                    <RangePicker />
                                </Form.Item>
                                <Form.Item wrapperCol={{ offset: 2}}>
                                    <Button type="primary" htmlType="submit">
                                        <CheckOutlined />计算
                                    </Button>
                                </Form.Item>
                            </Form>
                        </div>
                        <div className="fund-chat">
                            盈亏分析
                        </div>
                    </div>
                </div>
                <div className="tell-you">
                    <div className="notice">
                        <span className="warring">提示:</span>
                        <div>1、相关基金数据均来自<span className="warring-color">天天基金网</span>；</div>
                        <div>2、在当前版本的交易过程中，<span className="warring-color">没有将交易费用纳入到实际的盈亏</span>，在实际交易中，请酌情考虑；</div>
                        <div>3、相关盈亏计算仅为参考，<span className="warring-color">不作为最终的收益结果</span>；</div>
                        <div>4、<span className="warring-color">市场有风险，交易需谨慎</span>；</div>
                    </div>
                    <div className="help">
                        需要帮助？或有疑问？请发送Email到saya@saya.ac.cn(亲亲里实验室)寻求解决
                    </div>
                </div>
            </div>
        </div>
    )
}

export default React.memo(Home)
