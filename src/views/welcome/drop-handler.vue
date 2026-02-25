<script setup lang="ts">
import {getCurrentWebview} from "@tauri-apps/api/webview";
import {onBeforeUnmount, onMounted, ref, watch} from "vue";
import {call} from "@/utils/commands.ts";
import {ElMessage} from "element-plus";

let unlisten = null

onMounted(async () => {
  unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'over') {
      //console.log('User hovering', event.payload.position);
      // const elements = document.elementsFromPoint(event.payload.position.x, event.payload.position.y);
      // elements.forEach(el => {
      //   el.classList.remove('drop-active');
      // })
      // elements[0].classList.add('drop-active');
    } else if (event.payload.type === 'drop') {
      //console.log('User dropped', event.payload.paths);
      // const elements = document.elementsFromPoint(event.payload.position.x, event.payload.position.y);
      // elements[0].classList.add('drop-active');
      // console.log(elements)
      if (event.payload.paths.length === 0) {
        return
      }

      for (let path of event.payload.paths) {
        addWorkspace(path)
      }
    } else {
      console.log('File drop cancelled');
    }
  });

})


onBeforeUnmount(() => {
  if (unlisten) {
    unlisten()
  }
})

const addWorkspace = async (filePath: string) => {
  await call('add_workspace', {
    file_path: filePath
  });
}
</script>

<template>

</template>

<style scoped lang="scss">

</style>