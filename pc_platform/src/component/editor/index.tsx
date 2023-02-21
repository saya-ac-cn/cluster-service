import React, {forwardRef, useState, useRef, useImperativeHandle} from 'react';
import ForEditor from 'for-editor'
import axios from "axios";
import {uploadBase64PictureApi} from "@/http/api";
import {openNotificationWithIcon} from "@/utils/window";
import {isEmptyObject} from "@/utils/var"
import Storage from "@/utils/storage";
import {Spin} from "antd";

const toolbar = {
    h1: true, // h1
    h2: true, // h2
    h3: true, // h3
    h4: true, // h4
    img: true, // 图片
    link: true, // 链接
    code: true, // 代码块
    preview: true, // 预览
    // expand: true, // 全屏
    /* v0.0.9 */
    undo: true, // 撤销
    redo: true, // 重做
    // save: true, // 保存
    /* v0.2.3 */
    subfield: true, // 单双栏模式
}

const Editor = (props,ref) => {

    /**
     * 操作dom节点的ref
     */
    const editorRef = useRef<any>();

    /**
     * 组件特殊不能赋空值
     */
    const [content, setContent] = useState<string>('');
    /**
     * 编辑器的loading
     */
    const [loading, setLoading] = useState(false);

    /**
     * 暴露方法给父组件
     */
    useImperativeHandle(ref,()=>({
        initEditor,getContent
    }))

    /**
     * 初始化Markdown编辑器，本方法由外界触发
     * @param val markdown内容
     */
    const initEditor = (val:string) => {
        setContent(val?val:'')
    };

    /**
     * 内容改变时回调
     * @param value markdown内容
     */
    const handleChange = (value:string) => {
        setContent(value?value:'')
    };

    /**
     * 添加图片时回调
     * @param file 图片文件
     */
    const handleUploadPicture = async (file: File) => {
        const {data} = await uploadPicture(file);
        if (!isEmptyObject(data)) {
            // 第一个参数，图片的显示的描述信息，第二个参数，图片的url
            editorRef.current!.$img2Url(file.name, data.link)
        } else {
            editorRef.current!.$img2Url(file.name, 'file_url')
        }
    };


    /**
     * 向后台上传图片
     * @param file 图片文件
     */
    const uploadPicture = (file: File) => {
        let access_token = Storage.get(Storage.ACCESS_KEY)
        return new Promise(
            (resolve, reject) => {
                setLoading(true);
                let img = new Image();
                let reader = new FileReader();
                reader.readAsDataURL(file);
                reader.onload = function (e) {
                    console.log("onload",e)
                    img.src = e.target.result as string;
                };
                img.onload = function () {
                    let para = {
                        name: file.name,
                        content: img.src
                    };
                    axios({
                        method: "POST",
                        url: uploadBase64PictureApi,
                        data: para,
                        headers: {
                            "Content-Type": "application/json",
                            "access_token":access_token
                        },
                    }).then(response => {
                        const data = response.data; // 得到响应数据
                        setLoading(false);
                        if (data.code === 0) {
                            resolve({data: {link: data.data}})
                        } else {
                            openNotificationWithIcon("error", "错误提示", data.msg);
                            resolve({data: {link: ''}})
                        }
                        // /files/picture/illustrated/Pandora/20190728/2019072823539.png
                    }).catch(error => {
                        setLoading(false);
                        openNotificationWithIcon("error", "图片上传出错了", error.message);
                        reject(error)
                    })
                }
            })
    };

    /**
     * 获取markdown内容
     * @returns markdown内容
     */
    const getContent = () => {
        return content;
    };

    /**
     * 解析粘贴板（图片）
     * @param event
     */
    const pastePicture = (event) => {
        event.stopPropagation()
        const data = event.clipboardData || window.clipboardData
        const items = data.items
        // 存储文件数据
        let tempFile = null
        if (items && items.length) {
            // 检索剪切板items
            for (const item of items) {
                if (item.type.indexOf('image') !== -1) {
                    tempFile = item.getAsFile()
                    break
                }
            }
        }
        if (!tempFile){
            return
        }
        // 上传文件
        handleUploadPicture(tempFile).then(e => console.error('图片上传失败：',e))
    }

    return (
            <Spin spinning={loading} onPaste={pastePicture}>
                <ForEditor
                    ref={editorRef}
                    value={content}
                    lineNum={false}
                    style={{border:'none',boxShadow:'none'}}
                    addImg={($file: File) => handleUploadPicture($file)}
                    onChange={value => handleChange(value)}
                    toolbar={toolbar}
                />
            </Spin>
    );
}

// 对外暴露
export default forwardRef(Editor);
