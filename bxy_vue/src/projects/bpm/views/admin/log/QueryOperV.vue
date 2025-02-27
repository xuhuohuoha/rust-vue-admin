<template>
  <n-form ref="formRef" :model="model" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="5" label="用户账号" path="u_id">
        <n-input v-model:value="model.u_id" placeholder="请输入用户账号" />
      </n-form-item-gi>
      <n-form-item-gi :span="5" label="IP地址" path="ip">
        <n-input v-model:value="model.ip" placeholder="请输入IP地址" />
      </n-form-item-gi>
      <n-form-item-gi :span="10" label="操作时间">
        <n-date-picker v-model:formatted-value="model.begin_time" type="date" value-format="yyyy-MM-dd" />
        <n-date-picker v-model:formatted-value="model.end_time" type="date" value-format="yyyy-MM-dd" />
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
  ip: '',
  begin_time: null,
  end_time: null
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
