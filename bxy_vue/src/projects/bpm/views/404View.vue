<template>
  <div class="div-center">
    <n-result status="404" title="404 访问错误" description="您访问的资源不存在或发生错误" size="huge">
      <template #footer>
        <n-button>确定</n-button>
      </template>
    </n-result>
  </div>
</template>
<style scope lang="scss">
.div-center {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>
