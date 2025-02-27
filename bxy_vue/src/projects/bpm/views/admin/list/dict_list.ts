import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '类别代码',
    key: 'guid',
    width: 120,
  },
  {
    title: '类别名称',
    key: 'dname',
    width: 120,
  },
  {
    title: '属性',
    key: 'att',
    width: 150,
  },
  {
    title: '备注说明',
    key: 'remark',
    width: 300,
    ellipsis: {
      tooltip: true,
    },
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

export const action = {
  width: 120,
}
