<template>
  <div class="div-center">
    <n-result status="500" title="500 服务端错误" description="服务端发生错误" size="huge">
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
