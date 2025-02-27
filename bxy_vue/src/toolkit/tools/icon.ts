import * as icons from '@vicons/ionicons5'
import { NIcon } from 'naive-ui'
import { h } from 'vue'

/**
 * 渲染图标
 * @param icon 图标名称
 * @param props 图标属性
 */
export const renderIcon = (icon: string, props = { size: 20, color: '#2DCE89' }) => {
  if ('' === icon) {
    return () => h('div', null, [])
  }
  const component = icons[icon as keyof typeof icons]
  if (component) {
    return () => h(NIcon, props, { default: () => h(component) })
  }
}
