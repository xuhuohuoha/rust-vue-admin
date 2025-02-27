<template>
  <!-- 总布局 -->
  <n-layout>
    <!-- 中间部分 -->
    <n-layout has-sider>
      <n-layout-sider bordered show-trigger collapse-mode="width" :collapsed-width="64" :width="240"
        :native-scrollbar="true" :inverted>
        <slot name="sider">sider</slot>
      </n-layout-sider>
      <n-layout class="b-layout">
        <!-- 头部 -->
        <n-layout-header bordered class="b-header" :inverted>
          <slot name="header">header</slot>
        </n-layout-header>
        <n-layout-content class="b-content">
          <slot name="content">content</slot>
        </n-layout-content>
        <!-- 底部 -->
        <n-layout-footer bordered class="b-footer" :inverted>
          <slot name="footer">footer</slot>
        </n-layout-footer>
      </n-layout>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts" name="BLayout1">
// 是否开启增强
const inverted = ref<boolean>(false)
</script>
<style scoped lang="scss">
.b-layout {

  .b-header {
    height: 64px;
  }

  .b-footer {
    height: 26px;
  }

  .b-content {
    height: calc(100vh - 64px - 26px);
  }
}
</style>
