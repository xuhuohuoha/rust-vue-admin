<template>
  <n-modal v-model:show="bxy!.show" :show-icon="bxy!.showIcon" :preset="bxy!.preset" :positive-text="bxy!.positiveText"
    :negative-text="bxy!.negativeText" :title="bxy!.title" :block-scroll="true" :bordered="false" @close="handleClose"
    :segmented="{ content: 'soft', footer: 'soft' }">
    <Suspense>
      <component :is="formComponent" ref="comRef" :bxy></component>
    </Suspense>
  </n-modal>
</template>
<script lang="ts" setup name="BModal">
import { useTabStore } from '@/stores'
import type { BModalProps } from './types'
const bxy = defineModel<BModalProps>('bxy')
const { modules, replaceAlias } = useTabStore()

// 动态加载组件
const formComponent = computed(() => {
  const path = `${bxy.value?.component}.vue`
  return toRaw(modules[replaceAlias(path)])
})
// 业务组件
const comRef = ref()
// 关闭
const handleClose = () => {
  bxy.value!.show = false
}
</script>
<style lang="scss" scoped></style>
