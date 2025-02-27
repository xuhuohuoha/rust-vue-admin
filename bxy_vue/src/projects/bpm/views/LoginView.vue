<template>
  <div class="login-wrap">
    <n-space vertical>
      <n-form ref="formRef" :model="formValue" class="login-form">
        <n-form-item>
          <h2 class="login-title">Rust Vue Admin</h2>
        </n-form-item>
        <n-form-item label="用户账号" path="usercode">
          <n-input v-model:value="formValue.usercode" placeholder="请输入用户账号" />
        </n-form-item>
        <n-form-item label="密码" path="password">
          <n-input v-model:value="formValue.password" type="password" show-password-on="mousedown"
            placeholder="请输入密码" />
        </n-form-item>
        <n-form-item label="验证码" path="code">
          <n-space>
            <n-input v-model:value="formValue.code" placeholder="请输入验证码" />
            <n-image width="100" :src="srcRef" alt="刷新验证码" :preview-disabled="true" @click="getCaptcha()" />
          </n-space>
        </n-form-item>
        <div align="center">
          <n-button type="info" class="login-btn" @click="handleLoginClick"> 登录 </n-button>
        </div>
      </n-form>
    </n-space>
  </div>
</template>
<script setup lang="ts">
import { useUserStore } from '@/stores'
import { storage } from '@/toolkit'
import { TOKEN_KEY } from '@/types'
import { createDiscreteApi } from 'naive-ui'
import { AppAuth, GetCaptcha, Login } from '../api'
import router from '../router'

const { message: msg } = createDiscreteApi(['message'])
const userStore = useUserStore()

const srcRef = ref()
const formRef = ref()
const formValue = ref({
  usercode: 'admin', // 用户账号
  password: '123456', // 用户密码
  code: '', // 验证码
  app_code: '', // 应用授权码
  app_key: '', // 应用授权密钥
  uuid: '', // 加密后验证码
})
const authRef = ref({
  app_code: '03cv1mn38vhrkeli8jabfeej2', // 应用授权码
  app_key: '03cv1mogvphd42f4uyqamgfpc', // 应用授权密钥
})

// 登录
const handleLoginClick = () => {
  // 应用授权检验
  AppAuth(authRef.value).then((res) => {
    const { success, message, data } = res.data
    if (success) {
      userStore.setAppCode(data.app_code)
      Login(formValue.value).then((res1) => {
        const { success, message, data } = res1.data
        if (success) {
          msg.info('用户登录成功.')
          userStore.setToken(data.token)
          storage.set(TOKEN_KEY, data.token)
          router.push({ name: 'Index' })
        } else {
          msg.error(message)
        }
      })
    } else {
      msg.error(message)
    }
  })
}

// 获取验证码
const getCaptcha = () => {
  GetCaptcha().then((res) => {
    const { success, data } = res.data
    if (success) {
      formValue.value.uuid = data.uuid
      srcRef.value = data.img
    }
  })
}

onMounted(() => {
  getCaptcha()
})
</script>
<style lang="scss" scoped>
.login-wrap {
  min-height: 100vh;
  background: #293849;
  display: flex;
  justify-content: center;
  align-items: center;

  .login-form {
    width: 400px;
    background: #fff;
    border-radius: 5px;
    padding: 30px;
  }

  .login-btn {
    min-width: 100%;
  }
}
</style>
