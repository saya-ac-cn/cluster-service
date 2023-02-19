import { WebviewWindow } from '@tauri-apps/api/window'

// 参考 https://blog.csdn.net/weixin_47367099/article/details/127471024?spm=1001.2014.3001.5502
export const openLoginWindow = () => {
    const webview = new WebviewWindow("login", {
        label: 'login',
        title: '统一身份认证入口',
        url: '/',
        fullscreen: false,
        height: 528,
        width: 890,
        center: true,
        resizable: true,
        alwaysOnTop: false,
        decorations: false
    });

    webview.once("tauri://created", function () {
        // webview window successfully created
        console.log('Login Open Success');
        const stageWindow:WebviewWindow | null = WebviewWindow.getByLabel("stage");
        if (stageWindow){
            stageWindow?.close();
        }
    });
    webview.once("tauri://error", function (e) {
        // an error happened creating the webview window
        console.error('Login Open Fail:',e);
    });
}

export const openStageWindow = () => {
    const webview = new WebviewWindow("stage", {
        label: 'stage',
        title: '控制面板',
        url: '/backstage/me',
        width: 1600,
        height: 900,
        center: true,
        resizable: true,
        alwaysOnTop: false,
        decorations: false
    });

    webview.once("tauri://created", function () {
        // webview window successfully created
        console.log('Stage Open Success');
        const loginWindow:WebviewWindow | null = WebviewWindow.getByLabel("login");
        if (loginWindow){
            loginWindow?.close();
        }
    });
    webview.once("tauri://error", function (e) {
        // an error happened creating the webview window
        console.error('Stage Open Fail:',e);
    });
}