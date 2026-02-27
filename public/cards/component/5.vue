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

// 复古棕色主色调
const noteColor = '#5d4037'
</script>

<template>
  <div
      class="sticky-note"
      :style="{ fontFamily: props.font.font_family }"
  >
    <!-- 顶部复古边框 -->
    <div class="vintage-border"></div>

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

    <!-- 底部复古花纹 -->
    <div class="vintage-pattern"></div>
  </div>
</template>

<style scoped>
.sticky-note {
  position: relative;
  min-height: 100px;
  padding: 24px;
  margin: 0;
  background: linear-gradient(135deg, #efebe9 0%, #d7ccc8 100%);
  border: 2px solid #a1887f;
  border-radius: 16px;
  box-shadow: 
    0 3px 10px rgba(93, 64, 55, 0.2),
    inset 0 0 15px rgba(93, 64, 55, 0.05);
  cursor: pointer;
  font-family: 'Times New Roman', 'Georgia', serif;
  overflow: hidden;
  backdrop-filter: blur(3px);
}

.vintage-border {
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  height: 12px;
  background: linear-gradient(90deg, 
    #3e2723 0%, 
    #5d4037 20%, 
    #6d4c41 50%, 
    #5d4037 80%, 
    #3e2723 100%);
  border-radius: 6px 6px 0 0;
  background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 20"><rect x="0" y="8" width="100" height="4" fill="%238d6e63" opacity="0.3"/><path d="M0 10 L20 8 L40 12 L60 8 L80 12 L100 10" stroke="%23a1887f" stroke-width="1" fill="none" opacity="0.4"/></svg>');
  background-size: 100% 100%;
  background-repeat: no-repeat;
}

.sticky-note:hover {
  border-color: #8d6e63;
  box-shadow: 
    0 5px 15px rgba(93, 64, 55, 0.25),
    inset 0 0 20px rgba(93, 64, 55, 0.08);
}

.sticky-note:active {
  /* 移除点击动画 */
}

.note-content {
  position: relative;
  z-index: 2;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 16px;
  padding: 16px;
  border: 1px solid rgba(161, 136, 127, 0.4);
}

.note-title {
  font-size: 18px;
  font-weight: bold;
  color: #3e2723;
  text-align: left;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 2px solid rgba(161, 136, 127, 0.6);
  line-height: 1.4;
  letter-spacing: 0.5px;
  font-variant: small-caps;
}

.note-body {
  font-size: 15px;
  line-height: 1.6;
  color: #4e342e;
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
  min-height: 50px;
  font-feature-settings: 'liga' 1, 'kern' 1;
}

/* 复古纸张纹理 */
.sticky-note::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    linear-gradient(45deg, rgba(93, 64, 55, 0.03) 25%, transparent 25%),
    linear-gradient(-45deg, rgba(93, 64, 55, 0.03) 25%, transparent 25%),
    linear-gradient(45deg, transparent 75%, rgba(93, 64, 55, 0.02) 75%),
    linear-gradient(-45deg, transparent 75%, rgba(93, 64, 55, 0.02) 75%);
  background-size: 20px 20px;
  background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
  pointer-events: none;
  border-radius: 6px;
  opacity: 0.6;
  z-index: 1;
}

.vintage-pattern {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 16px;
  background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 20"><path d="M0 15 Q25 5 50 15 T100 15" stroke="%238d6e63" stroke-width="1" fill="none" opacity="0.4"/><circle cx="15" cy="10" r="2" fill="%23a1887f" opacity="0.3"/><circle cx="35" cy="12" r="1.5" fill="%238d6e63" opacity="0.2"/><circle cx="65" cy="8" r="1.8" fill="%23795548" opacity="0.3"/><circle cx="85" cy="11" r="1.2" fill="%236d4c41" opacity="0.25"/></svg>');
  background-size: 100% 100%;
  background-repeat: no-repeat;
  border-top: 1px solid rgba(161, 136, 127, 0.3);
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
  
  .vintage-border {
    height: 10px;
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
  
  .vintage-border {
    height: 8px;
  }
}
</style>