<script setup lang="ts">
import { computed } from 'vue'

interface FontConfig {
  font_family: string
}

const props = defineProps<{
  title: string
  content: string,
  font: FontConfig
}>()

// 森林绿主色调
const noteColor = '#2e7d32'
</script>

<template>
  <div
      class="sticky-note"
      :style="{ fontFamily: props.font.font_family }"
  >
    <!-- 顶部叶子装饰 -->
    <div class="leaf-decoration"></div>

    <!-- 便签内容 -->
    <div class="note-content">
      <!-- 标题 -->
      <div class="note-title">
        {{ title }}
      </div>

      <!-- 内容 -->
      <div class="note-body">
        {{ content }}
      </div>
    </div>

    <!-- 底部自然纹理 -->
    <div class="natural-texture"></div>
  </div>
</template>

<style scoped>
.sticky-note {
  position: relative;
  min-height: 100px;
  padding: 24px;
  margin: 0;
  background: linear-gradient(135deg, #e8f5e8 0%, #c8e6c9 100%);
  border: 1px solid #a5d6a7;
  border-radius: 16px;
  box-shadow: 
    0 2px 12px rgba(46, 125, 50, 0.15),
    inset 0 0 20px rgba(46, 125, 50, 0.05);
  cursor: pointer;
  font-family: 'Georgia', 'Times New Roman', serif;
  overflow: hidden;
  backdrop-filter: blur(5px);
}

.leaf-decoration {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 60px;
  height: 60px;
  background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><path d="M50 10 Q70 25 80 50 Q70 75 50 90 Q30 75 20 50 Q30 25 50 10 Z" fill="%234caf50" opacity="0.3"/><path d="M45 15 Q60 30 70 45 Q60 60 45 75 Q30 60 20 45 Q30 30 45 15 Z" fill="%2381c784" opacity="0.5"/></svg>');
  background-size: contain;
  background-repeat: no-repeat;
  transform: rotate(15deg);
  z-index: 3;
}

.sticky-note:hover {
  border-color: #81c784;
  box-shadow: 
    0 4px 16px rgba(46, 125, 50, 0.2),
    inset 0 0 25px rgba(46, 125, 50, 0.08);
}

.sticky-note:active {
  /* 移除点击动画 */
}

.note-content {
  position: relative;
  z-index: 2;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid rgba(129, 199, 132, 0.3);
}

.note-title {
  font-size: 18px;
  font-weight: 600;
  color: #1b5e20;
  text-align: left;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(129, 199, 132, 0.4);
  line-height: 1.4;
  letter-spacing: 0.3px;
}

.note-body {
  font-size: 15px;
  line-height: 1.6;
  color: #2e7d32;
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
  min-height: 50px;
  font-feature-settings: 'liga' 1;
}

/* 自然纹理背景 */
.sticky-note::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    radial-gradient(circle at 20% 30%, rgba(76, 175, 80, 0.1) 2px, transparent 2px),
    radial-gradient(circle at 80% 70%, rgba(129, 199, 132, 0.1) 3px, transparent 3px),
    radial-gradient(circle at 40% 80%, rgba(102, 187, 106, 0.08) 2px, transparent 2px);
  background-size: 60px 60px, 80px 80px, 40px 40px;
  pointer-events: none;
  border-radius: 16px;
  opacity: 0.4;
  z-index: 1;
}

.natural-texture {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 6px;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(76, 175, 80, 0.3) 20%, 
    rgba(129, 199, 132, 0.4) 50%, 
    rgba(76, 175, 80, 0.3) 80%, 
    transparent 100%);
  border-radius: 0 0 12px 12px;
  opacity: 0.6;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sticky-note {
    margin: 12px;
    padding: 20px;
    min-height: 350px;
  }

  .note-title {
    font-size: 16px;
    margin-bottom: 14px;
  }

  .note-body {
    font-size: 14px;
    min-height: 150px;
  }
  
  .leaf-decoration {
    width: 50px;
    height: 50px;
    top: -6px;
    right: -6px;
  }
}

@media (max-width: 480px) {
  .sticky-note {
    margin: 8px;
    padding: 16px;
    min-height: 300px;
  }

  .note-title {
    font-size: 15px;
    margin-bottom: 12px;
  }

  .note-body {
    font-size: 13px;
    min-height: 120px;
  }
  
  .leaf-decoration {
    width: 40px;
    height: 40px;
    top: -4px;
    right: -4px;
  }
}
</style>