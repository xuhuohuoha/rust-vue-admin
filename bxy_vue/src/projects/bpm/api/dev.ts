import { http } from '@/toolkit'
import type { HttpRequestData } from '@/types'

/**
 * 获取当前用户授权菜单
 * @param params
 * @returns
 */
export const DevGetMenu = (params: HttpRequestData) => {
  return http.get<any>('/api/v1/core/menu/find_menu_by_uid', params)
}

/**
 * 获取指定菜单功能
 * @param params
 * @returns
 */
export const DevGetMenuFn = (params: HttpRequestData) => {
  return http.get<any>('/api/v1/core/menu/find_menu_fn', params)
}

/**
 * 获取指定菜单授权功能
 * @param params
 * @returns
 */
export const DevGetMenuAuthFn = (params: HttpRequestData) => {
  return http.get<any>('/api/v1/core/menu/find_fn_by_uid', params)
}

/**
 * 获取关联树
 * @param params
 * @returns
 */
export const DevGetRelateTree = (params: HttpRequestData) => {
  return http.get<any>(`/api/v1/core/tree/find_by_mcode_uid/${params.mcode}`, [])
}

/**
 * 通过动态脚本获取所有数据
 * @param sign
 * @param params
 * @returns
 */
export const DevGetSelect = async (sign: string, params: Array<any>): Promise<any[]> => {
  const msg = useMessage()
  const _params = { sign: sign, params: params }
  const options = await http.get<any>('/api/v1/core/all', _params).then((res) => {
    const { success, message, data } = res.data
    if (success) {
      return data
    } else {
      msg.error(message)
      return []
    }
  })
  return options
}
