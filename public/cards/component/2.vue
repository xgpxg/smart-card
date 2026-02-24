<script setup lang="ts">
import {computed} from 'vue'

const props = defineProps<{
  title: string
  content: string,
  font: {
    type: Object,
    default: {
      font_family: 'Arial'
    }
  }
}>()

// 科技蓝主色调
const noteColor = '#1a73e8'
</script>

<template>
  <div
      class="sticky-note"
      :style="{ fontFamily: props.font.font_family }"
  >
    <!-- 顶部科技线条 -->
    <div class="tech-line"></div>

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

    <!-- 底部科技光效 -->
    <div class="tech-glow"></div>
  </div>
</template>

<style scoped>
.sticky-note {
  position: relative;
  min-height: 100px;
  padding: 24px;
  margin: 16px;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  border: 1px solid #334155;
  border-radius: 12px;
  box-shadow: 
    0 0 20px rgba(26, 115, 232, 0.3),
    inset 0 0 30px rgba(26, 115, 232, 0.1);
  cursor: pointer;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  overflow: hidden;
  backdrop-filter: blur(10px);
}

.tech-line {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, #1a73e8 0%, #8ab4f8 50%, #1a73e8 100%);
  border-radius: 12px 12px 0 0;
  box-shadow: 0 0 10px rgba(26, 115, 232, 0.8);
}

.sticky-note:hover {
  border-color: #8ab4f8;
  box-shadow: 
    0 0 30px rgba(26, 115, 232, 0.5),
    inset 0 0 40px rgba(26, 115, 232, 0.2);
}

.sticky-note:active {
  /* 移除点击动画 */
}

.note-content {
  position: relative;
  z-index: 2;
  background: rgba(15, 23, 42, 0.7);
  border-radius: 8px;
  padding: 16px;
  border: 1px solid rgba(138, 180, 248, 0.2);
}

.note-title {
  font-size: 18px;
  font-weight: 600;
  color: #e2e8f0;
  text-align: left;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(138, 180, 248, 0.3);
  line-height: 1.4;
  text-shadow: 0 0 5px rgba(26, 115, 232, 0.5);
  letter-spacing: 0.5px;
}

.note-body {
  font-size: 15px;
  line-height: 1.6;
  color: #cbd5e1;
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
  min-height: 50px;
  font-feature-settings: 'zero' 1;
}

/* 科技网格背景 */
.sticky-note::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    linear-gradient(rgba(26, 115, 232, 0.1) 1px, transparent 1px),
    linear-gradient(90deg, rgba(26, 115, 232, 0.1) 1px, transparent 1px);
  background-size: 20px 20px;
  pointer-events: none;
  border-radius: 12px;
  opacity: 0.3;
  z-index: 1;
}

.tech-glow {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, transparent 0%, #1a73e8 50%, transparent 100%);
  border-radius: 0 0 12px 12px;
  box-shadow: 0 0 15px rgba(26, 115, 232, 0.6);
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