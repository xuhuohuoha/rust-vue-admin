<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="6" label="应用代码" path="app_code">
        <n-input v-model:value="model.app_code" placeholder="请输入应用代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="临时授权码" path="temp_code">
        <n-input v-model:value="model.temp_code" placeholder="请输入临时授权码" />
      </n-form-item-gi>
      <n-form-item-gi :span="4">
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
  app_code: '',
  temp_code: ''
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
