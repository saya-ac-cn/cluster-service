import React, {forwardRef, useImperativeHandle, useState} from 'react';
import {Form, Input, Modal, Radio, Tooltip} from "antd";
import {clearTrimValueEvent} from "@/utils/string";
import {QuestionCircleOutlined} from "@ant-design/icons";
import {openNotificationWithIcon} from "@/utils/window";
import {createNoteBookApi, updateNoteBookApi} from "@/http/api";


const formItemLayout = {
    labelCol: {span: 6},
    wrapperCol: {span: 14},
};


const EditNoteBook = (props,ref) => {

    const [bookForm] = Form.useForm();

    const [bookId,setBookId] = useState<number>()
    const [open, setOpen] = useState<boolean>(false);
    const [confirmLoading, setConfirmLoading] = useState<boolean>(false);



    // 暴露方法给父组件
    useImperativeHandle(ref,()=>({
        handleDisplay
    }))

    const handleCancel = () => {
        setOpen(false);
    };

    /**
     * 显示弹框
     * @param val
     */
    const handleDisplay = (val) => {
        if(val){
            setBookId(val.id);
            bookForm.setFieldsValue({name: val.name, descript: val.descript, status: val.status});
        }else{
            bookForm.setFieldsValue({name: null, descript: null, status: 1});
        }
        setOpen(true);
    };

    /**
     * 响应用户提交事件
     */
    const handleSubmit = () => {
        bookForm.validateFields(['name', 'descript', 'status']).then(value => {
            if (bookId) {
                // 执行修改
                value.id = bookId;
                handleRenewBook(value);
            } else {
                // 执行添加
                handleAddBook(value);
            }

        }).catch(e => console.log("修改或添加笔记簿错误", e));
    }

    /**
     * 添加笔记簿
     * @param value
     * @returns {boolean}
     */
    const handleAddBook = (value) => {
        Modal.confirm({
            title: '您确定创建该笔记簿?',
            onOk: async () => {
                setConfirmLoading(true);
                const {msg, code} = await createNoteBookApi(value).catch(()=>setConfirmLoading(false));
                setConfirmLoading(false);
                if (code === 0) {
                    openNotificationWithIcon("success", "操作结果", "添加成功");
                    // 调用父页面的刷新数据方法
                    props.refreshPage();
                    handleCancel();
                } else {
                    openNotificationWithIcon("error", "错误提示", msg);
                }
            },
            onCancel() {
                return false;
            },
        });
    }

    /**
     * 修改笔记簿
     * @param value
     * @returns {boolean}
     */
    const handleRenewBook = (value) => {
        Modal.confirm({
            title: '您确定要保存此次修改结果?',
            onOk: async () => {
                setConfirmLoading(true);
                const {msg, code} = await updateNoteBookApi(value).catch(()=>setConfirmLoading(false));
                setConfirmLoading(false);
                if (code === 0) {
                    openNotificationWithIcon("success", "操作结果", "修改成功");
                    // 调用父页面的刷新数据方法
                    props.refreshPage();
                    handleCancel();
                } else {
                    openNotificationWithIcon("error", "错误提示", msg);
                }
            },
            onCancel() {
                return false;
            },
        });
    }

    return (
        <Modal title={bookId ? '修改笔记簿' : '添加笔记簿' } open={open} confirmLoading={confirmLoading} maskClosable={false} okText='保存' onOk={handleSubmit} onCancel={handleCancel}>
            <Form {...formItemLayout} form={bookForm}>
                <Form.Item label="笔记簿名：" {...formItemLayout}
                           getValueFromEvent={(e) => clearTrimValueEvent(e.target.value)} name='name'
                           rules={[{required: true, message: '请输入笔记簿名'}, {
                               min: 2,
                               message: '长度在 2 到 15 个字符'
                           }, {max: 15, message: '长度在 2 到 15 个字符'}]}>
                    <Input placeholder='请输入笔记簿名' maxLength={15}/>
                </Form.Item>
                <Form.Item label="分类描述：" {...formItemLayout}
                           getValueFromEvent={(e) => clearTrimValueEvent(e.target.value)} name='descript'
                           rules={[{required: true, message: '请输入笔记簿描述'}, {
                               min: 1,
                               message: '长度在 1 到 50 个字符'
                           }, {max: 50, message: '长度在 1 到 50 个字符'}]}>
                    <Input.TextArea autosize={{minRows: 2, maxRows: 4}} showCount maxLength={50}
                                    placeholder='请输入笔记簿描述'/>
                </Form.Item>
                <Form.Item label={<span>是否公开&nbsp;<Tooltip
                    title="是否公开显示该笔记簿下的所有笔记"><QuestionCircleOutlined/></Tooltip></span>} {...formItemLayout} name='status'
                           rules={[{required: true, message: '请选择笔记簿公开状态'}]}>
                    <Radio.Group>
                        <Radio value={1}>开启</Radio>
                        <Radio value={2}>关闭</Radio>
                    </Radio.Group>
                </Form.Item>
            </Form>
        </Modal>
    );

}

// 对外暴露
export default forwardRef(EditNoteBook);
