<template>
  <el-container>
    <el-header height="40px">
      <Header></Header>
    </el-header>
    <el-main>
      <div class="flex fill-width">
        <div>
          <Menu :collapse="collapse" :style="{width:menuWidth + 'px'}"></Menu>
        </div>
        <div class="fill-width">
          <AppMain></AppMain>
        </div>
      </div>
    </el-main>
  </el-container>


</template>


<script>

import Menu from "@/layout/components/Menu.vue";
import AppMain from "./components/AppMain.vue";
import Header from "@/layout/components/Header.vue";

export default {
  computed: {},
  components: {Header, AppMain, Menu},
  data() {
    return {
      menuWidth: 220,
      top: 0,
      showLogin: false,
      collapse: false,
    }
  },
  provide() {
    return {
      scrollTop: this.scrollTop,
      scrollTo: this.scrollTo,
      leftMenuWidth: this.getMenuWidth
    }
  },
  mounted() {
    PubSub.subscribe('NEED_LOGIN', (msg, data) => {
      this.showLogin = true
    })
  },
  methods: {
    getKey(route) {
      return route.fullPath;
    },
    switchCollapse(isCollapse) {
      this.collapse = isCollapse
      if (isCollapse) {
        this.menuWidth = 65
      } else {
        this.menuWidth = 220
      }
    },
    /**
     * 滚动事件
     * @param scrollTop
     */
    scroll({scrollTop}) {
      this.top = scrollTop
    },
    /**
     * 设置滚动距离顶部的位置
     * @param top
     */
    scrollTo(top) {
      this.$refs['scrollbar'].setScrollTop(top)
    },
    /**
     * 获取滚动距离顶部的位置
     * @returns {number}
     */
    scrollTop() {
      return this.top
    },
    getMenuWidth() {
      return this.menuWidth
    }
  }
}
</script>

<style scoped lang="scss">
.el-header {
  padding: 0;
}

.el-main {
}
</style>