import {lazy, ReactElement} from 'react'
import {HomeOutlined,UserOutlined,MoneyCollectOutlined,ProfileOutlined,NotificationOutlined,FileTextOutlined,ScheduleOutlined,TagOutlined,HistoryOutlined} from '@ant-design/icons';
interface Router {
    name: string,   // 组件名
    path: string,   // 打开路由
    children: any,
    element: any,    // 组件
    display: boolean,  // 是否在菜单中显示
    icon: any
}
const routes : Array<Router> = [
    {
        name: '我',
        path: '/me',
        children: null,
        element: lazy(() => import('../pages/home')),
        display: true,
        icon: HomeOutlined
    },
    {
        name: '记账本',
        path: '/backstage/financial',
        children: [
            {
                name: '记账本',
                path: '/backstage/financial/journal',
                exact: true,
                element: lazy(() => import('../pages/home')),
                display: true,
                icon: MoneyCollectOutlined
            },
            {
                name: '日度报表',
                path: '/backstage/financial/day',
                exact: true,
                element: lazy(() => import('../pages/home')),
                display: true,
                icon: ProfileOutlined
            }
        ],
        element: null,
        display: true,
        icon: HomeOutlined,
    }
]
export default routes