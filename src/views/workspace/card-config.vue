<script setup lang="ts">
import {computed, inject, onMounted, ref, watch} from "vue";
import {ElImageViewer, ElMessage} from 'element-plus';
import {ZoomIn} from '@element-plus/icons-vue';
import {call, convertAudioSrc} from "@/utils/commands.ts";
import TextCard from "@/views/workspace/text-card.vue";

// 工作空间，包含所有配置
const workspace = inject<any>('workspace')
const charCount = ref(workspace.value.pagination.char_count)
// 转换音频地址，用于播放
const audio_url = computed(() => {
  if (!workspace.value.file_path) {
    return null
  }
  return convertAudioSrc(workspace.value.file_path)
})

// 开始音频转文字
const startAudioToText = async () => {
  workspace.value.trans_text_status = 'Processing'
  await call('start_audio_to_text', {
    id: workspace.value.id
  })
  PubSub.publish('workspace/reload', {id: workspace.value.id})
}


const previewVisible = ref(false);
const previewUrl = ref('');

// 图片数据示例
const cardStyles = [
  {
    id: 1,
    name: '简约风',
    thumbnail: '/card-styles/简约风.png',
    preview: '/card-styles/简约风.png'
  },
  {
    id: 2,
    name: '卡通风',
    thumbnail: '/card-styles/卡通风.png',
    preview: '/card-styles/卡通风.png'
  },
  {
    id: 3,
    name: '商务风',
    thumbnail: '/card-styles/商务风.png',
    preview: '/card-styles/商务风.png'
  },
];

const fonts = ref([])
const loadFonts = async () => {
  if ("queryLocalFonts" in window) {
    try {
      const availableFonts = await (window as any).queryLocalFonts();
      if (!availableFonts.length) {
        return [];
      }

      fonts.value = availableFonts.map((font: any) => {
        return font.fullName
      });
    } catch (err) {
      return Promise.reject(err);
    }
  } else {
    return Promise.reject("浏览器版本太低 or 网站不安全");
  }
}
onMounted(() => {
  loadFonts()
})
const handleStyleClick = (styleItem: any) => {
  workspace.value.style_id = styleItem.id
  PubSub.publish('workspace/style/change')
}

const previewImage = (styleItem: any) => {
  previewUrl.value = styleItem.preview;
  previewVisible.value = true;
};

// 处理 textarea 中的 Tab 键输入
const handleTextareaKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Tab') {
    event.preventDefault();

    const target = event.target as HTMLTextAreaElement;
    const start = target.selectionStart;
    const end = target.selectionEnd;

    // 在光标位置插入制表符
    const value = target.value;
    target.value = value.substring(0, start) + '\t' + value.substring(end);

    // 更新光标位置
    const newCursorPos = start + 1;
    target.selectionStart = newCursorPos;
    target.selectionEnd = newCursorPos;

    // 触发 Vue 的响应式更新
    workspace.value.trans_text = target.value;
  }
};

watch(charCount, (newValue) => {
  const n = Number(newValue)
  if (n && n > 50) {
    workspace.value.pagination.char_count = n
  }
})


const exampleTitle = '示例标题'
const exampleContent = '人工智能正在深刻改变我们的生活。从智能语音助手到自动驾驶汽车，AI技术已经渗透到各个领域。机器学习算法能够分析海量数据，识别人类难以察觉的模式，为医疗诊断、金融预测和科学研究提供强大支持。深度学习网络模拟人脑神经元结构，在图像识别、自然语言处理等方面表现出色。AI不仅提高了工作效率，还创造了全新的商业模式和服务体验。然而，随着AI快速发展，数据隐私、算法偏见和就业冲击等挑战也日益凸显。未来，我们需要在推动技术创新的同时，建立完善的伦理规范和监管框架，确保AI发展真正造福人类社会，实现科技与人文的和谐统一。'
</script>

<template>
  <div class="flex">
    <audio :key="audio_url"
           controls controlslist="nodownload noplaybackrate"
           class="fill-width"
           style="height: 32px">
      <source :src="audio_url" type="audio/mp3">
      无法播放此音频。
    </audio>
    <el-button @click="startAudioToText" type="primary" class="ml5"
               :loading="workspace.trans_text_status==='Processing'">
      <template v-if="workspace.trans_text_status==='Processing'">
        处理中
      </template>
      <template v-else>
        转文字
      </template>

    </el-button>
  </div>
  <div class="fill-width mt10">
    <el-input v-model="workspace.trans_text"
              type="textarea"
              :rows="8"
              placeholder="文本内容"
              @keydown="handleTextareaKeydown"></el-input>
  </div>
  <el-form label-width="40px" class="mt20">
    <el-form-item label="样式">
      <el-input placeholder="搜索样式" suffix-icon="search"></el-input>
      <div class="style-gallery mt5">
        <div
            v-for="item in cardStyles"
            :key="item.id"
            class="style-item"
            @click="handleStyleClick(item)"
        >
          <div class="style-thumbnail">
            <text-card
                :url="`/cards/component/${item.id}.vue`"
                :title="exampleTitle"
                :content="exampleContent"
                :font="workspace.font"
                style="zoom:0.9;"
            ></text-card>
<!--            <img :src="item.thumbnail" :alt="item.name"/>-->
            <div class="style-overlay">
              <span class="style-name">{{ item.name }}</span>
              <el-icon class="zoom-icon" @click="previewImage(item)">
                <ZoomIn/>
              </el-icon>
            </div>
          </div>
        </div>
      </div>

      <!-- 图片预览组件 -->
      <el-image-viewer
          v-if="previewVisible"
          :url-list="[previewUrl]"
          @close="previewVisible = false"
      />
    </el-form-item>
    <el-form-item label="分页">
      <el-radio-group v-model="workspace.pagination.page_type">
        <el-radio-button label="Auto" value="Auto">自动</el-radio-button>
        <el-radio-button label="Single" value="Single">单页</el-radio-button>
        <el-radio-button label="Delimiter" value="Delimiter" @click="workspace.pagination.delimiter='\\n'">换行符
        </el-radio-button>
        <el-radio-button label="CharCount" value="CharCount">按字数拆分
        </el-radio-button>
      </el-radio-group>
      <el-input v-if="workspace.pagination.page_type ==='CharCount'"
                v-model="charCount"
                placeholder="字数" class="mt5" minlength="1" maxlength="10"></el-input>
      <!--      <el-input v-if="workspace.pagination.page_type ==='Delimiter'"
                      v-model="workspace.pagination.delimiter"
                      placeholder="分隔符" class="mt5" maxlength="8"></el-input>-->
    </el-form-item>
    <el-form-item label="字体">
      <el-select v-model="workspace.font.font_family" :teleported="false" filterable>
        <el-option v-for="item in fonts" :label="item" :value="item">
          {{ item }}
        </el-option>
      </el-select>
    </el-form-item>
    <el-form-item label="">
      <el-button type="primary" class="fill-width">生成卡片</el-button>
    </el-form-item>
  </el-form>
</template>

<style scoped lang="scss">
.style-gallery {
  display: flex;
  gap: 16px;
  overflow-x: auto;
  padding: 8px 0;
  scrollbar-width: thin;
  scrollbar-color: #cccccc transparent;

  &::-webkit-scrollbar {
    height: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background-color: #cccccc;
    border-radius: 3px;
  }
}

.style-item {
  flex: 0 0 auto;
  cursor: pointer;
  transition: transform 0.2s ease;
  border-radius: 8px;
  overflow: hidden;

  &:hover {
    transform: translateY(-4px);
  }
}

.style-thumbnail {
  position: relative;
  height: 120px;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease;
  }

  &:hover img {
    transform: scale(1.05);
  }
}

.style-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
  padding: 20px 12px 12px;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.style-name {
  font-size: 12px;
  font-weight: 500;
}

.zoom-icon {
  font-size: 14px;
  opacity: 0.8;
}

/* 滚动条美化 */
.style-gallery::-webkit-scrollbar {
  height: 8px;
}

.style-gallery::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.style-gallery::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;

  &:hover {
    background: #a8a8a8;
  }
}

audio {
  filter: invert(0.8) hue-rotate(180deg);

  &::-webkit-media-controls-enclosure {
    border-radius: 5px;
  }
}

</style>