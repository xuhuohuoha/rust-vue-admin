import { renderIcon } from '@/toolkit/tools/icon'
import { type MenuTree } from '@/types'
import type { MenuOption } from 'naive-ui'
import { defineStore } from 'pinia'
import type { UserState } from './types'

export const useUserStore = defineStore(
  'userStore',
  () => {
    const userState = ref<UserState>({
      app_code: '', // 应用编码
      u_id: '', // 用户 ID
      username: '', // 用户名
      email: '', // 邮箱
      avatar: '', // 头像
      token: '', // token
      menus: [], // 菜单
    })
    const setToken = (token: string) => {
        userState.value.token = token
    }
    const setAppCode = (app_code: string) => {
        userState.value.app_code = app_code
    }
    const setMenus = async (menus: MenuTree[]) => {
      userState.value.menus.length = 0
      if (userState.value.menus) {
        await removeEmptyChildren(menus).then((menu) => {
          userState.value.menus.push(...menu)
        })
      }
    }
    const removeEmptyChildren = async (menus: MenuTree[]): Promise<MenuTree[]> => {
      return Promise.all(
        menus.map(async (menu) => {
          const { children, icon, ...rest } = menu
          const newChildren =
            children && children.length > 0 ? await removeEmptyChildren(children) : undefined
          return {
            ...(typeof icon === 'string' ? { icon: renderIcon(icon) } : { icon }),
            ...rest,
            ...(newChildren ? { children: newChildren } : {}),
          }
        }),
      )
    }
    const toMenuOption = (menus: any[]): MenuOption[] => {
      return menus.map((menu) => {
        const { children, ...rest } = menu
        // 如果 children 存在且不是空数组，则递归处理 children
        const newChildren = children && children.length > 0 ? toMenuOption(children) : undefined
        // 返回一个新对象，如果 newChildren 为 undefined，则不包含 children 属性
        return {
          item: menu,
          ...rest,
          ...(newChildren ? { children: newChildren } : {}),
        }
      })
    }
    return { userState, setToken, setAppCode, setMenus, toMenuOption }
  },
  {
    persist: true,
  },
)
