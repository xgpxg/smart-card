<script setup lang="ts">

import CardConfig from "@/views/workspace/card-config.vue";
import {onMounted, provide, ref, watch} from "vue";
import {useRoute} from "vue-router";
import {call} from "@/utils/commands.ts";
import CardList from "@/views/workspace/card-list.vue";
import {store} from "@/store";

const route = useRoute()

const workspace_id = Number(route.params.id);
const workspace = ref(store.state.workspace.current)

provide('workspace', workspace)

onMounted(() => {
  // store里没有值，可能是直接刷新页面，重新加载数据
  // 这种情况应该只会在开发阶段
  if (workspace.value === null) {
    loadData()
  }

  // 监听重新加载workspace事件
  // 解析状态发生变化时会触发
  PubSub.subscribe('workspace/reload', (msg, data) => {
    if (data.id === workspace_id) {
      loadData()
    }
  })
})

const loadData = async () => {
  workspace.value = await call('get_workspace', {
    id: workspace_id
  })
}


watch(workspace, (newValue) => {
  console.log('workspace changed', newValue)
  call('update_workspace', {
    workspace: newValue
  })
}, {deep: true})
</script>

<template>
  <div class="pdt10 pdl5" v-if="workspace">
    <el-row :gutter="10">
      <el-col :span="12">
        <card-config></card-config>
      </el-col>
      <el-col :span="12">
        <card-list></card-list>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped lang="scss">

</style>