<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="12" label="树型名称" path="tname">
        <n-input v-model:value="model.tname" placeholder="请输入关联树名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="关联来源" path="tv">
        <n-input v-model:value="model.tv" placeholder="请输入关联树来源的表名或视图名" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="代码字段" path="guid">
        <n-input v-model:value="model.guid" placeholder="请输入代码字段" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="上级字段" path="pguid">
        <n-input v-model:value="model.pguid" placeholder="请输入上级代码字段" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="显示字段" path="sfield">
        <n-input v-model:value="model.sfield" placeholder="请输入显示名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="排序方式" path="tord">
        <n-input v-model:value="model.tord" placeholder="请输入排序字段" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="查询条件" path="twhere">
        <n-input v-model:value="model.twhere" placeholder="请输入查询条件" type="textarea" :autosize="{
          minRows: 2,
          maxRows: 3,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="关联应用" path="app">
        <n-select v-model:value="model.app" :options="appOptions" placeholder="请选择关联应用"
          @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="关联菜单" path="app">
        <n-tree-select v-model:value="model.mcode" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择关联菜单" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="树型字段" path="tfields">
        <n-input v-model:value="model.tfields" placeholder="请设置关联树采用的关联字段" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="菜单字段" path="mfields">
        <n-input v-model:value="model.mfields" placeholder="请设置菜单采用的关联字段" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="授权类型" path="atype">
        <n-select v-model:value="model.atype" :options="authTypeOptions" placeholder="请选择授权类型" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="授权方式" path="amethod">
        <n-select v-model:value="model.amethod" :options="authMethodOptions" placeholder="请选择授权方式" />
      </n-form-item-gi>
      <n-form-item-gi :span="8" label="指定用户" path="u_id">
        <n-select v-model:value="model.u_id" :options="userOptions" multiple placeholder="请选择用户" />
      </n-form-item-gi>
      <n-form-item-gi :span="4" label="指定部门" path="o_id">
        <n-tree-select v-model:value="model.o_id" :options="orgOptions" :key-field="'guid'" :label-field="'oname'"
          placeholder="请选择部门" />
      </n-form-item-gi>
      <n-form-item-gi :span="4" label="指定角色" path="r_id">
        <n-tree-select v-model:value="model.r_id" :options="roleOptions" :key-field="'guid'" :label-field="'rname'"
          placeholder="请选择角色" />
      </n-form-item-gi>
      <n-form-item-gi :span="4" label="指定岗位" path="p_id">
        <n-tree-select v-model:value="model.p_id" :options="postOptions" :key-field="'guid'" :label-field="'pname'"
          placeholder="请选择岗位" />
      </n-form-item-gi>
      <n-form-item-gi :span="4" label="指定职务" path="d_id">
        <n-tree-select v-model:value="model.d_id" :options="dutyOptions" :key-field="'guid'" :label-field="'dname'"
          placeholder="请选择职务" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入关联树相关备注说明" type="textarea" :autosize="{
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
  tv: '',
  sfield: '',
  twhere: '',
  tord: '',
  tname: '',
  mcode: '',
  tfields: '',
  mfields: '',
  atype: 0,
  amethod: 0,
  u_id: '',
  r_id: '',
  o_id: '',
  p_id: '',
  d_id: '',
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
}

// 应用选项
const appOptions = ref<any[]>()
// 菜单选项
const menuOptions = ref<any[]>()
// 授权类型选项
const authTypeOptions = ref<any[]>()
// 授权方式选项
const authMethodOptions = ref<any[]>()
// 用户选项
const userOptions = ref<any[]>()
// 部门选项
const orgOptions = ref<any[]>()
// 角色选项
const roleOptions = ref<any[]>()
// 岗位选项
const postOptions = ref<any[]>()
// 职务选项
const dutyOptions = ref<any[]>()
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
  authTypeOptions.value = await DevGetSelect('BXY_DICT_AUTH_TYPE', [])
  authMethodOptions.value = await DevGetSelect('BXY_DICT_AUTH_METHOD', [])
  userOptions.value = await DevGetSelect('BXY_USER_UID', [])
  const _org_options = await DevGetSelect('BXY_ORG_TREE', [])
  orgOptions.value = array2tree(_org_options, 'guid', 'pguid')
  const _role_options = await DevGetSelect('BXY_ROLE_TREE', [])
  roleOptions.value = array2tree(_role_options, 'guid', 'pguid')
  const _post_options = await DevGetSelect('BXY_POST_TREE', [])
  postOptions.value = array2tree(_post_options, 'guid', 'pguid')
  const _duty_options = await DevGetSelect('BXY_DUTY_TREE', [])
  dutyOptions.value = array2tree(_duty_options, 'guid', 'pguid')
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
