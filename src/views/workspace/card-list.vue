<script setup lang="ts">

import TextCard from "@/views/workspace/text-card.vue";
import {inject, nextTick, onBeforeUnmount, onMounted, ref, watch} from "vue";
import {open} from '@tauri-apps/plugin-dialog'
import {ElLoading} from "element-plus";
import PubSub from 'pubsub-js'

const workspace = inject<any>('workspace')
const cards = ref<any[]>([])
const cardRefs = ref([])

onMounted(() => {
  splitText()
});

watch(() => workspace.value, (newValue) => {
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

  // 更完善的文本分割逻辑
  const chunks = [];
  let currentChunk = "";

  // 首先按段落分割（双换行符）
  const paragraphs = text.split(/\n\s*\n/);

  for (const paragraph of paragraphs) {
    // 清理段落前后空白
    const cleanParagraph = paragraph.trim();
    if (!cleanParagraph) continue;

    // 如果单个段落就超过限制，需要进一步分割
    if (cleanParagraph.length > chunkSize) {
      // 先保存当前块（如果有的话）
      if (currentChunk) {
        chunks.push(currentChunk.trim());
        currentChunk = "";
      }

      // 按句子分割长段落
      const sentences = cleanParagraph.match(/[^.!?。！？\n]*[.!?。！？\n]+/g) || [cleanParagraph];

      for (const sentence of sentences) {
        const cleanSentence = sentence.trim();
        if (!cleanSentence) continue;

        if ((currentChunk + cleanSentence).length <= chunkSize) {
          currentChunk += cleanSentence + " ";
        } else {
          if (currentChunk) {
            chunks.push(currentChunk.trim());
          }
          // 如果单个句子就超过限制，强制分割
          if (cleanSentence.length > chunkSize) {
            // 按字符数强制分割
            for (let i = 0; i < cleanSentence.length; i += chunkSize) {
              chunks.push(cleanSentence.substring(i, i + chunkSize));
            }
            currentChunk = "";
          } else {
            currentChunk = cleanSentence + " ";
          }
        }
      }
    } else {
      // 段落长度在限制内
      if ((currentChunk + cleanParagraph).length <= chunkSize) {
        currentChunk += cleanParagraph + "\n\n";
      } else {
        if (currentChunk) {
          chunks.push(currentChunk.trim());
        }
        currentChunk = cleanParagraph + "\n\n";
      }
    }
  }

  // 保存最后的块
  if (currentChunk) {
    chunks.push(currentChunk.trim());
  }

  // 过滤空字符串并设置结果
  cards.value = chunks.filter(chunk => chunk.length > 0);
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