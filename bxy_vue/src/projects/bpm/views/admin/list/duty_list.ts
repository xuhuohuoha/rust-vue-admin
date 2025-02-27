import { NTag } from "naive-ui"

export const columns = [
  {
    type: 'selection',
  },
  {
    title: '职务名称',
    key: 'dname',
  },
  {
    title: '职务状态',
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
  {
    title: '属性',
    key: 'att',
  },
  {
    title: '备注说明',
    key: 'remark',
  },
]
