<script setup lang="ts">
import {computed} from 'vue'

const props = defineProps<{
  title: string
  content: string
}>()

// 便签颜色选项
const noteColors = [
  '#FFE5B4', // 浅橙色
  '#E0FFFF', // 浅青色
  '#FFB6C1', // 浅粉色
  '#E6E6FA', // 薰衣草色
  '#FFFACD', // 柠檬绸色
  '#F0FFF0'  // 蜂蜜露色
]

// 随机选择颜色
const randomColor = computed(() => {
  return noteColors[Math.floor(Math.random() * noteColors.length)]
})
</script>

<template>
  <div
      class="sticky-note"
      :style="{ backgroundColor: randomColor }"
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
  min-height: 400px;
  padding: 24px;
  margin: 16px;
  border-radius: 12px;
  box-shadow: 
    0 4px 12px rgba(0, 0, 0, 0.08),
    0 1px 3px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
  overflow: hidden;
}

.sticky-note::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 8px;
  background: linear-gradient(90deg, 
    rgba(255, 255, 255, 0.3) 0%, 
    rgba(255, 255, 255, 0.1) 50%, 
    rgba(255, 255, 255, 0.3) 100%);
  border-radius: 12px 12px 0 0;
}

.sticky-note:hover {
  transform: translateY(-2px);
  box-shadow: 
    0 8px 24px rgba(0, 0, 0, 0.12),
    0 2px 6px rgba(0, 0, 0, 0.08);
}

.sticky-note:active {
  transform: translateY(0);
  transition: transform 0.1s ease;
}

.note-content {
  position: relative;
  z-index: 2;
}

.note-title {
  font-size: 20px;
  font-weight: 600;
  color: rgba(0, 0, 0, 0.85);
  text-align: left;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  letter-spacing: -0.2px;
  line-height: 1.3;
}

.note-body {
  font-size: 16px;
  line-height: 1.5;
  color: rgba(0, 0, 0, 0.75);
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
  min-height: 200px;
  letter-spacing: -0.1px;
}

/* iOS风格的纸张纹理效果 */
.sticky-note::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    radial-gradient(circle at 10% 20%, rgba(255, 255, 255, 0.03) 0%, transparent 15%),
    radial-gradient(circle at 90% 80%, rgba(255, 255, 255, 0.02) 0%, transparent 15%);
  pointer-events: none;
  border-radius: 12px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sticky-note {
    margin: 12px;
    padding: 20px;
    min-height: 350px;
  }

  .note-title {
    font-size: 18px;
    margin-bottom: 16px;
  }

  .note-body {
    font-size: 15px;
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
    font-size: 16px;
    margin-bottom: 12px;
  }

  .note-body {
    font-size: 14px;
    min-height: 120px;
  }
}

/* 不同颜色主题的优化 */
.sticky-note[style*="#FFE5B4"] {
  background-color: #FFE5B4;
}

.sticky-note[style*="#FFE5B4"] .note-title {
  color: rgba(139, 69, 19, 0.9);
  border-bottom-color: rgba(139, 69, 19, 0.1);
}

.sticky-note[style*="#FFE5B4"] .note-body {
  color: rgba(101, 67, 33, 0.85);
}

.sticky-note[style*="#E0FFFF"] {
  background-color: #E0FFFF;
}

.sticky-note[style*="#E0FFFF"] .note-title {
  color: rgba(0, 100, 0, 0.9);
  border-bottom-color: rgba(0, 100, 0, 0.1);
}

.sticky-note[style*="#E0FFFF"] .note-body {
  color: rgba(0, 128, 128, 0.85);
}

.sticky-note[style*="#FFB6C1"] {
  background-color: #FFB6C1;
}

.sticky-note[style*="#FFB6C1"] .note-title {
  color: rgba(199, 21, 133, 0.9);
  border-bottom-color: rgba(199, 21, 133, 0.1);
}

.sticky-note[style*="#FFB6C1"] .note-body {
  color: rgba(219, 112, 147, 0.85);
}

.sticky-note[style*="#E6E6FA"] {
  background-color: #E6E6FA;
}

.sticky-note[style*="#E6E6FA"] .note-title {
  color: rgba(75, 0, 130, 0.9);
  border-bottom-color: rgba(75, 0, 130, 0.1);
}

.sticky-note[style*="#E6E6FA"] .note-body {
  color: rgba(106, 90, 205, 0.85);
}

.sticky-note[style*="#FFFACD"] {
  background-color: #FFFACD;
}

.sticky-note[style*="#FFFACD"] .note-title {
  color: rgba(184, 134, 11, 0.9);
  border-bottom-color: rgba(184, 134, 11, 0.1);
}

.sticky-note[style*="#FFFACD"] .note-body {
  color: rgba(218, 165, 32, 0.85);
}

.sticky-note[style*="#F0FFF0"] {
  background-color: #F0FFF0;
}

.sticky-note[style*="#F0FFF0"] .note-title {
  color: rgba(34, 139, 34, 0.9);
  border-bottom-color: rgba(34, 139, 34, 0.1);
}

.sticky-note[style*="#F0FFF0"] .note-body {
  color: rgba(50, 205, 50, 0.85);
}
</style>