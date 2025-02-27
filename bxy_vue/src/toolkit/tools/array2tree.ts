/**
 * 将数组数据转为树型结构
 * @param data 数组数据
 * @param code 代码字段
 * @param pcode 父级字段
 * @returns
 */
export const array2tree = <T extends { [key: string]: any }>(
  data: T[],
  code: keyof T,
  pcode: keyof T,
): T[] => {
  const map = new Map<any, T & { children?: T[] }>()
  const roots: (T & { children?: T[] })[] = []

  // 初始化 map
  data.forEach((node) => {
    map.set(node[code], { ...node, children: [] as T[] })
  })

  // 构建树结构
  data.forEach((node) => {
    const mappedNode = map.get(node[code])
    if (mappedNode) {
      if (node[code] === node[pcode]) {
        roots.push(mappedNode)
      } else {
        const parentNode = map.get(node[pcode])
        if (parentNode) {
          parentNode.children!.push(mappedNode)
        }
      }
    }
  })

  // 移除空子属性
  const removeEmptyChildren = (nodes: (T & { children?: T[] })[]): T[] => {
    return nodes.map((node: any) => {
      const { children, ...rest } = node
      // 如果 children 存在且不是空数组，则递归处理 children
      const newChildren =
        children && children.length > 0 ? removeEmptyChildren(children) : undefined
      // 返回一个新对象，如果 newChildren 为 undefined，则不包含 children 属性
      return { ...rest, ...(newChildren ? { children: newChildren } : {}) }
    })
  }

  const tree = removeEmptyChildren(roots)

  return tree
}
