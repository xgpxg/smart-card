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

// 紫色梦幻色调
const noteColor = '#7b1fa2'
</script>

<template>
  <div
      class="sticky-note"
      :style="{ fontFamily: props.font.font_family }"
  >
    <!-- 顶部星云效果 -->
    <div class="nebula-top"></div>

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

    <!-- 底部星光点缀 -->
    <div class="star-dots"></div>
  </div>
</template>

<style scoped>
.sticky-note {
  position: relative;
  min-height: 100px;
  padding: 24px;
  margin: 0;
  background: linear-gradient(135deg, #f3e5f5 0%, #e1bee7 50%, #ce93d8 100%);
  border: 1px solid #ba68c8;
  border-radius: 16px;
  box-shadow: 0 0 25px rgba(156, 39, 176, 0.2),
  inset 0 0 30px rgba(156, 39, 176, 0.1);
  cursor: pointer;
  font-family: 'Pacifico', 'Dancing Script', cursive;
  overflow: hidden;
  backdrop-filter: blur(8px);
}

.nebula-top {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 8px;
  border-radius: 16px 16px 0 0;
  background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 20"><circle cx="20" cy="10" r="3" fill="%23e1bee7" opacity="0.6"/><circle cx="50" cy="8" r="2" fill="%23ce93d8" opacity="0.4"/><circle cx="80" cy="12" r="2.5" fill="%23ba68c8" opacity="0.5"/></svg>');
  background-size: 100% 100%;
  background-repeat: no-repeat;
}


.sticky-note:active {
  /* 移除点击动画 */
}

.note-content {
  position: relative;
  z-index: 2;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid rgba(186, 104, 200, 0.3);
}

.note-title {
  font-size: 18px;
  font-weight: 600;
  color: #4a148c;
  text-align: left;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(186, 104, 200, 0.4);
  line-height: 1.4;
  letter-spacing: 0.5px;
  text-shadow: 0 1px 2px rgba(156, 39, 176, 0.1);
}

.note-body {
  font-size: 15px;
  line-height: 1.6;
  color: #6a1b9a;
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
  min-height: 50px;
  font-feature-settings: 'liga' 1;
}

/* 星空背景纹理 */
.sticky-note::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: radial-gradient(circle at 15% 25%, rgba(156, 39, 176, 0.15) 1px, transparent 1px),
  radial-gradient(circle at 85% 15%, rgba(106, 27, 154, 0.12) 1.5px, transparent 1.5px),
  radial-gradient(circle at 45% 75%, rgba(142, 36, 170, 0.1) 1px, transparent 1px),
  radial-gradient(circle at 75% 65%, rgba(171, 71, 188, 0.08) 2px, transparent 2px);
  background-size: 50px 50px, 70px 70px, 60px 60px, 80px 80px;
  pointer-events: none;
  border-radius: 16px;
  opacity: 0.5;
  z-index: 1;
}

.star-dots {
  position: absolute;
  bottom: 8px;
  left: 0;
  right: 0;
  height: 4px;
  background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 10"><circle cx="10" cy="5" r="1" fill="%23ba68c8" opacity="0.6"/><circle cx="30" cy="3" r="0.8" fill="%23ab47bc" opacity="0.4"/><circle cx="50" cy="6" r="1.2" fill="%238e24aa" opacity="0.5"/><circle cx="70" cy="4" r="0.9" fill="%23ce93d8" opacity="0.3"/><circle cx="90" cy="5" r="1.1" fill="%23e1bee7" opacity="0.4"/></svg>');
  background-size: 100% 100%;
  background-repeat: no-repeat;
  opacity: 0.7;
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

  .nebula-top {
    height: 6px;
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

  .nebula-top {
    height: 5px;
  }
}
</style>