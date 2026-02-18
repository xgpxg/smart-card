<template>
  <span>
      <el-tooltip v-bind="$attrs">
          <template #default>
              <svg-icon icon-class="help" color="#aaaaaa" :size="14"></svg-icon>
          </template>
          <template #content>
              <slot>
                  {{content}}
              </slot>
          </template>
      </el-tooltip>
  </span>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "HelpTip",
  components: {SvgIcon},
  props: {
    content: {
      type: String
    }
  }
}
</script>

<style scoped>

</style>