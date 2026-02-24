<script setup lang="ts">
import {onMounted, ref} from "vue";
import ContextMenu from '@imengyu/vue3-context-menu'
import {open} from '@tauri-apps/plugin-dialog'
import {useRouter} from "vue-router";
import {call} from "@/utils/commands.ts";
import {store} from "@/store";

const router = useRouter()
const kw = ref(null)
const workspaces = ref<any[]>([])
const selectedFiles = ref<string[]>([])

onMounted(() => {
  // 加载列表
  loadData()

  // 定时刷新
  setInterval(() => {
    loadData()
  }, 1000)
})

const loadData = async () => {
  workspaces.value = await call('list_workspaces')
}


const toWorkspace = async (workspace: any) => {
  // 先加载，放到store里
  const w = await call('get_workspace', {
    id: workspace.id
  })
  store.commit('workspace/setWorkspace', w)

  //跳转
  await router.push({name: 'Workspace', params: {id: workspace.id}})
}


const addWorkspace = async (filePath: string) => {
  await call('add_workspace', {
    file_path: filePath
  });
}

const deleteWorkspace = async (id: number) => {
  await call('delete_workspace', {
    id
  });
}

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

  if (selectedFiles.value.length === 0) {
    return
  }

  await addWorkspace(selectedFiles.value[0])

  await loadData()

}

const onContextMenu = (e: MouseEvent, item: any) => {
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
          deleteWorkspace(item.id).then(() => {
            loadData();
          })
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
    <el-button @click="openFileDialog" type="primary" icon="plus" class="fill-width">
      添加音频
    </el-button>
  </div>
  <div class="audio-list">
    <div v-for="item in workspaces" @contextmenu="onContextMenu ($event, item) ">
      <div class="audio-item flex-space-between" @click="toWorkspace(item)">
        <div class="ellipsis" :title="item.name">
          <svg-icon v-if="item.trans_text_status === 'Processing'" icon-class="loading" class="loading"></svg-icon>
          <svg-icon v-else icon-class="voice"></svg-icon>
          {{ item.file_name }}
        </div>
        <!--        <div class="ml5">
                  <el-text size="small" type="info">03:41</el-text>
                </div>-->
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