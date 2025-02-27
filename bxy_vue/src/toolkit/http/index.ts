import { type HttpResponseData } from '@/types'
import type { AxiosRequestConfig } from 'axios'
import qs from 'qs'
import instance from './axios'

export const http = {
  get<T>(url: string, data?: object, config?: AxiosRequestConfig): Promise<HttpResponseData<T>> {
    config = { paramsSerializer: (params) => qs.stringify(params, { arrayFormat: 'repeat' }) }
    return instance.get(url, {
      params: data,
      ...config,
    })
  },

  post<T>(url: string, data?: object, config?: AxiosRequestConfig): Promise<HttpResponseData<T>> {
    return instance.post(url, data, config)
  },

  put<T>(url: string, data?: object, config?: AxiosRequestConfig): Promise<HttpResponseData<T>> {
    return instance.put(url, data, config)
  },

  delete<T>(url: string, data?: object, config?: AxiosRequestConfig): Promise<HttpResponseData<T>> {
    return instance.delete(url, { data, ...config })
  },
}
