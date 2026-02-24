<script setup lang="ts">
import { computed } from 'vue'

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

// 深空蓝主色调
const primaryColor = '#0f172a'
const accentColor = '#3b82f6'
const textColor = '#f1f5f9'
const borderColor = '#334155'
</script>

<template>
  <div
      class="sticky-note"
      :style="{ fontFamily: props.font.font_family }"
  >
    <!-- 左侧装饰线 -->
    <div class="accent-line"></div>

    <!-- 主要内容区域 -->
    <div class="note-container">
      <!-- 标题区域 -->
      <div class="note-header">
        <h3 class="note-title">{{ title }}</h3>
      </div>

      <!-- 内容区域 -->
      <div class="note-content">
        <p class="note-body">{{ content }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sticky-note {
  position: relative;
  display: flex;
  min-height: 200px;
  margin: 16px;
  background: #0f172a;
  border: 1px solid #334155;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
  cursor: pointer;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  overflow: hidden;
}


.note-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 24px;
}

.note-header {
  margin-bottom: 20px;
}

.note-title {
  font-size: 20px;
  font-weight: 600;
  color: #f1f5f9;
  margin: 0;
  line-height: 1.3;
  letter-spacing: -0.02em;
}

.note-content {
  flex: 1;
}

.note-body {
  font-size: 16px;
  line-height: 1.6;
  color: #cbd5e1;
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sticky-note {
    margin: 12px;
    min-height: 180px;
  }

  .note-container {
    padding: 20px;
  }

  .note-title {
    font-size: 18px;
  }

  .note-body {
    font-size: 15px;
  }
}

@media (max-width: 480px) {
  .sticky-note {
    margin: 8px;
    min-height: 160px;
  }

  .note-container {
    padding: 16px;
  }

  .note-title {
    font-size: 16px;
  }

  .note-body {
    font-size: 14px;
  }
}
</style>