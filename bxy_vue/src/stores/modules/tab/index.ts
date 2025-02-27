import { defineStore } from 'pinia'
import { v4 as uuidv4 } from 'uuid'
import type { TabOption } from './types'

// 别名集合
const aliasMap = new Map()
aliasMap.set('@', '/src')
aliasMap.set('@projects', '/src/projects')
aliasMap.set('@bpm', '/src/projects/bpm')
aliasMap.set('@tcms', '/src/projects/tcms')

// 全局导入
const files = import.meta.glob<any>([
  '/src/projects/bpm/views/**/*.vue',
  '/src/projects/bpm/views/**/*.ts',
  '/src/projects/tcms/views/**/*.vue',
  '/src/projects/tcms/views/**/*.ts',
])

export const useTabStore = defineStore(
  'tabStore',
  () => {
    // 当前页签
    const activeTab = ref<string>('')
    // 页签集合
    const panels = ref<Array<TabOption>>([])
    // 是否新增
    const addable = ref<boolean>(true)
    // 是否关闭
    const closeable = ref<boolean>(true)
    // 是否全屏
    const fullscreen = ref<boolean>(false)
    // 模块
    const modules: {
      [key: string]: any
    } = {}
    for (const path in files) {
      files[path]().then((mod: any) => {
        modules[`${path}`] = mod.default
      })
    }
    // 初始化页签
    const initTabs = () => {
      // panels.value.length = 0
      // const tabs = []
      // panels.value?.push(...tabs)
      // activeTab.value = '6'
    }
    // 路径中的别名替换
    const replaceAlias = (path: string) => {
      if (path.startsWith('@')) {
        const paths = path.split('/')
        if (aliasMap.has(paths[0])) {
          paths[0] = aliasMap.get(paths[0])
          path = paths.join('/')
        }
      }
      return path
    }
    // 打开新页签
    const openTab = (tab: TabOption) => {
      const nameIndex = panels.value.findIndex((item: TabOption) => item.guid === tab.guid)
      if (!~nameIndex) {
        // 生成组件
        const path = replaceAlias(`${tab.component}.vue`)
        tab.component = toRaw(modules[path])
        tab.closeable = true
        tab.buid = uuidv4()
        tab.item = tab.item
        panels.value.push(tab)
      }
      activeTab.value = tab.guid
    }
    // 关闭页签
    const closeTab = (name: string) => {
      if (
        panels.value.length > 0 &&
        panels.value.find((item: TabOption) => item.name === name)?.closeable
      ) {
        const nameIndex = panels.value.findIndex((item: TabOption) => item.name === name)
        if (!~nameIndex) {
          return
        }
        panels.value.splice(nameIndex, 1)
        if (name === activeTab.value) {
          if (panels.value?.length > 0) {
            activeTab.value = panels.value[Math.min(nameIndex, panels.value?.length - 1)].name
          }
        }
      }
    }
    // 当前页签
    const currentTab = () => {
      return panels.value.filter((item: TabOption) => item.guid === activeTab.value)[0]
    }
    // 清空标签
    const cleanTab = () => {
      panels.value.length = 0
    }
    return {
      activeTab,
      panels,
      addable,
      closeable,
      fullscreen,
      modules,
      initTabs,
      openTab,
      closeTab,
      currentTab,
      cleanTab,
      replaceAlias,
    }
  },
  {
    persist: true,
  },
)
