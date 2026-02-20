<template>
  <div class="menu">
    <!-- 工作空间列表 -->
    <WorkspaceList class="audio-list-container"/>

    <!--    <div class="bottom">
          &lt;!&ndash; 用户信息区域 &ndash;&gt;
          <UserInfo :collapsed="collapse" />
        </div>-->
  </div>
</template>

<script>
import SvgIcon from "../../components/SvgIcon/index.vue";
import DevRemark from "@components/DevUtil/dev-remark.vue";
import AutoAvatar from "@components/avatar/AutoAvatar.vue";
import WorkspaceList from "@/views/workspace/workspace-list.vue";
import UserInfo from "@/views/user/index.vue";

const VERSION = import.meta.env.VITE_VERSION
export default {
  components: {
    UserInfo,
    WorkspaceList,
    AutoAvatar,
    DevRemark,
    SvgIcon
  },
  props: {
    collapse: Boolean,
  },
  provide() {
    return {}
  },
  data() {
    return {
      isCollapse: true,
    }
  },
  computed: {
    version() {
      return VERSION
    }
  },
  watch: {
    collapse: {
      handler(newVal) {
        this.isCollapse = newVal
      },
      immediate: true
    }
  },
  created() {
  },
  mounted() {
  },
  beforeUnmount() {

  },
  methods: {
    showUserInfoDialog() {
      this.$refs['user-info-dialog'].show()
    }
  }
}
</script>
<style scoped lang="scss">
.menu {
  height: calc(100vh - 40px);
  background: #2e2e2e;
  border-right: rgb(57, 57, 57) solid 1px;
  z-index: 1;
  width: 100%;
  position: relative;

  :deep(.el-menu) {
    border-right: unset;
    background: transparent;

    --el-menu-item-height: 40px;

  }

  :deep(.el-menu-item) {
    margin: 0 10px;
    font-size: 16px;
    border-radius: 10px;

    &.is-active {
      font-weight: 600;
      color: var(--el-color-primary);
      background: #F3EBFF;

    }

    &:hover {
      font-weight: bold;
      color: var(--el-color-primary);
      background: #F3EBFF;
      border-radius: 10px;
    }

    .el-icon {
      > :hover {
        font-size: 1.5em;
        transition: all 0.3s;
      }
    }
  }

  :deep(.el-badge__content) {
    border: none;
    scale: 0.8;
  }
}

.audio-list-container {
  margin-top: 70px; // 为用户信息区域留出空间
}
</style>