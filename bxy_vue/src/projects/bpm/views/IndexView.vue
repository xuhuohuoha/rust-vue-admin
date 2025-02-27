<template>
  <n-watermark content="Blunka BPM" cross fullscreen :font-size="16" :line-height="16" :width="384" :height="384"
    :x-offset="12" :y-offset="60" :rotate="-15" />
  <!-- 系统默认加载页面 -->
  <b-layout1>
    <template #header>
      <bpm-header></bpm-header>
    </template>
    <template #sider>
      <bpm-sider></bpm-sider>
    </template>
    <template #content>
      <b-tab></b-tab>
    </template>
    <template #footer>
      <b-footer></b-footer>
    </template>
  </b-layout1>
</template>
<script setup lang="ts">
import { useTabStore, useUserStore } from '@/stores'
import { createDiscreteApi } from 'naive-ui'
import { DevGetMenu } from '../api'

const { message: msg } = createDiscreteApi(['message'])
const userStore = useUserStore()
const tabStore = useTabStore()
onBeforeMount(() => {
  // 重新加载则清空已打开Tab缓存
  tabStore.cleanTab()
})
onMounted(() => {
  // 加载菜单
  const { app_code, u_id } = userStore.userState
  const params = {
    app_code: app_code,
    mcode: '',
    u_id: u_id,
  }

  DevGetMenu(params).then((res) => {
    const { success, message, data } = res.data
    if (success) {
      userStore.setMenus(data)
    } else {
      msg.error(message)
    }
  })
})
</script>
<style scoped lang="scss"></style>
