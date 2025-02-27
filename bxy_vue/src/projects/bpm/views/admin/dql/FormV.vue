<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="24" label="所属应用" path="app">
        <n-select v-model:value="model.app" :options="appOptions" placeholder="请选择所属应用"
          @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="所属菜单" path="mcode">
        <n-tree-select v-model:value="model.mcode" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择所属菜单" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="脚本名称" path="sign">
        <n-input v-model:value="model.sign" placeholder="请输入动态SQL名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="SQL脚本" path="dql">
        <n-input v-model:value="model.dql" placeholder="请输入动态SQL脚本" type="textarea" :autosize="{
          minRows: 5,
          maxRows: 8,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入动态SQL脚本相关备注说明" type="textarea" :autosize="{
          minRows: 2,
          maxRows: 3,
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
  mcode: '',
  sign: '',
  dql: '',
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
}

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
