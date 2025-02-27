<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="5" label="用户ID" path="u_id">
        <n-input v-model:value="model.u_id" placeholder="请输入用户ID" />
      </n-form-item-gi>
      <n-form-item-gi :span="5" label="用户账号" path="ucode">
        <n-input v-model:value="model.ucode" placeholder="请输入用户账号" />
      </n-form-item-gi>
      <n-form-item-gi :span="5" label="用户名称" path="uname">
        <n-input v-model:value="model.uname" placeholder="请输入用户名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="5" label="用户状态" path="status">
        <n-input v-model:value="model.status" placeholder="请选择用户状态" />
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
  u_id: '',
  ucode: '',
  uname: '',
  status: ''
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
