<template>
  <!-- 总布局 -->
  <n-layout class="b-layout">
    <!-- 头部 -->
    <n-layout-header bordered class="b-header">
      <slot name="header">header</slot>
    </n-layout-header>
    <n-layout-content>
      <slot name="content">content</slot>
    </n-layout-content>
    <!-- 底部 -->
    <n-layout-footer bordered class="b-footer">
      <slot name="footer">footer</slot>
    </n-layout-footer>
  </n-layout>
</template>

<script setup lang="ts" name="BLayout"></script>
<style scoped lang="scss">
.b-layout {
  height: 100%;

  .b-header {
    height: 82px;
  }

  .b-footer {
    height: 34px;
    line-height: 34px;
  }

  .b-content {
    width: 100vw;
  }
}
</style>
