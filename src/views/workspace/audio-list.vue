<script setup lang="ts">
import {ref} from "vue";
import ContextMenu from '@imengyu/vue3-context-menu'
import {open} from '@tauri-apps/plugin-dialog'
import {sep} from '@tauri-apps/api/path';
import {useRouter} from "vue-router";

const router = useRouter()
const kw = ref(null)

function onContextMenu(e: MouseEvent) {
  //prevent the browser's default menu
  e.preventDefault();
  //show your menu
  ContextMenu.showContextMenu({
    x: e.x,
    y: e.y,
    theme: 'win10 dark',
    items: [
      {
        label: "重命名",
        onClick: () => {
          alert("You click a menu item");
        }
      },
      {
        label: "删除",
        onClick: () => {
          alert("You click a menu item");
        }
      },
    ]
  });
}

const selectedFiles = ref<string[]>([])
const openFileDialog = async () => {
  const files = await open({
    multiple: true, // 允许选择多个文件
    filters: [
      {
        name: '音频文件',
        extensions: ['mp3', 'wav', 'ogg', 'm4a'],
      },
    ],
  })

  if (Array.isArray(files)) {
    selectedFiles.value = files
  } else if (files !== null) {
    selectedFiles.value = [files]
  }

  audioList.value.push(...selectedFiles.value.map(file => {
    return {
      name: file.substring(file.lastIndexOf(sep()) + 1)
    }
  }))

}

const audioList = ref<any[]>([])

const toWorkspace = () => {
  router.push({name: 'Workspace'})
}
</script>

<template>
  <div class="pd10">
    <el-input v-model="kw" placeholder="搜索音频/卡片/文本" suffix-icon="search"></el-input>
  </div>
  <div class="pd10">
    <el-button @click="openFileDialog" type="primary" icon="plus" class="fill-width">
      添加音频
    </el-button>
  </div>
  <div class="audio-list">
    <div v-for="item in audioList" @contextmenu="onContextMenu">
      <div class="audio-item flex-space-between" @click="toWorkspace">
        <div class="ellipsis" :title="item.name">
          <svg-icon icon-class="voice"></svg-icon>
          {{ item.name }}
        </div>
        <div class="ml5">
          <el-text size="small" type="info">03:41</el-text>
        </div>
      </div>
    </div>
  </div>

</template>

<style scoped lang="scss">
.audio-list {
  height: calc(100vh - 145px);
  overflow-y: auto;
}

.audio-item {
  padding: 16px 10px;

  &:hover {
    background-color: var(--el-color-primary-light-5);
    cursor: default;

  }
}

::-webkit-scrollbar {
  width: 0
}
</style>