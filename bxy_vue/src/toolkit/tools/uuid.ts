import { v4 as uuidv4 } from 'uuid'

/**
 * 生成唯一标识
 * @returns
 */
export const uuid = () => {
  return uuidv4()
}
