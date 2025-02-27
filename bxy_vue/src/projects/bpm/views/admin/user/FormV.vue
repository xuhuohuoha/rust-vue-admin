<template>
  <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
    <n-grid :cols="24" :x-gap="24">
      <n-form-item-gi :span="12" label="用户账号" path="ucode">
        <n-input v-model:value="model.ucode" placeholder="请设置用户账号" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="用户名称" path="uname">
        <n-input v-model:value="model.uname" placeholder="请设置用户名称" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="用户密码" path="upwd">
        <n-input type="password" v-model:value="model.upwd" show-password-on="mousedown" placeholder="请设置用户密码" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="用户性别" path="sex">
        <n-input v-model:value="model.sex" placeholder="请选择用户性别" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="电子邮箱" path="email">
        <n-input v-model:value="model.email" placeholder="请输入用户常用电子邮箱" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="联系电话" path="phone">
        <n-input v-model:value="model.phone" placeholder="请输入用户常用联系电话" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="社交QQ" path="qq">
        <n-input v-model:value="model.qq" placeholder="请输入用户社交QQ" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="微信账号" path="webchat">
        <n-input v-model:value="model.webchat" placeholder="请输入用户微信账号" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="授权密码" path="pin">
        <n-input v-model:value="model.pin" placeholder="请设置用户授权密码" />
      </n-form-item-gi>
      <n-form-item-gi :span="12" label="审批密码" path="pass">
        <n-input v-model:value="model.pass" placeholder="请设置用户审批密码" />
      </n-form-item-gi>
      <n-form-item-gi :span="24" label="备注说明" path="remark">
        <n-input v-model:value="model.remark" placeholder="请输入用户相关备注说明" type="textarea" :autosize="{
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
import type { HttpEntity } from '@/types'

const bxy = defineModel<BModalProps>('bxy')

const model = ref<HttpEntity>({
  id: '',
  u_id: '',
  ord: 0,
  status: '1',
  remark: '',
  ucode: '',
  uname: '',
  upwd: '',
  sex: '',
  email: '',
  qq: '',
  webchat: '',
  phone: '',
  pin: '',
  pass: '',
  avatar: '',
  ext1: '',
  ext2: '',
  ext3: ''
})
const { formRef, onBeforeSave, onAfterSave, onSave, onReset } = formAction(bxy.value!)

const rules = {
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
