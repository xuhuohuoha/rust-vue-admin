<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="岗位代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入岗位代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="上级岗位" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="postOptions" :key-field="'guid'" :label-field="'pname'"
          placeholder="请选择上级岗位" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="岗位名称" path="pname">
        <n-input v-model:value="model.pname" placeholder="请输入岗位名称" />
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
  pname: '',
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

// 岗位选项
const postOptions = ref<any[]>()

onMounted(async () => {
  const _post_options = await DevGetSelect('BXY_POST_TREE', [])
  postOptions.value = array2tree(_post_options, 'guid', 'pguid')
})

</script>
