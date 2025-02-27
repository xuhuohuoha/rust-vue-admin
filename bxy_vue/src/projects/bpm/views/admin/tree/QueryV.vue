<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="关联树名称" path="tname">
        <n-input v-model:value="model.tname" placeholder="请输入关联树名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="关联应用" path="app">
        <n-select v-model:value="model.app" :options="appOptions" placeholder="请选择应用" @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="关联菜单" path="menucode">
        <n-tree-select v-model:value="model.mcode" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择关联菜单" />
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
  app: '',
  mcode: '',
  tname: ''
})

// 应用选项
const appOptions = ref<any[]>()
// 菜单选项
const menuOptions = ref<any[]>()

// 应用下拉
const handleUpdateApp = async (value: string) => {
  const params = []
  params.push(value)
  const _menu_options = await DevGetSelect('BXY_MENU_C', params)
  menuOptions.value = array2tree(_menu_options, 'guid', 'pguid')
}

const handleQuery = () => {
  if (params.value) {
    for (const key of Object.keys(model.value
    )) {
      params.value.set(key, model.value[key])
    }
  }
  props.handleQuery()
}

onMounted(async () => {
  appOptions.value = await DevGetSelect('BXY_APP_ENABLED', [])
})

</script>
