<script setup lang="ts">
import {reactive, ref} from "vue";
import {ElImageViewer} from 'element-plus';
import {ZoomIn} from '@element-plus/icons-vue';
import {Card} from "./card.ts";

const value = defineModel<Card>( {
  default: () => reactive(new Card('', ''))
});


const proportion = ref(null);
const previewVisible = ref(false);
const previewUrl = ref('');

// 图片数据示例
const cardStyles = [
  {
    id: '1',
    name: '简约风',
    thumbnail: '/card-styles/简约风.png',
    preview: '/card-styles/简约风.png'
  },
  {
    id: '2',
    name: '卡通风',
    thumbnail: '/card-styles/卡通风.png',
    preview: '/card-styles/卡通风.png'
  },
  {
    id: '3',
    name: '商务风',
    thumbnail: '/card-styles/商务风.png',
    preview: '/card-styles/商务风.png'
  },
  {
    id: '4',
    name: '科技风',
    thumbnail: '/images/style4-thumb.jpg',
    preview: '/images/style4-preview.jpg'
  },
  {
    id: '5',
    name: '复古风',
    thumbnail: '/images/style5-thumb.jpg',
    preview: '/images/style5-preview.jpg'
  }
];

const handleStyleClick = (styleItem: any) => {
  previewUrl.value = styleItem.preview;
  previewVisible.value = true;
};

</script>

<template>
  <div>
    <div class="fill-width">
      <el-input v-model="value.content" type="textarea" :rows="10" placeholder="文本内容"></el-input>
    </div>
    <el-form label-width="40px" class="mt10">
      <el-form-item label="比例">
        <el-radio-group v-model="proportion">
          <el-radio-button label="1">1 : 1</el-radio-button>
          <el-radio-button label="2">4 : 3</el-radio-button>
          <el-radio-button label="4">3 : 4</el-radio-button>
          <el-radio-button label="3">16 : 9</el-radio-button>
          <el-radio-button label="5">9 : 16</el-radio-button>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="样式">
        <el-input placeholder="搜索样式" suffix-icon="search"></el-input>
        <div class="style-gallery mt5">
          <div
              v-for="styleItem in cardStyles"
              :key="styleItem.id"
              class="style-item"
              @click="handleStyleClick(styleItem)"
          >
            <div class="style-thumbnail">
              <img :src="styleItem.thumbnail" :alt="styleItem.name"/>
              <div class="style-overlay">
                <span class="style-name">{{ styleItem.name }}</span>
                <el-icon class="zoom-icon">
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
        <el-radio-group>
          <el-radio-button label="1">自动</el-radio-button>
          <el-radio-button label="2">单页</el-radio-button>
          <el-radio-button label="2">按字数拆分</el-radio-button>
          <el-radio-button label="2">分隔符</el-radio-button>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="字体">
        <el-select v-model="value.font" :teleported="false">
          <el-option label="1" value="1">自动</el-option>
          <el-option label="2" value="2">单张</el-option>
          <el-option label="3" value="3">按字数拆分</el-option>
          <el-option label="4" value="4">自定义</el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="">
        <el-button type="primary" class="fill-width">生成卡片</el-button>
      </el-form-item>
    </el-form>
  </div>
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
</style>