<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="12" label="所属应用" path="app">
        <n-select v-model:value="model.app" :options="appOptions" placeholder="请选择所属应用"
          @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="所属菜单" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择功能所属菜单" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="功能名称" path="mname">
        <n-input v-model:value="model.mname" placeholder="请输入菜单名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="功能类型" path="mtype">
        <n-select v-model:value="model.mtype" :options="fnOptions" placeholder="请输入菜单名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="路由方式" path="uio">
        <n-select v-model:value="model.uio" :options="routeOptions" placeholder="请选择路由方式" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="路由路径" path="path">
        <n-input v-model:value="model.path" placeholder="请设置路由路径" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="接口地址" path="api">
        <n-input v-model:value="model.api" placeholder="请设置接口地址" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="请求类型" path="method">
        <n-select v-model:value="model.method" :options="httpOptions" placeholder="请选择接口请求类型" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="打开方式" path="opt">
        <n-select v-model:value="model.opt" :options="openOptions" placeholder="请选择打开方式" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="显示别名" path="alias">
        <n-input v-model:value="model.alias" placeholder="请设置菜单显示别名" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="所属区域" path="show">
        <n-select v-model:value="model.show" :options="showOptions" placeholder="请设置功能所属区域" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="功能图标" path="icon">
        <n-input v-model:value="model.icon" placeholder="请设置菜单图标" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="功能样式" path="style">
        <n-select v-model:value="model.style" :options="buttonOptions" placeholder="请设置菜单样式" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入数据字典类别相关备注说明" type="textarea" :autosize="{
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
  guid: '',
  pguid: '',
  ord: 0,
  status: '1',
  remark: '',
  mname: '',
  mtype: '',
  app: '',
  uio: 0,
  path: '',
  api: '',
  method: null,
  opt: 0,
  alias: '',
  tbl: '',
  icon: '',
  style: '',
  show: 0,
  visible: '0',
  is_cache: '',
  log_method: '',
  data_cache_method: '',
  data_scope: '',
  i18n: '',
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
}

// 应用选项
const appOptions = ref<any[]>()
// 菜单选项
const menuOptions = ref<any[]>()
// 路由选项
const routeOptions = ref<any[]>()
// 请求选项
const httpOptions = ref<any[]>()
// 打开方式选项
const openOptions = ref<any[]>()
// 功能类型选项
const fnOptions = ref<any[]>()
// 所在区域选项
const showOptions = ref<any[]>()
// 功能样式选项
const buttonOptions = ref<any[]>()
// 应用下拉设置应用下菜单
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
    // 非新增则根据应用加载菜单
    handleUpdateApp(model.value.app)
  }
  appOptions.value = await DevGetSelect('BXY_APP_ENABLED', [])
  routeOptions.value = await DevGetSelect('BXY_DICT_ROUTE', [])
  httpOptions.value = await DevGetSelect('BXY_DICT_HTTP', [])
  openOptions.value = await DevGetSelect('BXY_DICT_OPEN', [])
  fnOptions.value = await DevGetSelect('BXY_DICT_FN', [])
  showOptions.value = await DevGetSelect('BXY_DICT_BUTTON_SCOPE', [])
  buttonOptions.value = await DevGetSelect('BXY_DICT_BUTTON_TYPE', [])
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
