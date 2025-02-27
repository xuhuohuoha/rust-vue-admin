import type { BModalProps } from '@/components/modal/types'
import { DevHttp } from '@/projects/bpm/api'
import type { FormInst, FormProps } from 'naive-ui'

/**
 * 普遍表单保存操作
 * @param bxy
 * @returns
 */
export const formAction = (bxy: BModalProps) => {
  const msg = useMessage()
  const formRef = ref<(FormProps & FormInst) | null>()
  const cache = ref<any>()
  const model = {}
  /**
   * 保存前操作
   * @returns
   */
  const onBeforeSave = (): Promise<boolean> => {
    return new Promise((resolve, reject) => {
      formRef.value
        ?.validate((valid) => {
          if (!valid) {
            resolve(true)
          } else {
            msg.error('数据验证失败，请根据要求填写数据！')
            resolve(false)
          }
        })
        .catch(() => {
          reject(false)
        })
    })
  }

  /**
   * 保存操作
   * @returns
   */
  const onSave = (): Promise<boolean> => {
    delete formRef.value?.model?.value // 防止递归
    Object.assign(model, formRef.value?.model)
    return new Promise((resolve, reject) => {
      const { method, api } = bxy
      DevHttp(method, api, model)
        .then((res) => {
          const { success, message } = res.data
          if (!success) {
            msg.error(message)
            resolve(false)
            return
          }
          msg.success('操作成功.')
          resolve(true)
        })
        .catch(() => {
          reject(false)
        })
    })
  }

  /**
   * 保存后操作
   * @returns
   */
  const onAfterSave = (): Promise<boolean> => {
    return new Promise((resolve) => {
      resolve(true)
    })
  }

  /**
   * 重置操作
   */
  const onReset = () => {
    if (formRef.value?.model) {
      // 重置成未修改前数据
      Object.assign(formRef.value?.model, cache.value)
    }
    msg.success('重置成功.')
  }

  // 初始化数据
  onMounted(() => {
    // 将传入的数据缓存
    if (JSON.stringify(bxy.data) !== null && JSON.stringify(bxy.data) !== undefined) {
      cache.value = bxy.data
      formRef.value!.model!.value = Object.assign({}, bxy.data)
    } else {
      cache.value = Object.assign({}, formRef.value!.model)
    }
  })

  return {
    formRef,
    onBeforeSave,
    onAfterSave,
    onSave,
    onReset,
  }
}
