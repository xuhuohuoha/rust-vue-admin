<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="5" label="应用代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入应用代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="5" label="应用名称" path="app_name">
        <n-input v-model:value="model.app_name" placeholder="请输入应用名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="授权代码" path="app_code">
        <n-input v-model:value="model.app_code" placeholder="请输入授权代码" />
      </n-form-item-gi>
      <n-form-item-gi :span="6" label="授权密钥" path="app_key">
        <n-input v-model:value="model.app_key" placeholder="请输入授权密钥" />
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
  app_code: '',
  app_key: '',
  app_name: '',
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
