import React, {useEffect, useState} from 'react';
import Cropper from 'react-cropper'
import 'cropperjs/dist/cropper.css'
import {Modal} from 'antd';
import './index.less';
import {uploadLogoApi} from "@/http/api";
import {openNotificationWithIcon} from "@/utils/window";
import Storage from "@/utils/storage";


/*
 * 文件名：index.jsx
 * 作者：liunengkai
 * 创建日期：2022-10-05 - 22:04
 * 描述：
 * 参考1：https://www.jianshu.com/p/eeaa9fba56d0
 * 参考2：https://blog.csdn.net/qq_37043418/article/details/109238963
 */

// 定义组件（ES6）
const CropperComponent = () => {

    const [open,setOpen] = useState<boolean>(false)
    const [confirm,setConfirm] = useState<boolean>(false)
    const [src,setSrc] = useState<string | ArrayBuffer | null>(null)
    const [headerImage,setHeaderImage] = useState<string>('')
    const [cropper,setCropper] = useState<Cropper>('')

    useEffect(() => {
        const fileReader = new FileReader();
        fileReader.onload = (e) => {
            setSrc(e.target.result)
        };
        // 从本地缓存取出当前用户的logo
        const user = Storage.get(Storage.USER_KEY);
        const headerImage = user.logo;
        setHeaderImage(headerImage);
        // let file = props.uploadedImageFile;
        // if (file) {
        //   fileReader.readAsDataURL(props.uploadedImageFile)
        // }
    }, []);


    //选择图片
    const handleFileChange = (e) => {
      const file = e.target.files[0];
      if (file) {
          //文件最多不能超过5M
          if (file.size / 1024 <= 5 * 1024) {
              const reader = new FileReader();
              reader.onload = () => {
                  setSrc(reader.result)
                  setOpen(true)
              };
              reader.readAsDataURL(file);
          } else {
              openNotificationWithIcon("error", "文件过大", "所选文件不能超过5M");
          }
      } else {
          openNotificationWithIcon("error", "上传提示", "请选择文件");
      }
      e.target.value = ''
    };

    //保存裁切
    const saveCropperImg = () => {
        // 用户未选择
        if (cropper.getCroppedCanvas() === 'null') {
            return false
        }
        // Crop
        const croppedCanvas = cropper.getCroppedCanvas();
        // Round
        let roundedCanvas = getRoundedCanvas(croppedCanvas);
        //获取Canvas图片，base64
        let headerImage = roundedCanvas.toDataURL();
        setHeaderImage(headerImage)
        upload(headerImage)
    };

    const upload = async (image) => {
        setConfirm(true)
        //这边写图片的上传
        let para = {
            content: image.toString()
        };
        const result = await uploadLogoApi(para).catch(()=>{setConfirm(true)});
        setConfirm(false)
        let {msg, code,data} = result;
        if (code === 0) {
            openNotificationWithIcon("success", "上传结果", "上传成功");
            // 覆写到本地缓存
            const user = Storage.get(Storage.USER_KEY);
            user.logo = data
            Storage.add(Storage.USER_KEY,user)
            setOpen(false);
            setHeaderImage(data)
        } else {
            openNotificationWithIcon("error", "上传结果", msg);
        }
    };

    // 裁剪一个圆形的图片
    const getRoundedCanvas = (sourceCanvas) => {
        let canvas = document.createElement('canvas');
        let context = canvas.getContext('2d');
        let width : number = sourceCanvas.width;
        let height: number = sourceCanvas.height;
        canvas.width = width;
        canvas.height = height;
        context.imageSmoothingEnabled = true;
        context.drawImage(sourceCanvas, 0, 0, width, height);
        context.globalCompositeOperation = 'destination-in';
        context.beginPath();
        context.arc(width / 2, height / 2, Math.min(width, height) / 2, 0, 2 * Math.PI, true);
        context.fill();
        return canvas;
    };

    //取消裁切
    const cancelCropper = () => {
        setOpen(false)
    };

    return (
        <div className='upload-cropper'>
            {/* 选择图片按钮 */}
            <div className="show">
                <div className="picture" style={{backgroundImage: 'url(' + headerImage + ')'}}></div>
                <div className="mask-layer">
                    {/*<EditOutlined className='edit-user-logo'>*/}
                    {/*</EditOutlined>*/}
                    <span className="upload-span" role="button">
                                <input
                                    type="file"
                                    accept="image/*"
                                    onChange={handleFileChange}
                                />
                            </span>
                </div>
            </div>
            {/* 裁切图片modal框 */}
            <Modal
                title="图片裁切"
                width="50vw"
                closable={false}
                destroyOnClose={true}
                maskClosable={false}
                open={open}
                onOk={saveCropperImg}
                confirmLoading={confirm}
                onCancel={cancelCropper}>
                <div className="cropperModal">
                    <Cropper
                        src={src}//图片路径，即是base64的值，在Upload上传的时候获取到的
                        style={{height: 400, width: '100%'}}
                        preview='.cropper-preview'
                        className="company-cropper"
                        viewMode={1} //定义cropper的视图模式
                        zoomable={true} //是否允许放大图像
                        aspectRatio={1} //image的纵横比1:1
                        guides={false} //显示在裁剪框上方的虚线
                        background={false} //是否显示背景的马赛克
                        rotatable={true} //是否旋转
                        //ref='cropper'//在官方更新之后ref不在支持，应写为以下
                        //crop={this._crop.bind(this)}
                        onInitialized={ (instance) => setCropper(instance)}
                    />
                    <div className='preview-button'>
                        <div className="cropper-preview" style={{
                            borderRadius: '50%',
                            height: 100,
                            width: '100%'
                        }}></div>
                    </div>
                </div>
            </Modal>

        </div>
    )

}

// 对外暴露
export default CropperComponent;
