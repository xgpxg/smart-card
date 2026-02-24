import {createApp} from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import 'virtual:svg-icons-register';
import SvgIcon from '@/components/SvgIcon/index.vue'
import router from './router'
import {store} from './store'
import PubSub from 'pubsub-js'
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import 'animate.css';

import '@/styles/index.scss'

const app = createApp(App)

app.use(router)
app.use(store)
app.use(ElementPlus, {locale: zhCn,})
app.use(PubSub)
app.component('SvgIcon', SvgIcon)

app.mount("#app")

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}