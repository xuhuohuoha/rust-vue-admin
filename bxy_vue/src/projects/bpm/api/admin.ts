import { http } from '@/toolkit'
import { HttpMethod, type HttpRequestData } from '@/types'
import type { AxiosRequestConfig } from 'axios'

/**
 * 应用授权校验
 * @param params
 * @returns
 */
export const AppAuth = (params: HttpRequestData) => {
  return http.get<any>('/api/v1/comm/app_auth', params)
}

/**
 * 应用临时授权码校验
 * @param params
 * @returns
 */
export const AppAuthCode = (params: HttpRequestData) => {
  return http.get<any>('/api/v1/comm/app_auth_code', params)
}

/**
 * 登陆
 * @param params
 * @returns
 */
export const Login = (params: HttpRequestData) => {
  return http.post<any>('/api/v1/comm/login', params)
}

/**
 * 退出
 * @param params
 * @returns
 */
export const Logout = (params: HttpRequestData) => {
  return http.post<any>('/api/v1/comm/logout', params)
}

/**
 * 获取验证码
 * @returns
 */
export const GetCaptcha = () => {
  return http.get<any>('/api/v1/comm/get_captcha', {})
}

/**
 * 获取长度为10的盐
 * @returns
 */
export const GetSalt = () => {
  return http.get<any>('/api/v1/comm/get_salt', {})
}

/**
 * 获取指定长度的盐
 * @param params
 * @returns
 */
export const GetSaltLen = (params: HttpRequestData) => {
  return http.get<any>(`/api/v1/comm/get_salt_len/${params.len}`, params)
}

/**
 * 密码加密
 * @param params
 * @returns
 */
export const GetEncryptPassword = (params: HttpRequestData) => {
  return http.get<any>(`/api/v1/comm/get_encrypt_password`, params)
}

/**
 * 获取UUID
 * @returns
 */
export const GetUUID = () => {
  return http.get<any>('/api/v1/comm/get_uuid', {})
}

/**
 * 通用新增
 * @param params
 * @returns
 */
export const DevAdd = (params: HttpRequestData) => {
  return http.post<any>('/api/v1/core/add', params)
}

/**
 * 通用查询
 * @param api
 * @param params
 * @returns
 */
export const DevList = (api: string, params: HttpRequestData) => {
  return http.get<any>(api, params)
}

export const DevHttp = (
  method: string,
  url: string,
  params: HttpRequestData,
  config?: AxiosRequestConfig,
) => {
  if (method === HttpMethod.GET) {
    return http.get<any>(url, params, config)
  } else if (method === HttpMethod.PUT) {
    return http.put<any>(url, params, config)
  } else if (method === HttpMethod.POST) {
    return http.post<any>(url, params, config)
  } else if (method === HttpMethod.DELETE) {
    return http.delete<any>(url, params, config)
  } else {
    return http.get<any>(url, params, config)
  }
}
