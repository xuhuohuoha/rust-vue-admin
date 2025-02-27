<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="24" label="应用代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入应用代码（手动输入请确保唯一，为空则由系统自动生成）" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="应用名称" path="app_name">
        <n-input v-model:value="model.app_name" placeholder="请输入应用名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="应用属性" path="att">
        <n-input v-model:value="model.att" placeholder="请输入应用扩展属性" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="应用标识" path="logo">
        <n-input v-model:value="model.logo" placeholder="请选择应用LOGO" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="应用类型" path="att_type">
        <n-input v-model:value="model.app_type" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="授权代码" path="app_code">
        <n-input v-model:value="model.app_code" placeholder="请生成应用授权代码" :disabled=true /><n-button
          @click="handleUUID('code')">生成</n-button>
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="授权密钥" path="app_key">
        <n-input v-model:value="model.app_key" placeholder="请生成应用授权密钥" :disabled=true /><n-button
          @click="handleUUID('key')">生成</n-button>
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入应用相关备注说明" type="textarea" :autosize="{
          minRows: 3,
          maxRows: 5,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="显示顺序" path="ord">
        <n-input-number v-model:value="model.ord" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="数据状态" path="status">
        <n-switch v-model:value="model.status" checked-value="1" unchecked-value="0">
          <template #unchecked>停用</template>
          <template #checked>正常</template>
        </n-switch>
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>
<script setup lang="ts">
import type { BModalProps } from '@/components/modal/types'
import { formAction } from '@/hooks/FormAction'
import { GetUUID } from '@/projects/bpm/api'
import type { HttpEntity } from '@/types'

const msg = useMessage()

const bxy = defineModel<BModalProps>('bxy')

const model = ref<HttpEntity>({
  id: '',
  guid: '',
  ord: 0,
  status: '1',
  remark: '',
  app_code: '',
  app_key: '',
  app_name: '',
  app_att: '',
  app_type: '',
  logo: ''
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
}
const handleUUID = (type: string) => {
  GetUUID().then((res) => {
    const { success, message, data } = res.data
    if (success) {
      if ('code' === type) {
        model.value.app_code = data
      } else {
        model.value.app_key = data
      }
    } else {
      msg.error(message)
    }
  })
}
onMounted(() => {
  // 非新增时数据初始化
  if (bxy.value?.model !== 'A') {
    model.value = formRef.value?.model?.value
  }
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
