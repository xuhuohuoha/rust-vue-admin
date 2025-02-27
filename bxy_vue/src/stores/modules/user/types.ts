import type { MenuTree } from '@/types'

/**
 * 当前用户信息
 */
export interface UserState {
  app_code: string // 应用编码
  u_id: string // 用户id
  username: string // 用户名
  email: string // 邮箱
  avatar: string // 头像
  token: string // token
  menus: MenuTree[] // 菜单
}
