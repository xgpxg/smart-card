<script setup lang="ts">
import AutoAvatar from '@/components/avatar/AutoAvatar.vue'
import { ref } from 'vue'

// 定义props
interface Props {
  collapsed?: boolean
}

const props = defineProps<Props>()

// 用户信息数据
const userInfo = ref({
  username: '管理员',
  avatarText: '管'
})
</script>

<template>
  <div class="user-info" :class="{ collapsed: collapsed }">
    <div class="user-content">
      <!-- 头像 -->
      <div class="avatar-wrapper">
        <AutoAvatar 
          :text="userInfo.avatarText" 
          :size="40"
        />
      </div>
      <!-- 用户名 -->
      <div class="username-wrapper">
        <span class="username">{{ userInfo.username }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.user-info {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 220px;
  height: 70px;
  background: #2e2e2e;
  border-bottom: 1px solid rgb(57, 57, 57);
  z-index: 100;
  padding: 15px 20px;
  box-sizing: border-box;
}

.user-content {
  display: flex;
  align-items: center;
  height: 100%;
}

.avatar-wrapper {
  flex-shrink: 0;
  margin-right: 12px;
}

.username-wrapper {
  flex: 1;
  min-width: 0;
}

.username {
  color: #ffffff;
  font-size: 16px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

// 折叠状态下的样式
.user-info.collapsed {
  width: 65px;
  padding: 15px 12px;
}

.user-info.collapsed .username {
  display: none;
}
</style>