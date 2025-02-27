<template>
  <div :id="buid">
    <component :is="comp" />
  </div>
</template>

<script setup lang="ts" name="BTabPane">
import type { TabOption } from '@/stores/modules/tab/types'

const props = withDefaults(defineProps<TabOption>(), {
  title: '',
  component: null,
  guid: '',
  name: '',
  closeable: false,
  buid: '',
})

const comp = toRaw(props.component)
</script>

<style lang="scss" scoped></style>
