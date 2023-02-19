import {lazy, ReactElement} from 'react'
import {HomeOutlined,UserOutlined,MoneyCollectOutlined,ProfileOutlined,NotificationOutlined,FileTextOutlined,ScheduleOutlined,TagOutlined,HistoryOutlined} from '@ant-design/icons';
interface Router {
    name: string,   // 组件名
    path: string,   // 打开路由
    root: boolean, // 是否为根节点，由于在antd渲染根节点时，需要特殊处理，
    children: any,
    element: any,    // 组件
    display: boolean,  // 是否在菜单中显示
    icon: any
}
const routes : Array<Router> = [
    {
        name: '我',
        path: '/me',
        root:true,
        children: null,
        element: lazy(() => import('../pages/home')),
        display: true,
        icon: HomeOutlined
    },
    {
        name: '记账本',
        path: '/financial',
        root:true,
        children: [
            {
                name: '记账本',
                path: '/financial/journal',
                root: false,
                children: null,
                element: lazy(() => import('../pages/home')),
                display: true,
                icon: MoneyCollectOutlined
            },
            {
                name: '日度报表',
                path: '/financial/day',
                root: false,
                children: null,
                element: lazy(() => import('../pages/home')),
                display: true,
                icon: ProfileOutlined
            },
            {
                name: '笔记簿',
                path: '/financial/note',
                root: false,
                children: null,
                element: lazy(() => import('../pages/memory/note')),
                display: true,
                icon: FileTextOutlined
            },
        ],
        element: null,
        display: true,
        icon: HomeOutlined,
    }
]
export default routes