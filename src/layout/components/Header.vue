<script setup lang="ts">
import {getCurrentWindow} from '@tauri-apps/api/window';
import SvgIcon from "@components/SvgIcon/index.vue";
import {computed, onMounted, ref} from "vue";

const appWindow = getCurrentWindow();

onMounted(() => {

})

</script>

<template>
  <div class="title-bar">
    <div class="drag-region" data-tauri-drag-region>
      <div class="fill-width flex" data-tauri-drag-region>
        <div class="logo-container">
          <div class="logo-icon">
            <svg-icon icon-class="logo" class="logo"></svg-icon>
          </div>
          <div class="logo-text">
            <div class="logo-main">墨音笺</div>
          </div>
        </div>
      </div>

    </div>
    <div class="controls">
      <svg-icon icon-class="minsize" id="title-bar-minimize" class="control" @click="appWindow.minimize()">
      </svg-icon>
      <svg-icon icon-class="maxsize" id="title-bar-maximize" class="control" size="14"
                @click="appWindow.toggleMaximize()">
      </svg-icon>
      <svg-icon icon-class="close" id="title-bar-close" class="control" @click="appWindow.close()">
      </svg-icon>
    </div>
  </div>
</template>

<style scoped lang="scss">
.title-bar {
  height: 40px;
  background: var(--el-color-primary);
  user-select: none;
  display: grid;
  grid-template-columns: auto max-content;

  .drag-region {
    height: 40px;
    width: calc(100vw - 200px);
  }

  .controls {
    display: flex;
    color: #ffffff;
    align-items: center; /* 垂直居中 */
    .control {
      padding: 0 20px;
      height: 40px;

      &:hover {
        background: var(--el-color-primary-light-3);
        //color: var(--el-color-primary);
      }
    }
  }

  .avatar {
    display: flex;
    align-items: center; /* 垂直居中 */
    height: 40px;
    padding: 0 20px;
  }

  .logo-container {
    display: flex;
    align-items: center;
    padding: 0 20px;
    height: 40px;

    .logo-icon {
      width: 24px;
      height: 24px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 6px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 20px;
      font-weight: bold;
      color: #ffffff;
      margin-right: 12px;
      border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .logo-text {
      .logo-main {
        font-size: 16px;
        font-weight: 600;
        color: #ffffff;
        line-height: 1.2;
      }

      .logo-sub {
        font-size: 12px;
        color: rgba(255, 255, 255, 0.7);
        line-height: 1.2;
        font-family: 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
      }
    }
  }

  .menu {
    display: flex;
    align-items: center;

    .main-menu {
      margin-left: 200px;
    }

    .main-menu-btn {
      background: transparent;
      border: unset;
      color: #ffffff;
    }
  }
}


</style>