<script setup lang="ts">
import {computed} from 'vue'

interface FontConfig {
  font_family: string
}

const props = defineProps<{
  title: string
  content: string,
  font: FontConfig
}>()

</script>

<template>
  <div
      class="sticky-note"
      :style="{ fontFamily: props.font.font_family }"
  >
    <!-- 顶部胶带效果 -->
    <div class="tape"></div>

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

    <!-- 底部阴影 -->
    <div class="note-shadow"></div>
  </div>
</template>

<style scoped>
.sticky-note {
  position: relative;
  min-height: 100px;
  padding: 24px;
  margin: 0;
  background-color: #FFF9C4;
  border: 1px solid #FFF59D;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  cursor: pointer;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  overflow: hidden;
}

.sticky-note::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background-color: #FFEB3B;
  border-radius: 8px 8px 0 0;
}

.sticky-note:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  border-color: #FFF176;
}

.note-content {
  position: relative;
  z-index: 2;
}

.note-title {
  font-size: 18px;
  font-weight: 500;
  color: #5D4037;
  text-align: left;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid #FFF59D;
  line-height: 1.4;
}

.note-body {
  font-size: 15px;
  line-height: 1.6;
  color: #4E342E;
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
  min-height: 50px;
}

/* 简洁的纹理效果 */
.sticky-note::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: linear-gradient(transparent 50%, rgba(255, 255, 255, 0.3) 50%);
  background-size: 100% 24px;
  pointer-events: none;
  border-radius: 8px;
  opacity: 0.1;
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
}


</style>