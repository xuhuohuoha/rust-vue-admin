import { NTag } from 'naive-ui'

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '用户账号',
    key: 'ucode',
  },
  {
    title: '用户名称',
    key: 'uname',
  },
  {
    title: '电子邮箱',
    key: 'email',
  },
  {
    title: '电话',
    key: 'phone',
  },
  {
    title: '最近一次登录时间',
    key: 'last_login_time',
  },
  {
    title: '用户状态',
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
