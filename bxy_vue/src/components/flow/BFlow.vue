<template>
  <div class="container">
    <n-flex>
      <n-button>打开</n-button>
      <n-button>下载</n-button>
      <n-button>图片</n-button>
      <n-button>保存</n-button>
      <n-button>预览</n-button>
      <n-button>模拟</n-button>
      <n-button>放大</n-button>
      <n-button>缩小</n-button>
      <n-button>清空</n-button>
    </n-flex>
    <div class="canvas" ref="canvas"></div>
    <div class="properties-panel" ref="properties"></div>
  </div>
</template>
<script setup lang="ts" name="BFlow">
// import 'bpmn-js/dist/assets/bpmn-font/css/bpmn-codes.css'
import 'bpmn-js/dist/assets/bpmn-font/css/bpmn-embedded.css'
// import 'bpmn-js/dist/assets/bpmn-font/css/bpmn.css'
import 'bpmn-js/dist/assets/bpmn-js.css'
import 'bpmn-js/dist/assets/diagram-js.css'
import BpmnModeler from 'bpmn-js/lib/Modeler' // Bpmn-Modeler
import { bpmnXML } from './demo'
import customTranslate from './i18n'

// 画图区域
const canvas = ref()
const properties = ref()
let modeler = null
const customTranslateModule = {
  translate: ['value', customTranslate]
}
onMounted(async () => {
  init()
})
const init = async () => {
  //初始化 modeler 编辑器
  modeler = new BpmnModeler({
    container: canvas.value,
    propertiesPanel: {
      parent: properties.value
    },
    additionalModules: [
      customTranslateModule
    ]
  })
  //加上这一句,否则无法添加节点元素
  // modeler.createDiagram();
  // import diagram
  try {
    await modeler.importXML(bpmnXML)
    // ...
  } catch (err) {
    console.error('Failed to import BPMN diagram', err)
  }
}
</script>
<style lang="scss" scoped>
// :deep(.djs-container) {
//   background-image: linear-gradient(90deg, hsla(0, 0%, 78.4%, .15) 10%, transparent 0), linear-gradient(hsla(0, 0%, 78.4%, .15) 10%, transparent 0);
//   background-size: 10px 10px;
// }

.container {
  position: absolute;
  // background-color: #ffffff;
  width: 100%;
  height: 100%;
}

.canvas {
  width: 100%;
  height: 100%;
}

.panel {
  position: absolute;
  right: 0;
  top: 0;
  width: 300px;
}
</style>
