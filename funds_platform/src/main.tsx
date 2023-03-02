import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {ConfigProvider} from 'antd';
import zhCN from 'antd/locale/zh_CN'
import 'dayjs/locale/zh-cn'
import './index.css'

/**
 * 入口
 */
type ThemeData = {
    borderRadius: number;
    colorPrimary: string;
};
const defaultData: ThemeData = {
    borderRadius: 6,
    colorPrimary: '#7bc0fe',
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <ConfigProvider locale={zhCN.default ?? zhCN}
                    theme={{ token: { colorPrimary: defaultData.colorPrimary, borderRadius: defaultData.borderRadius } }}>
        <App />
    </ConfigProvider>
);
