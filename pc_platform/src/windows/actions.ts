import { WebviewWindow } from '@tauri-apps/api/window'

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
    });
    webview.once("tauri://error", function (e) {
        // an error happened creating the webview window
        console.log('Login Open Fail:',e);
    });

    const stageWindow:WebviewWindow | null = WebviewWindow.getByLabel("stage");
    if (stageWindow){
        stageWindow.close();
    }
}

export const openStageWindow = () => {
    const webview = new WebviewWindow("stage", {
        label: 'stage',
        title: '控制面板',
        url: '/stage/home',
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
    });
    webview.once("tauri://error", function (e) {
        // an error happened creating the webview window
        console.log('Stage Open Fail:',e);
    });

    const loginWindow:WebviewWindow | null = WebviewWindow.getByLabel("login");
    if (loginWindow){
        loginWindow.close();
    }
}