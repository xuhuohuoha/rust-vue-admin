<template>
  <div class="div-center">
    <n-result status="403" title="403 禁止访问" description="您没有权限访问" size="huge">
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
