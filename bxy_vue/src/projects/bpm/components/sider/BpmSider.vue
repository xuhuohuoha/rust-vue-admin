<template>
  <n-menu accordion :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions"
    key-field="guid" label-field="mname" children-field="children" @update:value="handleUpdateValue" />
  <b-modal :bxy></b-modal>
</template>
<script setup lang="ts" name="BpmSider">
import type { BModalProps, OperModel } from '@/components/modal/types'
import { useTabStore, useUserStore } from '@/stores'
import type { MenuTree } from '@/types'
import { createDiscreteApi, type MenuOption } from 'naive-ui'
const { message: msg } = createDiscreteApi(['message'])
const userStore = useUserStore()
const tabStore = useTabStore()
// 模态窗口属性
const bxy = ref<BModalProps>({
  preset: 'card',
  show: false,
  showIcon: false,
  positiveText: '确 认',
  negativeText: '取 消',
  title: '对话框',
  api: '',
  method: '',
})
// 是否折叠
const collapsed = ref(false)
// 功能菜单
const menuOptions = ref<any[]>([])
// 菜单事件
const handleUpdateValue = (key: string, item: MenuOption) => {
  switch (item.uio) {
    case 0:
      inner(key, item)
      break
    case 1:
      outer(key, item)
      break
    default:
      break
  }
}

const inner = (key: string, item: MenuOption) => {
  const { opt } = item
  switch (opt) {
    case 0:
      tabStore.openTab({
        buid: '',
        guid: key,
        name: key,
        title: item.mname as string,
        component: item.path,
        item: item as unknown as MenuTree,
      })
      break
    case 1:
      break
    case 2:
      initModal(item, 'D')
      break
    default:
      msg.error('打开方式配置错误，请联系管理员！')
      break
  }
}

const outer = (key: string, item: MenuOption) => {
  const { path } = item
  let url: string = path as string
  if (path === null) {
    msg.error('发现异常，请确认链接配置是否正确！')
  }
  if (
    url.substring(0, 7).toLowerCase() !== 'http://' &&
    url.substring(0, 8).toLowerCase() !== 'https://'
  ) {
    url = 'http://' + path
  }
  window.open(url, '_blank')
}

// 初始化模态框参数
const initModal = (item: any, model: OperModel) => {
  bxy.value.show = true
  bxy.value.title = item.alias
  bxy.value.component = item.path
  bxy.value.api = item.api
  bxy.value.method = item.method
  bxy.value.model = model
}

onMounted(() => {
  const { menus } = userStore.userState
  menuOptions.value = menus
})
</script>
