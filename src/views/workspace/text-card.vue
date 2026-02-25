<script setup lang="ts">
import {ref, shallowRef, onMounted} from "vue";
import {loadRemoteComponent} from '@/utils/sfc-loader'
import html2canvas from 'html2canvas'
import {ElMessage} from 'element-plus'
import {save} from '@tauri-apps/plugin-dialog';
import {call} from "@/utils/commands.ts";

const props = defineProps({
  url: {type: String, required: true},
  title: {type: String, required: true},
  content: {type: String, required: true},
  font: {type: Object, default: () => ({font_family: 'Arial', font_size: '14'})},
})

const dom = ref()
const remoteComponent = shallowRef(null)
const loading = ref(false)
const error = ref<string | unknown | null>(null)

// 加载远程组件
const loadComponent = async () => {
  loading.value = true
  error.value = null

  try {
    remoteComponent.value = await loadRemoteComponent(props.url)
  } catch (err) {
    error.value = err
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadComponent()
})


const saveImage = async (path: string) => {
  if (!path) {
    path = await save({
      filters: [
        {
          name: 'Image',
          extensions: ['png'],
        },
      ],
      defaultPath: props.title + '.png'
    });
  }


  if (!dom.value) {
    ElMessage.error('无法获取组件DOM元素')
    return
  }

  try {
    const canvas = await html2canvas(dom.value, {
      backgroundColor: null,
      scale: 3,
      useCORS: true,
      logging: false,
      width: dom.value.scrollWidth,
      height: dom.value.scrollHeight
    })

    canvas.toBlob(async (blob) => {
      if (blob) {
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        await call('save_file', {
          path,
          content: uint8Array
        });
        ElMessage({
          message: '已保存',
          grouping: true,
          type: 'success',
        })
      }
    }, 'image/png')
  } catch (error) {
    console.error('保存失败:', error)
    ElMessage.error('保存失败: ' + (error as Error).message)
  }
}

const copyImage = async () => {
  if (!dom.value) {
    ElMessage.error('无法获取组件DOM元素')
    return
  }

  try {
    const canvas = await html2canvas(dom.value, {
      backgroundColor: null,
      scale: 3,
      useCORS: true,
      logging: false,
      width: dom.value.scrollWidth,
      height: dom.value.scrollHeight
    })

    canvas.toBlob(async (blob) => {
      if (blob) {
        if (navigator.clipboard && navigator.clipboard.write) {
          const clipboardItem = new ClipboardItem({
            'image/png': blob
          })

          await navigator.clipboard.write([clipboardItem])
          ElMessage.success('图片已复制到剪贴板')
        } else {
          ElMessage.error('您的系统不支持复制图片到剪贴板')
        }
      }
    })
  } catch (error) {
    console.error('保存失败:', error)
    ElMessage.error('保存失败: ' + (error as Error).message)
  }
}

defineExpose({
  saveImage
})
</script>

<template>
  <div ref="dom" class="card">
    <div v-if="loading" class="loading-state">
      正在加载...
    </div>

    <div v-else-if="error" class="error-state">
      加载失败: {{ error }}
    </div>

    <component
        v-else-if="remoteComponent"
        :is="remoteComponent"
        :title="title"
        :content="content"
        :font="font"
        class="remote-component-wrapper"
    />

    <div class="tools-area">
      <div class="tools">
        <el-button type="primary" @click="saveImage(null)" text icon="download">保存</el-button>
        <el-button type="primary" @click="copyImage" text icon="CopyDocument">复制</el-button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.card {
  &:hover {
    .tools-area {
      display: block;
      animation: fadeIn 0.3s ease-in-out;
    }
  }
}

.loading-state {
  padding: 20px;
  text-align: center;
  color: #666;
  font-size: 14px;
}

.error-state {
  padding: 20px;
  text-align: center;
  color: #f56c6c;
  font-size: 14px;
  border: 1px solid #f56c6c;
  border-radius: 4px;
  margin-bottom: 10px;
}

.remote-component-wrapper {
  margin-bottom: 10px;
  border: 1px solid #ebeef5;
  border-radius: 20px;
  overflow: hidden;
}

.tools-area {
  position: relative;
  display: none;

  .tools {
    position: absolute;
    bottom: 0;
    right: 0;
    margin-bottom: 15px;
    margin-right: 20px;

    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(10px);
    border-radius: 20px;
    padding: 2px 10px;

    .el-button {
      color: #ffffff;
      background: transparent;
      border: none;
      zoom: 0.85;
      padding: 4px;
    }
  }
}

</style>