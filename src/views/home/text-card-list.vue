<script setup lang="ts">

import TextCard from "@/views/home/text-card.vue";
import {Card} from "@/views/home/card.ts";

defineProps({
  data: {
    type: Card,
    required: true
  }
});
</script>

<template>
  <div class="pdr5 pdl5 mb10">
    <el-button type="primary" class="fill-width" icon="download">保存</el-button>
  </div>
  <div class="text-card-list">
    <el-row :gutter="10">
      <el-col v-for="i in 10" :span="24" :xs="24" :sm="24" :xm="24" :md="24" :lg="12" :xl="8">
        <div class="text-card">
          <text-card
              :title="`测试卡片 ${i}`"
              :content="data.content"
              :remote-url="`http://127.0.0.1:10800/public/test/test.vue?v=${i}`"
          ></text-card>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped lang="scss">
.text-card-list {
  overflow-y: auto;
  overflow-x: hidden;
  height: calc(100vh - 52px);
}


</style>