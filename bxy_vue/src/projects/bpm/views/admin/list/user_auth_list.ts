import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '授权类型',
    key: 'atype',
    width: 80,
    render(row: any) {
      let tag = ''
      if (0 === row.atype) {
        tag = '包含'
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
      } else {
        tag = '排除'
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
      }
    },
  },
  ,
  {
    title: '授权方式',
    key: 'amethod',
  },
  {
    title: '备注说明',
    key: 'remark',
  },
]

export const action = {
  width: 150,
}
