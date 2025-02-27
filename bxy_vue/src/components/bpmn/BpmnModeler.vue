<template>
  <n-layout has-sider sider-placement="right" style="height: 100vh;">
    <n-layout>
      <n-layout-header bordered>
        <n-flex justify="end">
          <n-button-group size="small">
            <n-button type="default">
              加载BPMN
            </n-button>
            <n-button type="default">
              <n-icon>
                <DownloadOutline />
              </n-icon>
            </n-button>
            <n-button type="default">
              <n-icon>
                <ImageOutline />
              </n-icon>
            </n-button>
          </n-button-group>
          <n-button-group size="small">
            <n-button type="default">
              新建
            </n-button>
            <n-button type="default">
              打开
            </n-button>
            <n-button type="default" title="保存">
              <n-icon>
                <Save />
              </n-icon>
            </n-button>
            <n-button type="default" title="删除">
              <n-icon>
                <Close />
              </n-icon>
            </n-button>
          </n-button-group>
          <n-button-group size="small">
            <n-button type="default" title="预览">
              <n-icon>
                <Eye />
              </n-icon>
            </n-button>
            <n-button type="default" title="模拟">
              <n-icon>
                <GameControllerOutline />
              </n-icon>
            </n-button>
          </n-button-group>
          <n-button-group size="small">
            <n-button @click="handleUndo">撤销</n-button>
            <n-button @click="handleRedo">恢复</n-button>
            <n-button>清空</n-button>
          </n-button-group>
          <n-button-group size="small">
            <n-button @click="handlerZoom(0.1)" title="放大">
              <n-icon>
                <Add />
              </n-icon>
            </n-button>
            <n-button @click="handlerZoom(-0.1)" title="缩小">
              <n-icon>
                <Remove />
              </n-icon>
            </n-button>
            <n-button @click="handlerZoom(0)">还原</n-button>
          </n-button-group>
          <n-button size="small">
            <n-icon>
              <Map />
            </n-icon>
          </n-button>
        </n-flex>
      </n-layout-header>
      <n-layout-content bordered>
        <div id="canvas" class="canvas" ref="canvas"></div>
        <!-- 属性面板 -->
        <!-- <div id="panel" class="panel" ref="panel"></div> -->
      </n-layout-content>
    </n-layout>
    <n-layout-sider bordered collapse-mode="transform" show-trigger="bar" class="palette">
      属性面板
    </n-layout-sider>
  </n-layout>
</template>
<script setup lang="ts" name="BpmnModeler">
import {
  Add, Close, DownloadOutline,
  Eye,
  GameControllerOutline,
  ImageOutline, Map, Remove, Save
} from '@vicons/ionicons5'
import 'bpmn-js/dist/assets/bpmn-font/css/bpmn-codes.css'
import 'bpmn-js/dist/assets/bpmn-font/css/bpmn-embedded.css'
import 'bpmn-js/dist/assets/bpmn-font/css/bpmn.css'
import 'bpmn-js/dist/assets/bpmn-js.css' // 左边工具栏以及编辑节点的样式
import 'bpmn-js/dist/assets/diagram-js.css'
import BpmnModeler from 'bpmn-js/lib/Modeler' // Bpmn-Modeler

// import {
//   BpmnPropertiesPanelModule,
//   BpmnPropertiesProviderModule,
// } from 'bpmn-js-properties-panel'
import minimapModule from 'diagram-js-minimap'
import "diagram-js-minimap/assets/diagram-js-minimap.css" // 小地图样式
import translate from './i18n/translate'
import { bpmnXML } from './xml'
// 缩放大小
const scale = ref(1)
// 画图区域
const canvas = ref()
const panel = ref()
// 流程模型
const modeler = ref()
// 国际化
const customTranslateModule = {
  translate: ['value', translate]
}
// 撤销
const handleUndo = () => {
  modeler.value.get('commandStack').undo()
}
// 恢复
const handleRedo = () => {
  modeler.value.get('commandStack').redo()
}
const handlerZoom = (radio: number) => {
  const newScale = !radio ? 1.0 : scale.value + radio
  modeler.value.get("canvas").zoom(newScale)
  scale.value = newScale
}
// 初始化
const init = async () => {
  //初始化 modeler 编辑器
  modeler.value = new BpmnModeler({
    container: canvas.value,
    // propertiesPanel: {
    //   parent: panel.value
    // },
    additionalModules: [
      customTranslateModule,
      // BpmnPropertiesPanelModule,
      // BpmnPropertiesProviderModule,
      minimapModule,
    ]
  })
  try {
    await modeler.value.importXML(bpmnXML)
    // 让图能自适应屏幕
    modeler.value.get('canvas').zoom('fit-viewport')
  } catch (err) {
    console.error('Failed to import BPMN diagram', err)
  }
}
const success = () => {
  const eventBus = modeler.value.get("eventBus")

  eventBus.on("element.click", function (e: any) {
    if (e.element.businessObject.$type === "bpmn2:userTask") {
      console.log(
        "这是一个用户节点",
        e.element.businessObject.id,
        e.element.businessObject.$type,
        e.element.businessObject.name
      )
    }
  })
}
onMounted(async () => {
  init()
})
</script>
<style lang="scss" scoped>
// 重置工具面板样式
:deep(.djs-palette.two-column.open) {
  width: 47px;
  top: 1px;
  left: 1px;
}

.palette {
  width: 50px;
}

.container {
  //position: absolute;
  background-color: #ffffff;
  width: 100%;
  height: 100%;
  overflow: auto;
  background-image: linear-gradient(90deg,
      rgba(220, 220, 220, 0.5) 6%,
      transparent 0),
    linear-gradient(rgba(192, 192, 192, 0.5) 6%, transparent 0);
  background-size: 12px 12px;
  width: 100%;
  height: calc(100vh - 82px);
  -webkit-tap-highlight-color: rgba(255, 255, 255, 0);
}

.toolbar {
  top: 0;
  left: 0;
  width: 100%;
  padding: 5px;
  background-color: #f8f9fa;
}

.canvas {
  background-color: #ffffff;
  width: 100%;
  height: 100%;
  overflow: auto;
  background-image: linear-gradient(90deg,
      rgba(220, 220, 220, 0.5) 6%,
      transparent 0),
    linear-gradient(rgba(192, 192, 192, 0.5) 6%, transparent 0);
  background-size: 12px 12px;
  width: 100%;
  height: calc(100vh - 82px);
  -webkit-tap-highlight-color: rgba(255, 255, 255, 0);
}

.panel {
  position: absolute;
  right: 0;
  top: 50px;
  width: 300px;
}
</style>
