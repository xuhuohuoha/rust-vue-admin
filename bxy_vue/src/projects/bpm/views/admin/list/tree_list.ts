import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '树名称',
    key: 'tname',
  },
  {
    title: '关联模块',
    key: 'mcode',
  },
  {
    title: '树关联字段',
    key: 'tfields',
  },
  {
    title: '模块关联字段',
    key: 'mfields',
  },
  {
    title: '状态',
    key: 'status',
    width: 80,
    render(row: any) {
      let tag = ''
      if ('0' === row.status) {
        tag = '停用'
        return h(
          NTag,
          {
            type: 'error',
            size: 'small',
            bordered: false,
          },
          {
            default: () => tag,
          },
        )
      } else {
        tag = '正常'
        return h(
          NTag,
          {
            type: 'info',
            size: 'small',
            bordered: false,
          },
          {
            default: () => tag,
          },
        )
      }
    },
  },
]
