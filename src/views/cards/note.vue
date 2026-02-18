<script setup lang="ts">
import { ref, computed } from 'vue'

interface NoteCardProps {
  title?: string
  content?: string
  author?: string
  date?: string
  color?: string
  editable?: boolean
}

const props = withDefaults(defineProps<NoteCardProps>(), {
  title: '',
  content: '',
  author: '',
  date: '',
  color: '#FFE5B4',
  editable: false
})

const emit = defineEmits(['update:title', 'update:content', 'update:author'])

const isEditing = ref(false)
const localTitle = ref(props.title)
const localContent = ref(props.content)
const localAuthor = ref(props.author)

const noteColors = [
  '#FFE5B4', // 浅橙色
  '#E0FFFF', // 浅青色
  '#FFB6C1', // 浅粉色
  '#E6E6FA', // 薰衣草色
  '#FFFACD', // 柠檬绸色
  '#F0FFF0'  // 蜂蜜露色
]

const randomColor = computed(() => {
  return noteColors[Math.floor(Math.random() * noteColors.length)]
})

const currentColor = computed(() => {
  return props.color || randomColor.value
})

const toggleEdit = () => {
  if (!props.editable) return
  
  if (isEditing.value) {
    // 保存编辑
    emit('update:title', localTitle.value)
    emit('update:content', localContent.value)
    emit('update:author', localAuthor.value)
  } else {
    // 开始编辑
    localTitle.value = props.title
    localContent.value = props.content
    localAuthor.value = props.author
  }
  isEditing.value = !isEditing.value
}

const cancelEdit = () => {
  isEditing.value = false
  localTitle.value = props.title
  localContent.value = props.content
  localAuthor.value = props.author
}
</script>

<template>
  <div 
    class="note-card" 
    :style="{ backgroundColor: currentColor }"
    @dblclick="toggleEdit"
  >
    <!-- 便签顶部的胶带效果 -->
    <div class="tape"></div>
    
    <!-- 便签主体内容 -->
    <div class="note-content">
      <!-- 标题区域 -->
      <div class="note-header">
        <div v-if="!isEditing && title" class="note-title">
          {{ title }}
        </div>
        <el-input
          v-else-if="editable"
          v-model="localTitle"
          placeholder="请输入标题"
          class="title-input"
          size="small"
        />
      </div>
      
      <!-- 内容区域 -->
      <div class="note-body">
        <div v-if="!isEditing && content" class="note-text">
          {{ content }}
        </div>
        <el-input
          v-else-if="editable"
          v-model="localContent"
          type="textarea"
          placeholder="请输入内容"
          class="content-input"
          :autosize="{ minRows: 3, maxRows: 6 }"
        />
      </div>
      
      <!-- 底部信息 -->
      <div class="note-footer">
        <div class="note-meta">
          <div v-if="!isEditing && (author || date)" class="meta-info">
            <span v-if="author" class="author">{{ author }}</span>
            <span v-if="author && date" class="separator">•</span>
            <span v-if="date" class="date">{{ date }}</span>
          </div>
          <div v-else-if="editable" class="meta-edit">
            <el-input
              v-model="localAuthor"
              placeholder="署名"
              class="author-input"
              size="small"
            />
          </div>
        </div>
        
        <!-- 编辑按钮 -->
        <div v-if="editable" class="edit-actions">
          <el-button 
            v-if="!isEditing" 
            type="primary" 
            size="small" 
            @click.stop="toggleEdit"
            class="edit-btn"
          >
            编辑
          </el-button>
          <div v-else class="edit-buttons">
            <el-button size="small" @click.stop="cancelEdit">取消</el-button>
            <el-button type="primary" size="small" @click.stop="toggleEdit">保存</el-button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 便签阴影效果 -->
    <div class="note-shadow"></div>
  </div>
</template>

<style scoped lang="scss">
.note-card {
  position: relative;
  min-height: 200px;
  padding: 20px;
  margin: 20px;
  border-radius: 8px 8px 2px 2px;
  box-shadow: 
    0 4px 8px rgba(0, 0, 0, 0.1),
    0 2px 4px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  cursor: pointer;
  font-family: 'Comic Sans MS', 'Arial Rounded MT Bold', cursive, sans-serif;
  
  &:hover {
    transform: translateY(-2px);
    box-shadow: 
      0 6px 12px rgba(0, 0, 0, 0.15),
      0 4px 6px rgba(0, 0, 0, 0.15);
  }
  
  &:active {
    transform: translateY(0);
  }
}

// 顶部胶带效果
.tape {
  position: absolute;
  top: -10px;
  left: 50%;
  transform: translateX(-50%);
  width: 60px;
  height: 20px;
  background: linear-gradient(45deg, #f8f8f8 25%, transparent 25%, transparent 75%, #f8f8f8 75%),
              linear-gradient(45deg, #f8f8f8 25%, transparent 25%, transparent 75%, #f8f8f8 75%);
  background-size: 4px 4px;
  background-position: 0 0, 2px 2px;
  border-radius: 4px;
  box-shadow: 
    inset 0 1px 2px rgba(0, 0, 0, 0.1),
    0 1px 2px rgba(0, 0, 0, 0.1);
}

.note-content {
  position: relative;
  z-index: 2;
}

.note-header {
  margin-bottom: 15px;
  border-bottom: 2px dashed rgba(0, 0, 0, 0.1);
  padding-bottom: 10px;
}

.note-title {
  font-size: 18px;
  font-weight: bold;
  color: #333;
  text-align: center;
  margin-bottom: 5px;
  letter-spacing: 1px;
}

.title-input {
  :deep(.el-input__inner) {
    text-align: center;
    font-weight: bold;
    font-size: 18px;
    border: none;
    background: transparent;
    box-shadow: none;
    
    &:focus {
      border: none;
      box-shadow: none;
    }
  }
}

.note-body {
  margin-bottom: 15px;
  min-height: 80px;
}

.note-text {
  font-size: 14px;
  line-height: 1.6;
  color: #444;
  white-space: pre-wrap;
  word-wrap: break-word;
  text-align: left;
}

.content-input {
  :deep(.el-textarea__inner) {
    border: none;
    background: transparent;
    resize: none;
    font-size: 14px;
    line-height: 1.6;
    padding: 5px;
    
    &:focus {
      border: none;
      box-shadow: none;
    }
  }
}

.note-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-top: 2px dashed rgba(0, 0, 0, 0.1);
  padding-top: 10px;
}

.note-meta {
  flex: 1;
}

.meta-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #666;
}

.author {
  font-weight: 500;
  color: #333;
}

.separator {
  color: #ccc;
}

.date {
  color: #888;
}

.meta-edit {
  .author-input {
    :deep(.el-input__inner) {
      border: none;
      background: transparent;
      font-size: 12px;
      text-align: left;
      
      &:focus {
        border: none;
        box-shadow: none;
      }
    }
  }
}

.edit-actions {
  .edit-btn {
    padding: 4px 12px;
    font-size: 12px;
  }
  
  .edit-buttons {
    display: flex;
    gap: 5px;
    
    .el-button {
      padding: 4px 8px;
      font-size: 12px;
    }
  }
}

// 便签阴影效果
.note-shadow {
  position: absolute;
  bottom: -8px;
  left: 10px;
  right: 10px;
  height: 20px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  filter: blur(4px);
  z-index: 1;
}

// 响应式设计
@media (max-width: 768px) {
  .note-card {
    width: 250px;
    margin: 15px;
    padding: 15px;
  }
  
  .note-title {
    font-size: 16px;
  }
  
  .note-text {
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  .note-card {
    width: 200px;
    margin: 10px;
    padding: 12px;
  }
  
  .note-title {
    font-size: 14px;
  }
  
  .note-text {
    font-size: 12px;
  }
  
  .meta-info {
    font-size: 11px;
  }
}

// 不同颜色主题的特殊处理
.note-card {
  &[style*="#FFE5B4"] {
    // 浅橙色 - 温暖主题
    .note-title { color: #8B4513; }
    .note-text { color: #654321; }
    .author { color: #A0522D; }
  }
  
  &[style*="#E0FFFF"] {
    // 浅青色 - 清新主题
    .note-title { color: #006400; }
    .note-text { color: #008080; }
    .author { color: #2E8B57; }
  }
  
  &[style*="#FFB6C1"] {
    // 浅粉色 - 可爱主题
    .note-title { color: #C71585; }
    .note-text { color: #DB7093; }
    .author { color: #FF69B4; }
  }
  
  &[style*="#E6E6FA"] {
    // 薰衣草色 - 优雅主题
    .note-title { color: #4B0082; }
    .note-text { color: #6A5ACD; }
    .author { color: #9370DB; }
  }
  
  &[style*="#FFFACD"] {
    // 柠檬绸色 - 明亮主题
    .note-title { color: #B8860B; }
    .note-text { color: #DAA520; }
    .author { color: #F4A460; }
  }
  
  &[style*="#F0FFF0"] {
    // 蜂蜜露色 - 自然主题
    .note-title { color: #228B22; }
    .note-text { color: #32CD32; }
    .author { color: #6B8E23; }
  }
}
</style>