<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="职务代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入职务代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="上级职务" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="dutyOptions" :key-field="'guid'" :label-field="'dname'"
          placeholder="请选择上级职务" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="职务名称" path="dname">
        <n-input v-model:value="model.dname" placeholder="请输入职务名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="2">
        <n-button type="info" @click="handleQuery">搜索</n-button>
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>
<script setup lang="ts">
import { DevGetSelect } from '@/projects/bpm/api'
import { array2tree } from '@/toolkit'
import type { HttpRequestData } from '@/types'

const params = defineModel<Map<string, any>>('params')

const props = withDefaults(defineProps<{
  handleQuery: () => void
}>(), {
  handleQuery: () => { }
})

const model = ref<HttpRequestData>({
  guid: '',
  pguid: '',
  dname: '',
})

const handleQuery = () => {
  if (params.value) {
    for (const key of Object.keys(model.value
    )) {
      params.value.set(key, model.value[key])
    }
  }
  props.handleQuery()
}

// 职务选项
const dutyOptions = ref<any[]>()

onMounted(async () => {
  const _duty_options = await DevGetSelect('BXY_DUTY_TREE', [])
  dutyOptions.value = array2tree(_duty_options, 'guid', 'pguid')
})

</script>
