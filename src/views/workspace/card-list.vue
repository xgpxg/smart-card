<script setup lang="ts">

import TextCard from "@/views/workspace/text-card.vue";
import {inject, nextTick, onBeforeUnmount, onMounted, ref, watch} from "vue";
import {open} from '@tauri-apps/plugin-dialog'
import {ElLoading} from "element-plus";
import PubSub from 'pubsub-js'

const workspace = inject<any>('workspace')

const pagination = workspace.value.pagination

const cards = ref<any[]>([])
const cardRefs = ref([])
onMounted(() => {
  splitText()
});

watch(workspace.value, (newValue) => {
  splitText()
}, {deep: true})


const initCardRefs = () => {
  cardRefs.value = Array(cards.value.length).fill(null)
}

watch(cards, () => {
  // 当 cards 数组变化时，重新初始化 refs
  nextTick(() => {
    initCardRefs()
  })
}, {deep: true})

const splitText = () => {
  switch (workspace.value.pagination.page_type) {
    case 'Auto':
      splitTextWithAuto()
      break;
    case 'Single':
      cards.value = [workspace.value.trans_text]
      break;
    case 'CharCount':
      splitTextWithCharCount()
      break;
    case 'Delimiter':
      splitTextWithDelimiter()
      break;
    default:
      splitTextWithAuto()
      break;
  }
}
const splitTextWithAuto = () => {
  const text = workspace.value.trans_text || ""
  const chunkSize = 300; // 目标字符数

  // 支持中英文标点的句子分割正则表达式
  const sentences = text.match(/[^.!?。！？]*[.!?。！？]+/g) || [text];
  const chunks = [];
  let currentChunk = "";

  for (const sentence of sentences) {
    if ((currentChunk + sentence).length <= chunkSize) {
      currentChunk += sentence; // 累加句子
    } else {
      if (currentChunk) {
        chunks.push(currentChunk.trim()); // 保存当前块
      }
      currentChunk = sentence; // 开始新块
    }
  }

  if (currentChunk) {
    chunks.push(currentChunk.trim()); // 保存最后一块
  }

  cards.value = chunks;
}

const splitTextWithCharCount = () => {
  const charCount = workspace.value.pagination.char_count
  const text = workspace.value.trans_text || ""
  const chunks = [];
  for (let i = 0; i < text.length; i += charCount) {
    chunks.push(text.substring(i, i + charCount));
  }
  cards.value = chunks
}

const splitTextWithDelimiter = () => {
  const delimiter = workspace.value.pagination.delimiter
  const text = workspace.value.trans_text || ""
  cards.value = text.split(unescapeString(delimiter)).filter((chunk: string) => chunk.trim() !== '');
}

const unescapeString = (str) => {
  return str
      .replace(/\\n/g, '\n')
      .replace(/\\t/g, '\t')
      .replace(/\\r/g, '\r')
      .replace(/\\"/g, '"')
      .replace(/\\\\/g, '\\');
}

const isShow = ref(true)
PubSub.subscribe('workspace/style/change', () => {
  isShow.value = false
  nextTick(() => {
    isShow.value = true
  })
})

PubSub.subscribe('workspace/card/save', async () => {
  console.log('cardRefs', cardRefs.value)
  const path = await open({
    directory: true,
    multiple: false,
    title: '选择保存文件夹'
  });

  if (!path) {
    return
  }
  const loading = ElLoading.service({
    text: '正在导出图片...',
  })
  cardRefs.value.forEach((cardRef, index) => {
    if (cardRef) {
      console.log(`Card ${index}:`, cardRef)
      cardRef.saveImage(path + '/' + workspace.value.text_title + '/' + workspace.value.text_title + '_' + index + '.png')
    }
  })
  loading.close()

})

onBeforeUnmount(() => {
  PubSub.unsubscribe('workspace/style/change')
  PubSub.unsubscribe('workspace/card/save')
})

</script>

<template>
  <!--  <div class="pdr5 pdl5 mb10">
      <el-button type="primary" class="fill-width" icon="download">保存</el-button>
    </div>-->
  <div class="text-card-list" v-if="isShow">
    <el-row :gutter="10">
      <el-col v-for="(item, index) in cards" :span="24" :xs="24" :sm="24" :xm="24" :md="24" :lg="12" :xl="8">
        <text-card
            :ref="(el) => { cardRefs[index] = el }"
            :url="`/cards/component/${workspace.style_id}.vue`"
            :title="workspace.text_title"
            :content="item"
            :font="workspace.font"
        ></text-card>
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