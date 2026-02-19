<script setup lang="ts">

import TextCardList from "@/views/workspace/text-card-list.vue";
import CardConfig from "@/views/workspace/card-config.vue";
import {ref} from "vue";
import {Card} from "./card.ts";


const data = ref(new Card('',''))
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