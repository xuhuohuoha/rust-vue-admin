<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="部门代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入部门代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="上级部门" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="orgOptions" :key-field="'guid'" :label-field="'oname'"
          placeholder="请选择上级部门" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="部门名称" path="oname">
        <n-input v-model:value="model.oname" placeholder="请输入部门名称" />
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
  oname: '',
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

// 部门选项
const orgOptions = ref<any[]>()

onMounted(async () => {
  const _org_options = await DevGetSelect('BXY_ORG_TREE', [])
  orgOptions.value = array2tree(_org_options, 'guid', 'pguid')
})

</script>
