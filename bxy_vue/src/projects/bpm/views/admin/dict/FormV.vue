<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="24" label="类别代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入数据字典类别代码（为空则由系统自动生成）" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="上级类别" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="dictOptions" :key-field="'guid'" :label-field="'dname'"
          placeholder="请选择数据字典上级类别（为空则与类别代码相同，默认为根类别）" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="类别名称" path="dname">
        <n-input v-model:value="model.dname" placeholder="请输入数据字典类别名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="类别属性" path="att">
        <n-input v-model:value="model.att" placeholder="请输入数据字典类别扩展属性" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入数据字典类别相关备注说明" type="textarea" :autosize="{
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
  guid: '',
  pguid: '',
  ord: 0,
  status: '1',
  remark: '',
  dname: '',
  att: ''
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
}

const dictOptions = ref<any[]>()

onMounted(async () => {
  // 非新增时数据初始化
  if (bxy.value?.model !== 'A') {
    model.value = formRef.value?.model?.value
  }
  const options = await DevGetSelect('BXY_DICT_TYPE', [])
  dictOptions.value = array2tree(options, 'guid', 'pguid')
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
