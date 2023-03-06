import React, {useEffect, useState} from "react";
import { Descriptions,Spin,Button, DatePicker, Form, Input ,Radio,notification } from 'antd';
import { CheckOutlined,SearchOutlined,FileExcelOutlined } from '@ant-design/icons';
import type { Dayjs } from 'dayjs';
import * as dayjs from 'dayjs'
import { invoke } from '@tauri-apps/api/tauri'
import './index.less'
import { Line } from '@ant-design/charts';
import { open } from '@tauri-apps/api/dialog';
import { desktopDir } from '@tauri-apps/api/path';


const { RangePicker } = DatePicker;
const Home = () => {

    const [whereForm] = Form.useForm();
    const [where, setWhere] = useState({fund_code:'',start_date:null,end_date:null,flag:true,rise:1,buy:1000,fall:1,sell:1000});
    const [calculateType,setCalculateType] = useState("%")
    const [loading, setLoading] = useState(false);
    const [fundInfo, setFundInfo] = useState({fundcode:null,name:null,jzrq:null,dwjz:null,gsz:null,gszzl:0,gztime:null,start_date:null,end_date:null});
    const [queryLoading, setQueryLoading] = useState(false);
    const [calculateLoading, setCalculateLoading] = useState(false);
    const [excelLoading, setExcelLoading] = useState(false);
    const [result, setResult] = useState([
        {
            cash_out: "0",
            cost: "2225.000",
            date: "2023-02-13",
            earning_rate: 0.00000,
            hold_share: 1000,
            hold_value: "2225.000",
            net_worth: "2.225",
            rise: "0.03396",
            rise_rate: 1.55,
            trade_share: 1000,
            trade_type: "买入"
        },
        {
            cash_out: "0",
            cost: "2225.000",
            date: "2023-02-14",
            earning_rate: -0.22472,
            hold_share: 1000,
            hold_value: "2220.00",
            net_worth: "2.22",
            rise: "-0.00489",
            rise_rate: -0.22,
            trade_share: 0,
            trade_type: "赎回"
        },
        {
            cash_out: "0",
            cost: "2225.000",
            date: "2023-02-15",
            earning_rate: -0.22472,
            hold_share: 1000,
            hold_value: "2220.00",
            net_worth: "2.22",
            rise: "0.00000",
            rise_rate: 0,
            trade_share: 0,
            trade_type: "赎回"
        },
        {
            cash_out: "0",
            cost: "2225.000",
            date: "2023-02-16",
            earning_rate: -0.71910,
            hold_share: 1000,
            hold_value: "2209.000",
            net_worth: "2.209",
            rise: "-0.01110",
            rise_rate: -0.5,
            trade_share: 0,
            trade_type: "赎回"
        },
        {
            cash_out: "0",
            cost: "2225.000",
            date: "2023-02-17",
            earning_rate: -0.98876,
            hold_share: 1000,
            hold_value: "2203.000",
            net_worth: "2.203",
            rise: "-0.00596",
            rise_rate: -0.27,
            trade_share: 0,
            trade_type: "赎回"
        },
        {
            cash_out: "0",
            cost: "6731.000",
            date: "2023-02-20",
            earning_rate: 0.41599,
            hold_share: 3000,
            hold_value: "6759.000",
            net_worth: "2.253",
            rise: "0.05001",
            rise_rate: 2.27,
            trade_share: 2000,
            trade_type: "买入"
        },
        {
            cash_out: "2222.000",
            cost: "6731.000",
            date: "2023-02-21",
            earning_rate: -0.96568,
            hold_share: 2000,
            hold_value: "4444.000",
            net_worth: "2.222",
            rise: "-0.03086",
            rise_rate: -1.37,
            trade_share: 1000,
            trade_type: "赎回"
        },
        {
            cash_out: "2222.000",
            cost: "28727.000",
            date: "2023-02-22",
            earning_rate: 2.09211,
            hold_share: 11000,
            hold_value: "27106.000",
            net_worth: "2.444",
            rise: "0.22198",
            rise_rate: 9.99,
            trade_share: 9000,
            trade_type: "买入"
        }
    ]);
    const [data, setData] = useState([]);
    const [warehouse, setWarehouse] = useState({});

    useEffect(() => {
        const data = []
        const warehouse = {};
        for (const index in result) {
           const item = result[index]
            data.push({date:item.date,value:item.rise_rate,category:'净值涨幅'})
            data.push({date:item.date,value:item.earning_rate,category:'累计收益'})
            warehouse[item.date] = item
        }
        setWarehouse(warehouse);
        setData(data)
    }, []);

    /**
     * 查询基金详情
     * @param values 验证通过的表单
     */
    const onQuery = (values: any) => {
        setQueryLoading(true)
        setLoading(true)
        invoke('query_fund_info',{fundCode:values.fundCode}).then((message) => {
            console.log('Success:', message);
            setFundInfo(message);
            setQueryLoading(false)
            setLoading(false)
        }).catch((error) => {
            setQueryLoading(false)
            setLoading(false)
            console.error('error',error)
            openNotification(error);
        })

    };

    /**
     * 计算收益（含导出）
     */
    const onCalculate = () => {
        const {fundcode,start_date,end_date } = fundInfo;
        if (null == fundcode || null === start_date || null === end_date){
            openNotification('请先查询基金详细信息~');
            return
        }
        whereForm.validateFields(['flag','buy','fall','rise','sell','date']).then((values) => {
            setCalculateLoading(true)
            setLoading(true)
            const param = {
                fund_code:fundcode,
                start_date:values.date[0].unix()*1000,
                end_date:values.date[1].unix()*1000,
                buy:values.buy,
                fall:values.fall,
                flag:values.flag,
                rise:values.rise,
                sell:values.sell
            }
            invoke('fund_calculate',{param:param}).then((message) => {
                const data = []
                const warehouse = {};
                for (const index in message) {
                    const item = message[index]
                    data.push({date:item.date,value:item.rise_rate,category:'净值涨幅'})
                    data.push({date:item.date,value:item.earning_rate,category:'累计收益'})
                    warehouse[item.date] = item
                }
                setWarehouse(warehouse);
                setData(data)

                setCalculateLoading(false)
                setLoading(false)
            }).catch((error) => {
                setCalculateLoading(false)
                setLoading(false)
                console.error('error',error)
                openNotification(error);
            })
        }).catch(e => {
            console.error("表单或者计算发生错误",e)
        });
    };

    /**
     * 导出激素哑巴结果
     */
    const onOutExcel = async () => {
        const {fundcode,start_date,end_date } = fundInfo;
        if (null == fundcode || null === start_date || null === end_date){
            openNotification('请先查询基金详细信息~');
            return
        }
        // 选择存储位置
        const save_path = await open({
            directory: true,
            multiple: false,
            defaultPath: await desktopDir(),
        });
        if (!save_path){
            openNotification('请选择保存位置~');
            return
        }

        whereForm.validateFields(['flag','buy','fall','rise','sell','date']).then((values) => {
            setCalculateLoading(true)
            setExcelLoading(true)
            const param = {
                fund_code:fundcode,
                start_date:values.date[0].unix()*1000,
                end_date:values.date[1].unix()*1000,
                buy:values.buy,
                fall:values.fall,
                flag:values.flag,
                rise:values.rise,
                sell:values.sell,
                save_path:save_path
            }
            invoke('out_excel',{param:param}).then((message) => {
                notification.success({
                    message: `执行结果`,
                    placement:'topRight',
                    description:'导出成功'
                })
                setCalculateLoading(false)
                setExcelLoading(false)
            }).catch((error) => {
                setCalculateLoading(false)
                setExcelLoading(false)
                console.error('error',error)
                openNotification(error);
            })
        }).catch(e => {
            console.error("表单或者导出发生错误",e)
        });

    }

    // 计算类型的切换
    const switchType = (e) => {
        if (e.target.value){
            setCalculateType("%")
        }else {
            setCalculateType("￥")
        }
    }

    // 不可选的日期
    const disabledDate = (current: Dayjs) => {
        const {start_date,end_date } = fundInfo;
        const now = current.unix()*1000
        if (null === start_date || null === end_date){
            const yesterday = dayjs().subtract(1, 'day').unix()*1000;
            const _now = dayjs().unix()*1000;
            return now < yesterday || now > _now
        }else{
            return now < start_date || now > end_date
        }
    };

    const COLOR_PLATE_10 = [
        '#5B8FF9',
        '#5AD8A6',
        '#5D7092',
        '#F6BD16',
        '#E8684A',
        '#6DC8EC',
        '#9270CA',
        '#FF9D4D',
        '#269A99',
        '#FF99C3',
    ];

    const config = {
        data,
        padding: 'auto',
        xField: 'date',
        yField: 'value',
        seriesField: 'category',
        xAxis: {
            tickCount: 30,
        },
        yAxis: {
            label: {
                // 数值格式化为千分位
                formatter: (v) => `${v}%`.replace(/\d{1,3}(?=(\d{3})+$)/g, (s) => `${s},`),
            },
        },
        slider: {
            start: 0.1,
            end: 0.8,
        },
        color: COLOR_PLATE_10,
        render: 'svg',
        point: {
            shape: ({ category }) => {
                return category === '累计收益' ? 'square' : 'circle';
            },
            style: ({ year }) => {
                return {
                    r: Number(year) % 4 ? 0 : 3, // 4 个数据示一个点标记
                };
            },
        },
        tooltip: {
            customContent: (title,items) => {
                const data = items[0]?.data || {};
                const date = data.date;
                const item = date?warehouse[date]:{}
                if (!item){
                    return `<div class="custom-tooltip">
                    <div class ="custom-tooltip-title">${date}</div>
                    <div class="custom-tooltip-line">
                       <div><span>净值：</span><span></span></div>
                       <div><span>净值涨幅：</span><span></span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>涨幅：</span><span></span></div>
                       <div><span>交易类型：</span><span></span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>交易份额：</span><span></span></div>
                       <div><span>持有份额：</span><span></span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>持有总市值：</span><span></span></div>
                       <div><span>已赎回：</span><span></span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>总成本价：</span><span></span></div>
                       <div><span>累计收益：</span><span></span></div>
                    </div>
                </div>`
                }else {
                 return `<div class="custom-tooltip">
                    <div class ="custom-tooltip-title">${date}</div>
                    <div class="custom-tooltip-line">
                       <div><span>净值：</span><span>${item ? item.net_worth : null}</span></div>
                       <div><span>净值涨幅：</span><span>${item ? item.rise_rate : null}%</span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>涨幅：</span><span>${item ? item.rise : null}</span></div>
                       <div><span>交易类型：</span><span>${item ? item.trade_type : null}</span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>交易份额：</span><span>${item ? item.trade_share : null}</span></div>
                       <div><span>持有份额：</span><span>${item ? item.hold_share : null}</span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>持有总市值：</span><span>${item ? item.hold_value : null}</span></div>
                       <div><span>已赎回：</span><span>${item ? item.cash_out : null}</span></div>
                    </div>
                    <div class="custom-tooltip-line">
                       <div><span>总成本价：</span><span>${item ? item.cost : null}</span></div>
                       <div><span>累计收益：</span><span>${item ? item.earning_rate : null}%</span></div>
                    </div>
                </div>`
                }
            }
        }
    };

    const openNotification = (message:string) => {
        notification.error({
            message: `错误提示`,
            placement:'topRight',
            description:message
        })
    };

    return (
        <div className="funds-div">
            <div className="funds-top">
                <span className="page-name">基金回测</span>
                <Form name="search-form" className="search-form" layout='inline' onFinish={onQuery} autoComplete="off">
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
                        <Button type="primary" htmlType="submit" loading={queryLoading}>
                            <SearchOutlined />查询
                        </Button>
                    </Form.Item>
                </Form>
            </div>

            <div className="funds-bottom">
                <div className="funds-data">
                    <Spin size="large" spinning={loading}>
                        <Descriptions title={<div className="fund-info-label">基金详情</div>} className='search-fund'>
                            <Descriptions.Item label="基金名称">{fundInfo.name}</Descriptions.Item>
                            <Descriptions.Item label="净值日期">{fundInfo.jzrq}</Descriptions.Item>
                            <Descriptions.Item label="当日净值">{fundInfo.dwjz}</Descriptions.Item>
                            <Descriptions.Item label="估算净值">{fundInfo.gsz}</Descriptions.Item>
                            <Descriptions.Item label="估算涨跌">{
                                fundInfo.gszzl?
                                    fundInfo.gszzl>=0?<span style={{color:'#cf1322'}}>{fundInfo.gszzl}%</span>:<span style={{color:'#3f8600'}}>{fundInfo.gszzl}%</span>
                                    :''
                            }</Descriptions.Item>
                            <Descriptions.Item label="估值时间">{fundInfo.gztime}</Descriptions.Item>
                        </Descriptions>
                        <div className="funds-container">
                            <div className="fund-setting">
                                <div className="fund-form-label">交易设置</div>
                                <Form name="fund-form" form={whereForm} labelCol={{ span: 5 }}  wrapperCol={{ span: 16 }}>
                                    <Form.Item name="flag" initialValue={where.flag} label="涨跌类型" rules={[{ required: true, message: '买卖参数不能为空!' }]}>
                                        <Radio.Group>
                                            <Radio.Button onChange={switchType} value={true}>%</Radio.Button>
                                            <Radio.Button onChange={switchType} value={false}>￥</Radio.Button>
                                        </Radio.Group>
                                    </Form.Item>
                                    <Form.Item label="每上涨" className="fund-setting-where-div" style={{ marginBottom: 0 }}>
                                        <Form.Item name="rise" initialValue={where.rise}  rules={[{ required: true,message: '请输入上涨幅度'}]} className="fund-setting-where">
                                            <Input placeholder="1" className='fund-shares'/>
                                        </Form.Item>
                                        <span>{calculateType}，卖出：</span>
                                        <Form.Item name="buy" initialValue={where.sell} rules={[{ required: true,message: '请输入交易份额' }]} className="fund-setting-where">
                                            <Input placeholder="1" className='fund-shares'/>
                                        </Form.Item>
                                        <span>份</span>
                                    </Form.Item>
                                    <Form.Item label="每下跌" className="fund-setting-where-div" style={{ marginBottom: 0 }}>
                                        <Form.Item name="fall" initialValue={where.fall} rules={[{ required: true,message: '请输入下跌幅度'}]} className="fund-setting-where">
                                            <Input placeholder="1" className='fund-shares'/>
                                        </Form.Item>
                                        <span>{calculateType}，买入：</span>
                                        <Form.Item name="sell" initialValue={where.buy} rules={[{ required: true,message: '请输入交易份额' }]} className="fund-setting-where">
                                            <Input placeholder="1" className='fund-shares'/>
                                        </Form.Item>
                                        <span>份</span>
                                    </Form.Item>
                                    <Form.Item name="date" label="时间范围" rules={[{ type: 'array' as const, required: true, message: '请选择时间范围!' }]}>
                                        <RangePicker disabledDate={disabledDate}/>
                                    </Form.Item>
                                    <Form.Item wrapperCol={{ offset: 2}}>
                                        <Button type="primary" onClick={onCalculate} loading={calculateLoading}>
                                            <CheckOutlined />计算
                                        </Button>
                                    </Form.Item>
                                </Form>
                            </div>
                            <div className="fund-chat">
                                <div className="fund-analyse-label">
                                    <span>收益分析</span>
                                    <Button type="primary" size="small" onClick={onOutExcel} loading={excelLoading}>
                                        <FileExcelOutlined />导出
                                    </Button>
                                </div>
                                <Line {...config} />
                            </div>
                        </div>
                    </Spin>
                </div>
                <div className="tell-you">
                    <div className="notice">
                        <span className="warring">提示:</span>
                        <div>1、相关基金数据均来自<span className="warring-color">中国建设银行，天天基金网</span>；</div>
                        <div>2、在当前版本的交易过程中，<span className="warring-color">没有将交易费用纳入到实际的盈亏</span>，在实际交易中，请酌情考虑；</div>
                        <div>3、相关盈亏计算仅为参考，<span className="warring-color">不作为最终的收益结果</span>；</div>
                        <div>4、<span className="warring-color">市场有风险，交易需谨慎</span>；</div>
                    </div>
                    <div className="help">
                        需要帮助？或有疑问？请发送Email到saya@saya.ac.cn(亲亲里实验室)寻求解决。
                    </div>
                </div>
            </div>
        </div>
    )
}

export default React.memo(Home)
