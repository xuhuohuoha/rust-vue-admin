import type { VNode } from 'vue'

/**
 * 菜单树
 */
export interface MenuTree {
  guid: string // 菜单代码
  pguid: string // 上级菜单代码
  ord: number // 排序号
  mname: string // 菜单名称
  mtype: string // 菜单类型
  uio: number // 路由方式
  path: string // 路由路径
  api: string // api接口
  method: string // http请求类型
  opt: number // 组件打开方式
  alias: string // 显示别名
  tbl: string // 表格类型
  query: string // 查询头路径
  qscript: string // 查询头脚本
  cols: string // 列表头路径
  cscript: string // 列表头脚本
  icon?: string | VNode | (() => VNode) // 图标
  style: string // 样式
  show: number // 显示区域
  comp: string // 组件
  visible: number //
  valid: boolean
  children?: MenuTree[] | undefined
  [prop: string]: any
}
