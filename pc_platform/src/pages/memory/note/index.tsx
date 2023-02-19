import React, {useEffect, useRef, useState} from 'react';
import {Button, Col, Row, DatePicker, Menu, Table, Form, Modal, Tag, Select, Tooltip, Input} from "antd";
import {notePageApi, deleteNoteApi, noteBookListApi, deleteNoteBookApi} from "@/http/api";
import {openNotificationWithIcon} from "@/utils/window";
import dayjs from 'dayjs';
import {
    DeleteOutlined,
    EditOutlined,
    PlusOutlined,
    MenuOutlined,
    EllipsisOutlined,
    ReloadOutlined,
    SearchOutlined,
    LineOutlined,
    EyeOutlined,
    EyeInvisibleOutlined
} from "@ant-design/icons";
import {extractUserName} from "@/utils/var"
import Storage from "@/utils/storage";
import EditNoteBook from './book';
import EditNote from './note'
import './index.less'

const {RangePicker} = DatePicker;

// 定义组件（ES6）
const Note = () => {

    const groupRef = useRef();
    const noteRef = useRef();

    const [grid,setGrid] = useState([])
    const [pagination,setPagination] = useState({page_no:1,page_size:10,data_total:0})
    const [filters,setFilters] = useState({topic:null,notebook_id:null,begin_time: null,end_time: null})
    const [loading,setLoading] = useState(false)
    const tagColor = ['magenta', 'red', 'volcano', 'orange', 'gold', 'lime', 'green', 'cyan', 'blue', 'geekblue', 'purple']
    const organize = Storage.get(Storage.ORGANIZE_KEY)
    const [group,setGroup] = useState([])
    const [currentGroup,setCurrentGroup] = useState<string>('0');

    useEffect(()=>{
        getData();
        getGroup();
    },[])

    /**
     * 初始化Table所有列的数组
     */
    const columns = [
        {
            title: '作者',
            dataIndex: 'source', // 显示数据对应的属性名
            render:(value,row) => (extractUserName(organize, row.source))
        },
        {
            title: '标题',
            dataIndex: 'topic', // 显示数据对应的属性名
        },
        {
            title: '标签',
            render: (value, row) => {
                const tags = row.label === null ? [] : (row.label).split(';')
                if (tags.length > 0) {
                    return tags.map(forTagMap)
                } else {
                    return ''
                }
            },
        },
        {
            title: '创建时间',
            dataIndex: 'create_time', // 显示数据对应的属性名
        },
        {
            title: '修改时间',
            dataIndex: 'update_time', // 显示数据对应的属性名
        },
        {
            title: '管理',
            render: (text, record) => (
                <div>
                    <Button type="primary" size="small"
                            onClick={() => handleNoteModalOpen(record.id)}
                            shape="circle" icon={<EditOutlined/>}/>
                    &nbsp;
                    <Button type="danger" size="small" shape="circle" onClick={() => handleDeleteNote(record)}
                            icon={<DeleteOutlined/>}/>
                </div>
            ),
        },
    ]

    /**
     * 生成tag标签
     * @param tag
     * @returns {JSX.Element}
     */
    const forTagMap = tag => {
        const tagElem = (
            <Tag color={tagColor[Math.floor(Math.random() * 10)]}>
                {tag}
            </Tag>
        );
        return (<span key={tag} style={{display: 'inline-block'}}>{tagElem}</span>);
    };

    /**
     * 获取笔记列表数据
     * @returns {Promise<void>}
     */
    const getData = async (_filters = filters,_pagination= pagination) => {
        let para = {
            topic: _filters.topic,
            notebook_id: _filters.notebook_id == 0 ? null : _filters.notebook_id,
            page_no: _pagination.page_no,
            page_size: _pagination.page_size,
            begin_time: _filters.begin_time,
            end_time: _filters.end_time
        };
        // 在发请求前, 显示loading
        setLoading(true);
        // 发异步ajax请求, 获取数据
        const {msg, code, data} = await notePageApi(para).catch(()=>{setLoading(false)});
        // 在请求完成后, 隐藏loading
        setLoading(false)
        if (code === 0) {
            setGrid(data.records);
            setPagination({..._pagination,data_total: data.total_row})
        } else {
            openNotificationWithIcon("error", "错误提示", msg);
        }
    };

    /**
     * 得到笔记簿下拉选择列表数据
     */
    const getGroup = async () => {
        // 发异步ajax请求, 获取数据
        const {msg, code, data} = await noteBookListApi()
        // 在请求完成后, 隐藏loading
        // 所有的笔记簿一级选项
        let notebooks = [];
        // 所有的笔记簿二级选项
        let notebook_item = [];
        // 构建一个全部的选项
        const allSelectObject = <div style={{width: "100%", display: "flex", alignItems: "center", justifyContent: "space-between"}}><span>全部</span></div>
        notebook_item.push({label: allSelectObject,key: 0})
        if (code === 0) {
            let index = 0;
            for (let key in data) {
                let item = data[key];
                const itemSelectObject = <div
                    style={{width: "100%", display: "flex", alignItems: "center", justifyContent: "space-between"}}>
                        <span>{item.name}&nbsp;&nbsp;{(1 === item.status) ? <EyeOutlined style={{color: '#389e0d'}}/> :
                            <EyeInvisibleOutlined style={{color: '#ff4d4f'}}/>}</span>
                    <Tooltip placement="right" trigger='click' color='#fff' title={<div>
                        <Button type="primary" shape="circle" onClick={() => handleModalGroupEdit(item)}
                                icon={<EditOutlined/>} size='small'/>&nbsp;
                        <Button type="primary" shape="circle" onClick={() => handleDeleteGroup(item)}
                                icon={<LineOutlined/>} size='small' danger/>
                    </div>}>
                        <EllipsisOutlined/>
                    </Tooltip>
                </div>;
                notebook_item.push({label: itemSelectObject,key: item.id})
                index = index + 1
            }
            const submenu = {
                label: <span style={{float: "right"}}><PlusOutlined title='添加笔记簿'  onClick={handleModalGroupAdd}/></span>,
                key: -1,
                icon: <MenuOutlined title='展开/收起'/>,
                children: notebook_item,
            }
            notebooks.push(submenu)
            setGroup(notebooks);
        } else {
            openNotificationWithIcon("error", "错误提示", msg);
        }
    }


    /**
     * 重置查询条件
     */
    const reloadPage = () => {
        const _filters = {begin_time: null,end_time: null,topic: null,notebook_id:filters.notebook_id}
        setFilters(_filters);
        const _pagination = {...pagination,page_no:1}
        setPagination(_pagination)
        getData(_filters,_pagination)
    };


    /**
     * 回调函数,改变页宽大小
     * @param page_size
     * @param current
     */
    const changePageSize = (page_size, current) => {
        const _pagination = {...pagination,page_no:1,page_size:page_size}
        setPagination(pagination)
        getData(filters,_pagination)
    };

    /**
     * 回调函数，页面发生跳转
     * @param current
     */
    const changePage = (current) => {
        const _pagination = {...pagination,page_no:current}
        setPagination(_pagination)
        getData(filters,_pagination)
    };

    /**
     * 日期选择发生变化
     * @param date
     * @param dateString
     */
    const onChangeDate = (date, dateString) => {
        let _filters = {...filters}
        // 为空要单独判断
        if (dateString[0] !== '' && dateString[1] !== ''){
            _filters.begin_time = dateString[0];
            _filters.end_time = dateString[1];
        }else{
            _filters.begin_time = null;
            _filters.end_time = null;
        }
        setFilters(_filters)
        const _pagination = {...pagination,page_no:1}
        setPagination(_pagination)
        getData(_filters,_pagination)
    };


    /**
     * 双向绑定用户查询主题
     * @param event
     */
    const topicInputChange = (event) => {
        const value = event.target.value;
        const _filters = {...filters,topic:value}
        setFilters(_filters)
        const _pagination = {...pagination,page_no:1}
        setPagination(_pagination)
        getData(_filters,_pagination)
    };

    /**
     * 显示创建笔记簿的弹框
     */
    const handleModalGroupAdd = (e) => {
        // 阻止默认的事件
        e.stopPropagation();
        groupRef.current.handleDisplay(null);
    };

    /**
     * 显示修改笔记簿的弹框
     * @param value
     */
    const handleModalGroupEdit = (value) => {
        groupRef.current.handleDisplay(value);
    };

    /**
     * 删除指定笔记簿
     * @param item
     */
    const handleDeleteGroup = (item) => {
        let tips = '';
        if (item.notes_count > 0) {
            tips = `“${item.name}”笔记簿下还有：${item.notes_count}条笔记，您确认删除该笔记簿及该笔记簿下的所有笔记？`
        } else {
            tips = `您确认删除“${item.name}”空笔记簿？`
        }
        Modal.confirm({
            title: '删除确认',
            content: tips,
            cancelText: '再想想',
            okText: '不要啦',
            onOk: async () => {
                // 在发请求前, 显示loading
                setLoading(true);
                const {msg, code} = await deleteNoteBookApi(item.id).catch(()=>{setLoading(false)});
                // 在请求完成后, 隐藏loading
                setLoading(false)
                if (code === 0) {
                    openNotificationWithIcon("success", "操作结果", "删除成功");
                    // 调到第一项
                    setCurrentGroup('0');
                    getGroup()
                } else {
                    openNotificationWithIcon("error", "错误提示", msg);
                }
            }
        })
    };



    /**
     * 用户所选笔记簿发生改变
     * @param e
     */
    const handleGroupChange = (e) => {
        const _filters = {begin_time: null,end_time: null,topic: null,notebook_id:e.key==='0'?null:e.key};
        setFilters(_filters);
        setCurrentGroup(e.key);
        const _pagination = {...pagination,page_no:1};
        setPagination(_pagination);
        getData(_filters,_pagination);
    };

    /**
     * 删除指定笔记
     * @param item
     */
    const handleDeleteNote = (item) => {
        Modal.confirm({
            title: '删除确认',
            content: `确认删除主题为:${item.topic}的笔记吗?`,
            onOk: async () => {
                // 在发请求前, 显示loading
                setLoading(true);
                const {msg, code} = await deleteNoteApi(item.id).catch(()=>{setLoading(false)});
                // 在请求完成后, 隐藏loading
                setLoading(false)
                if (code === 0) {
                    openNotificationWithIcon("success", "操作结果", "删除成功");
                    getData();
                } else {
                    openNotificationWithIcon("error", "错误提示", msg);
                }
            }
        })
    };

    /**
     * 显示笔记编辑的弹窗
     */
    const handleNoteModalOpen = (val:number) => {
        noteRef.current.handleDisplay(val);
    };

    return (
        <div>
            <div className='child-container'>
                <div className='header-tools'>
                    笔记簿
                </div>
                <div className='child-content bv1-note-index'>
                    <EditNoteBook ref={groupRef} refreshPage={getGroup}/>
                    <EditNote ref={noteRef} refreshPage={getData}/>
                    <Row className="note-data">
                        <Col span={5} className="tree-area">
                            <Menu mode="inline"
                                  selectedKeys={[currentGroup ? currentGroup : '0']}
                                  defaultOpenKeys={['-1']} onSelect={handleGroupChange} items={group}>
                            </Menu>
                        </Col>
                        <Col span={19} className="table-area">
                            <Row>
                                <Col span={24} className="toolbar">
                                    <Form layout="inline">
                                        <Form.Item label="主题:">
                                            <Input type='text' value={filters.topic} allowClear={true}
                                                   onChange={topicInputChange}
                                                   placeholder='按主题检索'/>
                                        </Form.Item>
                                        <Form.Item label="添加时间:">
                                            <RangePicker value={(filters.begin_time !== null && filters.end_time !== null)?[dayjs(filters.begin_time),dayjs(filters.end_time)]:[null,null]} onChange={onChangeDate}/>
                                        </Form.Item>
                                        <Form.Item>
                                            <Button type="primary" htmlType="button" onClick={getData}>
                                                <SearchOutlined/>查询
                                            </Button>
                                        </Form.Item>
                                        <Form.Item>
                                            <Button type="primary" htmlType="button" onClick={reloadPage}>
                                                <ReloadOutlined/>重置
                                            </Button>
                                        </Form.Item>
                                        <Form.Item>
                                            <Button type="primary" htmlType="button" onClick={() => handleNoteModalOpen(null)}>
                                                <PlusOutlined/>发布
                                            </Button>
                                        </Form.Item>
                                    </Form>
                                </Col>
                                <Col span={24} className="dataTable">
                                    <Table size="small" rowKey="id" bordered loading={loading} columns={columns} dataSource={grid}
                                           pagination={{
                                               current:pagination.page_no,
                                               showTotal: () => `当前第${pagination.page_no}页 共${pagination.data_total}条`,
                                               pageSize: pagination.page_size, showQuickJumper: true, total: pagination.data_total, showSizeChanger: true,
                                               onShowSizeChange: (current, page_size) => changePageSize(page_size, current),
                                               onChange: changePage,
                                           }}/>
                                </Col>
                            </Row>
                        </Col>
                    </Row>
                </div>
            </div>
        </div>
    )
}

// 对外暴露
export default Note;
