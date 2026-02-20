<script setup lang="ts">
import {ref, shallowRef, onMounted} from "vue";
import {loadRemoteComponent} from '@/utils/sfc-loader'

const props = defineProps({
  title: {type: String, required: true},
  content: {type: String, required: true},
  options: {type: Object},
  remoteUrl: {type: String, required: true}
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
    remoteComponent.value = await loadRemoteComponent(props.remoteUrl)
  } catch (err) {
    error.value = err
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadComponent()
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
        class="remote-component-wrapper"
    />

    <div class="tools-area">
      <div class="tools">
        <el-button type="primary" @click="loadComponent" text icon="download">保存</el-button>
        <el-button type="primary" @click="loadComponent" text icon="CopyDocument">复制</el-button>
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