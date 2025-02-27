<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="12" label="所属应用" path="app">
        <n-select v-model:value="prop.app" :options="appOptions" placeholder="请选择应用" @update:value="handleUpdateApp" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="授权菜单" path="menucode">
        <n-tree-select v-model:value="model.mcode" :options="menuOptions" :key-field="'guid'" :label-field="'mname'"
          placeholder="请选择菜单" @update:value="handleUpdateMenu" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="授权说明" path="title">
        <n-input v-model:value="model.title" placeholder="请输入行级授权说明" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="授权内容" path="content">
        <n-input v-model:value="model.content" placeholder="请输入行级授权详细内容" type="textarea" :autosize="{
          minRows: 3,
          maxRows: 5,
        }" />
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
        <n-input v-model:value="model.remark" placeholder="请输入行级授权相关备注说明" type="textarea" :autosize="{
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
import { DevGetMenuAuthFn, DevGetSelect } from '@/projects/bpm/api'
import { useUserStore } from '@/stores'
import { array2tree } from '@/toolkit'
import type { HttpEntity } from '@/types'

const msg = useMessage()

const bxy = defineModel<BModalProps>('bxy')

const model = ref<HttpEntity>({
  id: '',
  guid: '',
  ord: 0,
  status: '1',
  remark: '',
  title: '',
  content: '',
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

const prop = ref({
  app: '',
  mcode: ''
})
// 功能
const fns = ref<Array<any>>([])
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
// 菜单下拉
const handleUpdateMenu = async (
  value: string | number | Array<string | number> | null
) => {
  // 加载列表工具条
  const { u_id } = useUserStore().userState
  const fnParams = {
    app_code: prop.value.app,
    mcode: value,
    u_id: u_id,
  }
  fns.value.length = 0
  DevGetMenuAuthFn(fnParams).then((res) => {
    const { success, message, data } = res.data
    if (success) {
      data.forEach((e: any) => {
        fns.value?.push(e)
      })
    } else {
      msg.error(message)
    }
  })
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
