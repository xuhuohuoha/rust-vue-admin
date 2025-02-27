<template>
  <n-modal v-model:show="bxy!.show" :show-icon="bxy!.showIcon" :preset="bxy!.preset" :positive-text="bxy!.positiveText"
    :negative-text="bxy!.negativeText" :title="bxy!.title" :block-scroll="true" :bordered="false"
    :segmented="{ content: 'soft', footer: 'soft' }">
    <component :is="formComponent" ref="comRef" :bxy></component>
    <template #action>
      <n-flex justify="center">
        <n-button type="info" @click="handleSave" v-if="saveRef">保存</n-button>
        <n-button type="warning" @click="handleReset" v-if="saveRef">重置</n-button>
        <n-button type="primary" @click="handleClose">关闭</n-button>
      </n-flex>
    </template>
  </n-modal>
</template>
<script setup lang="ts" name="BFormModal">
import { useTabStore } from '@/stores'
import type { BModalProps } from './types'
const bxy = defineModel<BModalProps>('bxy')
const { modules, replaceAlias } = useTabStore()

const props = withDefaults(defineProps<{
  handleQuery: () => void
}>(), {
  handleQuery: () => { }
})

// 动态加载组件
const formComponent = computed(() => {
  const path = `${bxy.value?.component}.vue`
  return toRaw(modules[replaceAlias(path)])
})
// 是否需要保存功能
const saveRef = computed(() => {
  return bxy.value?.model === 'A' || bxy.value?.model === 'E' || bxy.value?.model === 'D'
})
// 业务组件
const comRef = ref()
// 保存
const handleSave = () => {
  comRef.value.onBeforeSave().then((res_bf: boolean) => {
    if (res_bf) comRef.value.onSave().then((res: boolean) => {
      if (res) comRef.value.onAfterSave().then((res_af: boolean) => {
        if (res_af) {
          handleClose()
          props.handleQuery()
        }
      })
    })
  })
}
// 重置
const handleReset = () => {
  comRef.value.onReset()
}
// 关闭
const handleClose = () => {
  bxy.value!.show = false
}
</script>
<style lang="scss" scoped></style>
