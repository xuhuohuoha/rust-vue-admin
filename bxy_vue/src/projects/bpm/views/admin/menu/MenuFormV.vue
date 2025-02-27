<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="12" label="所属应用" path="app">
        <n-select v-model:value="model.app" :options="appOptions" placeholder="请选择所属应用"
          @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="上级菜单" path="pguid">
        <n-tree-select v-model:value="model.pguid" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择上级菜单（为空则与菜单代码相同，默认为根菜单）" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="菜单代码" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入菜单代码（手动输入请确保唯一，为空则由系统自动生成）" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="菜单名称" path="mname">
        <n-input v-model:value="model.mname" placeholder="请输入菜单名称" />
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
      <n-form-item-gi :span="12" label="菜单图标" path="icon">
        <n-input v-model:value="model.icon" placeholder="请设置菜单图标" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="菜单样式" path="style">
        <n-input v-model:value="model.style" placeholder="请设置菜单样式" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="查询组件" path="query">
        <n-input v-model:value="model.query" placeholder="请设置查询组件" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="列表组件" path="cols">
        <n-input v-model:value="model.cols" placeholder="请设置列表组件" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="查询脚本" path="qscript">
        <n-input v-model:value="model.qscript" placeholder="请输入查询脚本" type="textarea" :autosize="{
          minRows: 5,
          maxRows: 8,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="列表脚本" path="cscript">
        <n-input v-model:value="model.cscript" placeholder="请输入列表脚本" type="textarea" :autosize="{
          minRows: 5,
          maxRows: 8,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入菜单相关备注说明" type="textarea" :autosize="{
          minRows: 3,
          maxRows: 5,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="是否隐藏" path="visible">
        <n-switch v-model:value="model.visible" checked-value="1" unchecked-value="0">
          <template #unchecked>否</template>
          <template #checked>是</template>
        </n-switch>
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="日志方式" path="log_method">
        <n-select v-model:value="model.log_method" :options="logOptions" placeholder="请选择日志记录方式" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="是否缓存" path="is_cache">
        <n-switch v-model:value="model.is_cache" checked-value="1" unchecked-value="0">
          <template #unchecked>否</template>
          <template #checked>是</template>
        </n-switch>
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="缓存方法" path="data_cache_method">

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
  mtype: 'C',
  app: '',
  path: '',
  api: '',
  uio: 0,
  method: '',
  opt: '',
  alias: '',
  tbl: '',
  query: '',
  cols: '',
  qscript: '',
  cscript: '',
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
// 日志方式选项
const logOptions = ref<any[]>()
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
  logOptions.value = await DevGetSelect('BXY_DICT_LOG_METHOD', [])
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
