# Tauri + React + Typescript

# 初始化
```shell
# 使用 yarn
yarn create tauri-app

# 或使用 npx
npx create-tauri-app
```

# 项目结构

```text
[pc_platform] # 项目名称
├─ [node_modules] # 前端依赖
├─ [src] # 前端程序源
├─ [src-tauri] # Tauri 程序源
│    ├─ [icons] # 应用程序图标
│    ├─ [src] # Tauri App 程序源，例如系统菜单，托盘，插件配置等
│    ├─ [target] # 构建的产物会被放入此文件夹中，target 目录的结构取决于是否使用 --target 标志为特定的平台构建
│    ├─ build.rs # Tauri 构建应用
│    ├─ Cargo.lock # 包含了依赖的精确描述信息，类似于 yarn.lock 或 package-lock.json
│    ├─ Cargo.toml # Tauri (Rust) 项目清单
│    └─ tauri.conf.json # 自定义 Tauri 应用程序的配置文件，例如应用程序窗口尺寸，应用名称，权限等
├─ index.html # 项目主界面
├─ package.json # 前端项目清单
├─ tsconfig.json # typescript 配置文件
├─ vite.config.ts # vite 配置文件
├─ package-lock.json # 前端依赖的精确描述信息
└─ ... # 其他
```

# 启动项目

有两种启动方式：

## 1. 启动 web 项目

纯前端项目，不和操作系统产生任何交互
```shell
npm run dev
```

## 2. 启动 tauri 项目

需要和操作系统产生交互，如系统文件读写操作

第一次启动项目时，tauri 会根据src-tauri/Cargo.toml 去下载相关依赖（导致第一次启动比较慢），第二次启动会快很多。

```shell
npm run tauri dev
```

# 检查信息

检查 Tauri 信息以确保一切设置正确，在对问题进行分类时，此信息可能很有用。

```shell
yarn tauri info
```

# 解决错误
```text

antd报错Instance created by `useForm` isnot connected to any Form element. Forget to pass `form` prop

提示：使用了Modal组件
例如：我在Form外层包裹了Modal（Drawer同理）组件，在调用form的实例时，Modal内部的组件并未渲染，才导致了如此错误。

强制Modal渲染 通过antd提供的forceRender属性即可

```
# 关于更换图标

1、准备一个尺寸为1240 x 1240 的 PNG图片或者正方形的SVG ，文件命名为app-icon.png，将图片文件放置在项目的根目录

2、cd进入到项目根目录，执行npm run tauri icon命令 然后就会自动生成相关尺寸的图标，并自动应用到对应的位置 图标存放和应用路径为项目根目录下的src-tauri\icons\

```shell

PS E:\rust\src\cluster-service\pc_platform> npm run tauri icon

> pc_platform@0.0.0 tauri
> tauri icon

        Appx Creating StoreLogo.png
        Appx Creating Square30x30Logo.png
        Appx Creating Square44x44Logo.png
        Appx Creating Square71x71Logo.png
        Appx Creating Square89x89Logo.png
        Appx Creating Square107x107Logo.png
        Appx Creating Square142x142Logo.png
        Appx Creating Square150x150Logo.png
        Appx Creating Square284x284Logo.png
        Appx Creating Square310x310Logo.png
        ICNS Creating icon.icns
         ICO Creating icon.ico
         PNG Creating 32x32.png
         PNG Creating 128x128.png
         PNG Creating 128x128@2x.png
         PNG Creating icon.png


```

# 参考
1、tauri.conf.json 配置 https://zhuanlan.zhihu.com/p/536675879

2、动态加载路由 https://www.codeleading.com/article/97586131268/

3、https://blog.csdn.net/m0_73121002/article/details/128331442

4、react-router v6 https://www.jianshu.com/p/1c54f96831b9

5、阻止父组件更新带来的子组件也重新渲染 https://blog.csdn.net/qq_37766810/article/details/121810665
