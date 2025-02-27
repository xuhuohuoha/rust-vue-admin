<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="24" label="授权功能" path="mcode">
        <n-tree-select v-model:value="model.mcode" checkable :check-strategy="'all'" :show-path="true"
          :options="menuOptions" :key-field="'guid'" :label-field="'mname'" placeholder="请选择功能" />
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
        <n-input v-model:value="model.remark" placeholder="请输入功能授权相关备注说明" type="textarea" :autosize="{
          minRows: 2,
          maxRows: 3,
        }" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="显示顺序" path="ord">
        <n-input-number v-model:value="model.ord" />
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
  ord: 0,
  remark: '',
  mcode: '',
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
  const _menu_options = await DevGetSelect('BXY_MENU_FN', [])
  menuOptions.value = array2tree(_menu_options, 'guid', 'pguid')
})

defineExpose({
  formRef, onBeforeSave, onSave, onAfterSave, onReset
})
</script>
