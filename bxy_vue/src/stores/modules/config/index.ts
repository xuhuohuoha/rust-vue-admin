import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useConfigStore = defineStore(
  'configStore',
  () => {
    // 主题
    const theme = ref('')
    // 布局
    const layout = ref('L1')
    // 锁屏
    const lockscreen = ref<boolean>(false)
    // 增强
    const inverted = ref<boolean>(false)

    return { theme, layout, lockscreen, inverted }
  },
  {
    persist: true,
  },
)
