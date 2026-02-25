<script setup lang="ts">
import {call} from "@/utils/commands";
import {open} from "@tauri-apps/plugin-dialog";
import DropHandler from "@/views/welcome/drop-handler.vue";

const openFileDialog = async () => {
  const path = await open({
    multiple: false,
    filters: [
      {
        name: '音频文件',
        extensions: ['mp3', 'wav', 'ogg', 'm4a'],
      },
    ],
  })

  if (!path) {
    return
  }

  await call('add_workspace', {
    file_path: path
  })
}

</script>

<template>
  <div class="flex flex-column-v flex-center" style="height: 80vh">
    <div class="logo-container mb50 mt50">
      <div class="logo-circle">
        <img src="/logo.png" class="icon"/>
      </div>
      <div class="ml20">
        <h1>墨音笺</h1>
        <h3 class="mt10">将音频转换为文字卡片</h3>
      </div>
    </div>
    <div class="import-area mt50" @click="openFileDialog">
      <el-text>点击导入或将文件拖拽到这里</el-text>
    </div>
  </div>
  <drop-handler></drop-handler>
</template>

<style scoped lang="scss">
.import-area {
  width: 500px;
  margin: 0 auto;
  text-align: center;
  line-height: 100px;
  color: #ccc;
  cursor: pointer;
  background: var(--el-color-primary-light-5);
  border-radius: 10px;

  &:hover {
    color: #409eff;
  }
}

.logo-container {
  // 居中
  display: flex;
  justify-content: center;
  align-items: center;

  .logo-circle {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    background: #1a1a1a;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: breathe 4s infinite ease-in-out, glow 2s infinite alternate;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    position: relative;
    overflow: hidden;

    &::before {
      content: '';
      position: absolute;
      top: -50%;
      left: -50%;
      width: 200%;
      height: 200%;
      background: rgba(255, 255, 255, 0.05);
      transform: rotate(45deg);
      animation: shine 3s infinite;
    }

    .icon {
      width: 80px;
      height: 80px;
      color: white;
      z-index: 2;
    }
  }
}
</style>