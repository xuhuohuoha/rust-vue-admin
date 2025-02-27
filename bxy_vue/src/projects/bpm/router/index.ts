import { storage } from '@/toolkit'
import { TOKEN_KEY } from '@/types'
import { createDiscreteApi } from 'naive-ui'
import { createRouter, createWebHashHistory, createWebHistory } from 'vue-router'
import { bpmStaticRoutes } from './routes'

// 是否启用 hash 模式
const isHash = !!import.meta.env.VITE_APP_USE_HASH

// 资源公共路径,需要以 /开头和结尾
const PATH = import.meta.env.VITE_PUBLIC_PATH

// 白名单
const WHITE_LIST = ['/login', '/404', '/redirect']

// 默认标题
const defaultTitle: string = import.meta.env.VITE_APP_TITLE

// 加载进度条
const { loadingBar } = createDiscreteApi(['loadingBar'])

const router = createRouter({
  history: isHash ? createWebHashHistory(`${PATH}`) : createWebHistory(`${PATH}`), //路由模式
  routes: bpmStaticRoutes, //静态路由
  scrollBehavior: () => ({ left: 0, top: 0 }), //刷新时，滚动条位置还原
})

router.beforeEach(async (to) => {
  loadingBar.start()
  if (to.path.startsWith('/src/projects') || to.path.startsWith('/undefined')) {
    return {
      name: 'Index',
    }
  }
  if (!storage.get(TOKEN_KEY)) {
    if (WHITE_LIST.includes(to.path)) {
      return true
    }
    // 过滤掉登录路由
    if (to.name !== 'Login') {
      return {
        name: 'Login',
      }
    }
  }
})

router.afterEach((to) => {
  const pageTitle = to.meta?.title
  if (pageTitle) {
    document.title = `${pageTitle} | ${defaultTitle}`
  } else {
    document.title = defaultTitle
  }
  setTimeout(() => {
    loadingBar.finish()
  }, 200)
})

// 加载进度条错误
router.onError(() => {
  loadingBar.error()
})

export default router
