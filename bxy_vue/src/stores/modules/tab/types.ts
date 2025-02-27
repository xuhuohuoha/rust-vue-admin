import type { MenuTree } from '@/types'

export interface TabOption {
  // 模块 guid
  guid: string
  // 页签显示标题
  title: string
  // 页签名称
  name: string
  // 页签组件
  component: any
  // 页签是否可以关闭
  closeable?: boolean
  // 组件 uuid
  buid: string
  // 扩展属性
  item: MenuTree
  // 其他数据
  [key: string]: any
}
