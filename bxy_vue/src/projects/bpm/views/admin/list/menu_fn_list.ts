import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '功能名称',
    key: 'mname',
    width: 100,
  },
  {
    title: '功能类别',
    key: 'mtype',
    width: 100,
  },
  {
    title: '路由组件',
    key: 'path',
    width: 200,
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: 'Api',
    key: 'api',
    width: 220,
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: '功能状态',
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
  width: 190,
}
