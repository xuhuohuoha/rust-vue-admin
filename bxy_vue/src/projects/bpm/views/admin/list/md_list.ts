import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '主表模块',
    key: 'mcode',
  },
  {
    title: '从表模块',
    key: 'dcode',
  },
  {
    title: '主表关联字段',
    key: 'm_fields',
  },
  {
    title: '从表关联字段',
    key: 'd_fields',
  },
  {
    title: '备注说明',
    key: 'remark',
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
