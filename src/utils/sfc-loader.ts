import {loadModule} from 'vue3-sfc-loader'
import * as Vue from 'vue'

// 配置 vue3-sfc-loader
const options = {
    moduleCache: {
        vue: Vue
    },
    async getFile(url: string) {
        const res = await fetch(url)
        if (!res.ok) {
            throw new Error(`Failed to fetch ${url}: ${res.statusText}`)
        }
        return {
            getContentData: (asBinary: boolean) => asBinary ? res.arrayBuffer() : res.text(),
        }
    },
    addStyle(textContent: string) {
        const style = Object.assign(document.createElement('style'), {
            textContent
        })
        const ref = document.head.getElementsByTagName('style')[0] || null
        document.head.insertBefore(style, ref)
    },
}

// 创建加载器实例
export function loadRemoteComponent(url: string) {
    return loadModule(url, options)
}