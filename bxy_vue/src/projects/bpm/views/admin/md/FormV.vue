<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="24" label="所属应用" path="app">
        <n-select v-model:value="prop.app" :options="appOptions" placeholder="请选择应用" @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="主表模块" path="menucode">
        <n-tree-select v-model:value="model.mcode" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择主表模块" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="从表模块" path="menucode">
        <n-tree-select v-model:value="model.dcode" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择从表模块" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="主表字段" path="m_fields">
        <n-input v-model:value="model.m_fields" placeholder="请输入主表关联字段，多个以逗号分隔" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="从表字段" path="d_fields">
        <n-input v-model:value="model.d_fields" placeholder="请输入从表关联字段，多个以逗号分隔" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入附件类别相关备注说明" type="textarea" :autosize="{
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
import { DevGetSelect } from '@/projects/bpm/api'
import { array2tree } from '@/toolkit'
import type { HttpEntity } from '@/types'

const bxy = defineModel<BModalProps>('bxy')

const model = ref<HttpEntity>({
  id: '',
  ord: 0,
  status: '1',
  remark: '',
  m_fields: '',
  d_fields: '',
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
}

const prop = ref({
  app: '',
  mcode: ''
})
// 应用选项
const appOptions = ref<any[]>()
// 菜单选项
const menuOptions = ref<any[]>()
// 应用下拉
const handleUpdateApp = async (value: string) => {
  const params = []
  params.push(value)
  const options = await DevGetSelect('BXY_MENU_C', params)
  menuOptions.value = array2tree(options, 'guid', 'pguid')
}

onMounted(async () => {
  // 非新增时数据初始化
  if (bxy.value?.model !== 'A') {
    model.value = formRef.value?.model?.value
  }
  appOptions.value = await DevGetSelect('BXY_APP_ENABLED', [])
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
