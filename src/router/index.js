import {createRouter, createWebHistory, createWebHashHistory} from 'vue-router'
import Layout from '@/layout/index.vue'

const routes = [
    {
        path: '/init',
        component: () => import('@/views/init/index.vue'),
        name: 'Init',
        meta: {title: '初始化', icon: 'init', affix: true,}
    },
    {
        path: '/',
        component: Layout,
        redirect: 'welcome',
        children: [
            {
                path: 'welcome',
                component: () => import('@/views/welcome/index.vue'),
                name: 'Welcome',
                meta: {title: '首页', icon: 'welcome', affix: true,}
            },
            {
                path: 'workspace',
                component: () => import('@/views/workspace/index.vue'),
                name: 'Workspace',
                meta: {title: '首页', icon: 'workspace', affix: true,}
            },
        ]
    },
]


const router = createRouter({
    //history: createWebHashHistory(),  // hash 模式
    history: createWebHistory(),  // history 模式
    routes: routes
})

// 全局路由守卫
router.beforeEach((to, from, next) => {
    // console.log(to, from)
    if (to.meta.title) {
        document.title = `${to.meta.title}`;
    }
    next()
})

export default router
