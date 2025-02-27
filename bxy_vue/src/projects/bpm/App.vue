<template>
  <b-application-provider>
    <router-view v-slot="{ Component }">
      <component :is="Component" />
    </router-view>
  </b-application-provider>
</template>
<script setup lang="ts">

</script>
<style scoped lang="scss"></style>
