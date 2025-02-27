import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '应用代码',
    key: 'guid',
    width: 200,
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: '应用名称',
    key: 'app_name',
    width: 200,
  },
  {
    title: '授权码',
    key: 'app_code',
    width: 200,
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: '授权密钥',
    key: 'app_key',
    width: 200,
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: '应用状态',
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
