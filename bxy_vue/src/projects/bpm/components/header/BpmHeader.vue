<template>
  <div class="header">
    <n-flex>
      <n-flex :style="'width:240px'">
        <BpmLogo />
      </n-flex>
      <n-flex>
        <n-input placeholder="搜索" style="width: 200px;"></n-input>
        <n-button quaternary>通知</n-button>
        <n-button quaternary @click="handleFullScreen()">{{ isFullScreen ? '还原' : '全屏' }}</n-button>
        <n-button quaternary>锁屏</n-button>
        <n-dropdown :options="options" @select="handleSelect">
          <n-avatar>
          </n-avatar>
        </n-dropdown>
      </n-flex>
    </n-flex>
  </div>
</template>
<script setup lang="ts" name="BpmHeader">
import { screenfullAction } from '@/hooks/ScreenFullAction'
import { useTabStore } from '@/stores'
import { storage } from '@/toolkit'
import router from '@bpm/router'

const { isFullScreen, handleFullScreen } = screenfullAction()
const { cleanTab } = useTabStore()

const options = [
  {
    label: '个人资料',
    key: 'information',
  },
  {
    label: '个人设置',
    key: 'profile',
  },
  {
    label: '退出登录',
    key: 'logout',
  }
]
const handleSelect = (key: string | number) => {
  // 1）用户信息
  // 2）退出登陆
  switch (key) {
    case 'profile':
      break
    case 'logout':
      storage.clear()
      cleanTab()  // 清空已打开Tab
      router.push({ name: 'Login' }) // 跳转登录页
      break
    default:
      break
  }
}
</script>
<style scoped lang="scss">
.header {
  display: flex;
  align-items: center;
}
</style>
