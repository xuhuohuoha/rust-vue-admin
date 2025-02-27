<template>
  <!-- 总布局 -->
  <n-layout class="b-layout">
    <!-- 头部 -->
    <n-layout-header bordered class="b-header" :inverted>
      <slot name="header">header</slot>
    </n-layout-header>
    <!-- 中间部分 -->
    <n-layout has-sider class="b-content">
      <n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :inverted>
        <slot name="sider">sider</slot>
      </n-layout-sider>
      <n-layout-content :inverted>
        <slot name="content">content</slot>
      </n-layout-content>
    </n-layout>
    <!-- 底部 -->
    <n-layout-footer bordered class="b-footer" :inverted>
      <slot name="footer">footer</slot>
    </n-layout-footer>
  </n-layout>
</template>

<script setup lang="ts" name="BLayout1">
// 是否开启增强
const inverted = ref<boolean>(false)
</script>
<style scoped lang="scss">
.b-layout {
  height: 100%;

  .b-header {
    height: 34px;
  }

  .b-footer {
    height: 34px;
    line-height: 34px;
  }

  .b-content {
    height: calc(99.9vh - 34px - 34px);
    width: 100vw;
  }
}
</style>
