<script setup lang="ts">

import TextCardList from "@/views/workspace/text-card-list.vue";
import CardConfig from "@/views/workspace/card-config.vue";
import {onMounted, provide, reactive, ref} from "vue";
import {Card} from "./card.ts";
import {useRoute} from "vue-router";
import {call} from "@/utils/commands.ts";

const route = useRoute()

const data = ref(new Card('', ''))

const workspace_id = Number(route.params.id);

const workspace = ref({})

onMounted(() => {
  loadData()
})
const loadData = async () => {
  workspace.value = await call('get_workspace', {
    id: workspace_id
  })
}

provide('workspace', workspace)
</script>

<template>
  <div class="pdt10 pdl5 ">
    <el-row :gutter="10">
      <el-col :span="12">
        <card-config v-model="data"></card-config>
      </el-col>
      <el-col :span="12">
        <text-card-list :data="data"></text-card-list>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped lang="scss">

</style>