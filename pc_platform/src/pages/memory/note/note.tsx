import React, {forwardRef, useEffect, useImperativeHandle, useRef, useState} from 'react';
import {Button, Form, Drawer, Input, Tag, Select, Space} from "antd";
import Editor from '@/component/editor'
import {openNotificationWithIcon} from "@/utils/window";
import {noteInfoApi, updateNoteApi, createNoteApi, noteBookListApi} from "@/http/api"
import {clearTrimValueEvent} from "@/utils/string";
import {PlusOutlined} from "@ant-design/icons";
import type { InputRef } from 'antd';

const formItemLayout = {
    labelCol: {span: 2},
    wrapperCol: {span: 20},
};
const {Option} = Select;


const EditNote = (props,ref) => {

    //操作dom节点
    const editorRef = useRef<any>();
    const labelRef = useRef<InputRef>();

    const [noteForm] = Form.useForm();
    const [note,setNote] = useState({id:null,topic: null,notebook_id: null,label: [],content: null})
    const [open, setOpen] = useState<boolean>(false);
    const tagColor = ['magenta', 'red', 'volcano', 'orange', 'gold', 'lime', 'green', 'cyan', 'blue', 'geekblue', 'purple']
    const [confirmLoading, setConfirmLoading] = useState<boolean>(false);
    const [labelVisible,setLabelVisible] = useState<boolean>(false);
    const [label,setLabel] = useState<string>(null);
    const [group,setGroup] = useState([])

    // 暴露方法给父组件
    useImperativeHandle(ref,()=>({
        handleDisplay
    }))

    useEffect(() => {
        if (labelVisible) {
            labelRef.current?.focus();
        }
    }, [labelVisible]);


    /**
     * 初始化数据
     * @param val
     */
    const handleDisplay = async (val: number) => {
        getNoteBook();
        if (val) {
            // 发异步ajax请求, 获取数据
            const {msg, code, data} = await noteInfoApi(val);
            if (code === 0) {
                const label = data.label === null ? [] : (data.label).split(';')
                setNote({id:data.id,topic: data.topic,label: label,notebook_id:data.notebook_id,content: data.content})
                noteForm.setFieldsValue({topic: data.topic,notebook_id:data.notebook_id});
                editorRef.current.initEditor(data.content);
            } else {
                openNotificationWithIcon("error", "错误提示", msg);
                setNote({id:null,topic: null,notebook_id: null,label: [],content: null})
                noteForm.setFieldsValue({topic: null,notebook_id:null});
                editorRef.current.initEditor(null);
            }
        } else {
            setNote({id:null,topic: null,notebook_id: null,label: [],content: null})
            noteForm.setFieldsValue({topic: null,notebook_id:null});
            editorRef.current.initEditor(null);
        }
        setOpen(true);
    };

    const handleCancel = () => {
        setNote({id:null,topic: null,notebook_id: null,label: [],content: null})
        noteForm.setFieldsValue({label: null});
        setOpen(false);
    };

    /**
     * 删除label
     * @param removedTag
     */
    const handleLabelDelete = removedTag => {
        const label = note.label;
        const tags = label.filter(tag => tag !== removedTag);
        setNote({...note,label: tags});
    };

    /**
     * 显示文本框，让用户可输入Label
     */
    const showLabelInput = () => {
        // 至多允许用户输入5个tag
        if (note.label.length < 5){
            setLabelVisible(true);
        }
    };

    const handleLabelInputChange = e => {
        setLabel(e.target.value);
    };

    /**
     * 添加tag
     */
    const handleLabelInputConfirm = () => {
        let _label = note.label;
        if (label && _label.indexOf(label) === -1) {
            _label = [..._label, label];
        }
        setLabelVisible(false);
        setLabel(null);
        setNote({...note,label: _label})
    };


    const forMap = tag => {
        const tagElem = (
            <Tag
                closable
                color={tagColor[Math.floor(Math.random()*10)]}
                onClose={e => {
                    e.preventDefault();
                    handleLabelDelete(tag);
                }}
            >
                {tag}
            </Tag>
        );
        return (<span key={tag} style={{ display: 'inline-block' }}>{tagElem}</span>
        );
    };


    /**
     * 响应用户提交事件
     */
    const handleSubmit = () => {
        let content = editorRef.current.getContent();
        if (null == content || content === ''){
            openNotificationWithIcon("error", "错误提示", "请填写您要发布的内容");
            return
        }
        noteForm.validateFields(['topic', 'notebook_id']).then(values => {
            let label = null;
            if (note.label.length > 1){
                label = note.label.join(';')
            } else if(note.label.length === 1){
                label = note.label[0]
            } else {
                label = null
            }
            let param = {topic: values.topic,label: label,content: content,notebook_id: values.notebook_id};
            if (note.id){
                // 执行修改
                param.id = note.id;
                updateNotes(param)
            } else{
                // 执行添加
                createNotes(param)
            }
        }).catch(e => console.log("修改或添加笔记错误", e));
    };

    /**
     * 更新动态
     * @param param
     * @returns {Promise<void>}
     */
    const updateNotes = async (param) => {
        setConfirmLoading(true);
        const {msg, code} = await updateNoteApi(param).catch(()=>setConfirmLoading(false));
        setConfirmLoading(false);
        if (code === 0) {
            openNotificationWithIcon("success", "操作结果", "笔记修改成功");
            handleCancel();
            // 调用父页面的刷新数据方法
            props.refreshPage();
        } else {
            openNotificationWithIcon("error", "错误提示", msg);
        }
    };

    /**
     * 创建动态
     * @param param
     * @returns {Promise<void>}
     */
    const createNotes = async (param) => {
        setConfirmLoading(true);
        const {msg, code} = await createNoteApi(param).catch(()=>setConfirmLoading(false));
        setConfirmLoading(false);
        if (code === 0) {
            openNotificationWithIcon("success", "操作结果", "笔记保存成功");
            handleCancel();
            // 调用父页面的刷新数据方法
            props.refreshPage();
        } else {
            openNotificationWithIcon("error", "错误提示", msg);
        }
    };


    /**
     * 得到笔记簿下拉选择列表数据
     */
    const getNoteBook = async () => {
        // 发异步ajax请求, 获取数据
        const {msg, code, data} = await noteBookListApi()
        if (code === 0) {
            let notebooks = [];
            data.forEach(item => {
                notebooks.push((<Option key={item.id} value={item.id}>{item.name}</Option>));
            });
            setGroup(notebooks);
        } else {
            openNotificationWithIcon("error", "错误提示", msg);
        }
    }

    return (
        <Drawer title={note && note.id ? '编辑笔记':'发布笔记'} width='75%' forceRender onClose={handleCancel} open={open} bodyStyle={{ paddingBottom: 80 }} maskClosable={false}
                footer={
                    <Space>
                        <Button onClick={handleCancel}>取消</Button>
                        <Button loading={confirmLoading} onClick={handleSubmit} type="primary">
                            保存
                        </Button>
                    </Space>
                }
        >
            <Form {...formItemLayout} form={noteForm} className="bk-transparent">
                <Form.Item label="标题" {...formItemLayout} name='topic'
                           getValueFromEvent={(e) => clearTrimValueEvent(e.target.value)}
                           rules={[{required: true, message: '请输入标题'}, {
                               max: 50,
                               message: '最多不超过50个字符'
                           },]}>
                    <Input type='text' style={{width: '40em'}}/>
                </Form.Item>
                <Form.Item label="所属分类" {...formItemLayout} name='notebook_id'
                           rules={[{required: true, message: '请选择所属分类'}]}>
                    <Select style={{width: '24em'}} showSearch
                            placeholder="请选择所属分类">
                        {group}
                    </Select>
                </Form.Item>
                <Form.Item label="标签" {...formItemLayout}>
                    <div style={{display: 'inline'}}>
                        {note.label.map(forMap)}
                    </div>
                    {labelVisible && (
                        <Input
                            ref={labelRef}
                            type="text"
                            size="small"
                            style={{width: 78}}
                            value={label}
                            onChange={handleLabelInputChange}
                            onBlur={handleLabelInputConfirm}
                            onPressEnter={handleLabelInputConfirm}
                        />
                    )}
                    {!labelVisible && (
                        <Tag onClick={showLabelInput}>
                            <PlusOutlined/> New Tag
                        </Tag>
                    )}
                </Form.Item>
                <Form.Item label="内容" {...formItemLayout}>
                    <Editor ref={editorRef}/>
                </Form.Item>
            </Form>
        </Drawer>
    )
}

// 对外暴露
export default forwardRef(EditNote);
