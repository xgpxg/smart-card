<!--自适应头像：当头像为空是，默认取text的第一个字符作为头像，背景色由第一个字符hash得到-->
<template>
  <div>
    <el-avatar v-if="src" :src="src"
               style="background: transparent;" :size="size" shape="square"></el-avatar>
    <el-avatar v-else :style="{background: randomColor(text?text:'123555')}" :size="size" shape="square">
      {{ text?.substring(0, 1) }}
    </el-avatar>
  </div>
</template>

<script>


import {randomColor} from "@/utils/util.ts";

export default {
  name: "auto-avatar",
  computed: {},
  components: {},
  props: {
    src: String,
    text: String,
    size: String | Number
  },
  data() {
    return {}
  },
  created() {
    this.init()
  },
  methods: {
    randomColor,
    init() {

    }
  }
}
</script>


<style scoped lang="scss">

</style>