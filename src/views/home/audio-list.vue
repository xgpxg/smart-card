<script setup lang="ts">
import {ref} from "vue";
import ContextMenu from '@imengyu/vue3-context-menu'


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
        label: "删除",
        onClick: () => {
          alert("You click a menu item");
        }
      },
    ]
  });
}
</script>

<template>
  <div class="pd10">
    <el-input v-model="kw" placeholder="搜索音频/卡片/文本" suffix-icon="search"></el-input>
  </div>
  <div class="pd10">
    <el-button type="primary" icon="plus" class="fill-width">
      添加音频
    </el-button>
  </div>
  <div class="audio-list">
    <div v-for="i in 20" @contextmenu="onContextMenu">
      <div class="audio-item flex-space-between">
        <div>
          <svg-icon icon-class="voice"></svg-icon>
          音频 {{ i }}
        </div>
        <div>
          <el-text size="small" type="info">03:41</el-text>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.audio-list {
  height: calc(100vh - 105px);
  overflow-y: auto;
}

.audio-item {
  padding: 16px 10px;
}

::-webkit-scrollbar {
  width: 0
}
</style>