/**
 * 模态窗口操作类型
 * A：新增
 * E：编辑
 * V：查看
 * R：报表
 * S：单条
 * M：多条
 * D：默认
 */
export type OperModel = 'A' | 'E' | 'V' | 'R' | 'D' | 'S' | 'M'

/**
 * 模态窗口属性
 */
export interface BModalProps {
  // 展示模态框
  show: boolean
  // 是否显示icon
  showIcon: boolean
  // 积极显示的文本
  positiveText: string
  // 消极显示的文本
  negativeText: string
  // 模态框显示的标题
  title: string
  // 预设类型
  preset: 'dialog' | 'card'
  // 表单模式
  model?: OperModel
  // 组件
  component?: string
  // Api 接口
  api: string
  // Api 请求方式
  method: string
  // 数据
  data?: object
}
