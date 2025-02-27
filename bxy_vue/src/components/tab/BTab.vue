<template>
  <n-tabs v-model:value="activeTab" type="card" pane-class="tab-pane-class" tab-style="min-width: 80px;"
    :closable="closeable" @close="tabStore.closeTab">
    <n-tab-pane v-for="panel in panels" :key="panel.guid" :name="panel.name" :tab="panel.title" display-directive="show"
      :closable="panel.closeable">
      <b-tab-pane v-bind="panel"></b-tab-pane>
    </n-tab-pane>
  </n-tabs>
</template>
<script setup lang="ts" name="BTab">
import { useTabStore } from '@/stores'
import { storeToRefs } from 'pinia'

const tabStore = useTabStore()
const { activeTab, panels, closeable } = storeToRefs(tabStore)
</script>

<style lang="scss" scoped>
.tab-pane-class {
  padding: 0px;
  height: 100vh;
}
</style>
