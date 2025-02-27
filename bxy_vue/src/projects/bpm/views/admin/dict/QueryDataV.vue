<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="字典代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入字典代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="字典名称" path="att">
        <n-input v-model:value="model.att" placeholder="请输入字典名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="2">
        <n-button type="info" @click="handleQuery">搜索</n-button>
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>
<script setup lang="ts">
import type { HttpRequestData } from '@/types'

const params = defineModel<Map<string, any>>('params')

const props = withDefaults(defineProps<{
  handleQuery: () => void
}>(), {
  handleQuery: () => { }
})

const model = ref<HttpRequestData>({
  guid: '',
  att: '',
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

</script>
