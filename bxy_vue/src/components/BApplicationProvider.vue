<template>
  <!-- naive ui 全局配置 -->
  <n-config-provider class="cls-app-provider" :locale="zhCN" :date-locale="dateZhCN" :theme="theme">
    <!-- 全局样式 -->
    <n-global-style />
    <!-- naive ui 加载进度条 -->
    <n-loading-bar-provider>
      <!-- naive ui 对话框 -->
      <n-dialog-provider>
        <!-- naive ui 通知 -->
        <n-notification-provider>
          <!-- naive ui 消息 -->
          <n-message-provider>
            <slot></slot>
          </n-message-provider>
        </n-notification-provider>
      </n-dialog-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>
<script setup lang="ts" name="BApplicationProvider">
import { darkTheme, dateZhCN, lightTheme, useOsTheme, zhCN } from 'naive-ui'
import { computed } from 'vue'
// 系统主题
const osTheme = useOsTheme()
// 响应式主题
const theme = computed(() => (osTheme.value === 'dark' ? darkTheme : lightTheme))
</script>
<style scoped lang="scss">
.cls-app-provider {
  height: 100vh;
}
</style>
