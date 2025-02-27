import { DEFAULT_CACHE_TIME } from "@/types/contant"

/**
 * 创建本地缓存对象
 * @param param
 */
const createWebStorage = (prefixKey = '', storage: Storage) => {
  /**
   * 本地缓存类
   */
  const Storage = class {
    public storage
    public prefixKey

    constructor(prefixKey: string, storage: Storage) {
      this.prefixKey = prefixKey
      this.storage = storage
    }

    public getKey(key: string) {
      return `${this.prefixKey}${key}`.toUpperCase()
    }

    /**
     * 设置缓存
     * @param key       缓存键
     * @param value     缓存值
     * @param expire    有效期
     */
    public set(key: string, value: unknown, expire: number | null = DEFAULT_CACHE_TIME) {
      const stringData = JSON.stringify({
        value,
        expire: expire !== null ? new Date().getTime() + expire * 1000 : null,
      })
      this.storage.setItem(this.getKey(key), stringData)
    }

    /**
     * 读取缓存
     * @param key   缓存键
     * @param def   默认值
     * @returns
     */
    public get(key: string, def: unknown = null) {
      const item = this.storage.getItem(this.getKey(key))
      if (item) {
        try {
          const data = JSON.parse(item)
          const { value, expire } = data
          // 在有效期内直接返回
          if (expire === null || expire >= Date.now()) {
            return value
          }
          this.remove(key)
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        } catch (e) {
          return def
        }
      }
      return def
    }

    /**
     * 从缓存删除指定项
     * @param key   缓存键
     */
    public remove(key: string) {
      this.storage.removeItem(this.getKey(key))
    }

    /**
     * 清空所有缓存
     */
    public clear(): void {
      this.storage.clear()
    }

    /**
     * 设置cookie
     * @param name      cookie名称
     * @param value     cookie值
     * @param expire    过期时间
     *
     * 如不设置过期时间，默认浏览器关系自动删除
     */
    public setCookie(name: string, value: unknown, expire: number | null = DEFAULT_CACHE_TIME) {
      document.cookie = `${this.getKey(name)}=${value};Max-Age=${expire}`
    }

    /**
     * 根据名字获取cookie值
     * @param name
     * @returns
     */
    public getCooke(name: string): string {
      const cookieArr = document.cookie.split('; ')
      for (let i = 0, length = cookieArr.length; i < length; i++) {
        const kv = cookieArr[i].split('=')
        if (kv[0] === this.getKey(name)) {
          return kv[1]
        }
      }
      return ''
    }

    /**
     * 根据名字删除指定cookie
     * @param key
     */
    public removeCookie(key: string) {
      this.setCookie(key, 1, -1)
    }

    /**
     * 清空cookie，使所有cookie失效
     */
    public clearCookie(): void {
      const keys = document.cookie.match(/[^ =;]+(?==)/g)
      if (keys) {
        for (let i = keys.length; i--; ) {
          document.cookie = keys[i] + '=0;expire=' + new Date(0).toUTCString()
        }
      }
    }
  }
  return new Storage(prefixKey, storage)
}

/**
 * 默认 localStorage
 */
export const storage = createWebStorage('', window.localStorage)

export const sstorage = createWebStorage('', window.sessionStorage)
