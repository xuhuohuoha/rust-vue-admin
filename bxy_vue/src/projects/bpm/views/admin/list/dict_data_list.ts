import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '字典代码',
    key: 'guid',
    width: 100,
  },
  {
    title: '字典名称',
    key: 'att',
    width: 100,
  },
  {
    title: '字典类别',
    key: 'dname',
    width: 180,
  },
  {
    title: '备注说明',
    key: 'remark',
    width: 200,
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
  width: 150,
}
