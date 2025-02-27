<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="角色代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入角色代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="上级角色" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="roleOptions" :key-field="'guid'" :label-field="'rname'"
          placeholder="请选择上级角色" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="角色名称" path="rname">
        <n-input v-model:value="model.rname" placeholder="请输入角色名称" />
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
  rname: '',
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

// 角色选项
const roleOptions = ref<any[]>()

onMounted(async () => {
  const _role_options = await DevGetSelect('BXY_ROLE_TREE', [])
  roleOptions.value = array2tree(_role_options, 'guid', 'pguid')
})

</script>
