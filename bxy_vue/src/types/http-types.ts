/**
 * 常用 ContentType类型
 */
export enum HttpContentType {
  // json
  JSON = 'application/json',
  // text
  TEXT = 'text/plain',
  // form-data
  FORM_URLENCODED = 'application/x-www-form-urlencoded',
  // form-data 上传
  FORM_DATA = 'multipart/form-data',
}

/**
 * 统一请求封装：JSON 格式参数
 */
export interface HttpRequestData {
  [prop: string]: any
}

/**
 * 统一返回封装：JSON 格式参数
 */
export interface HttpResponseData<T> {
  // 请求是否成功
  success: boolean
  // 状态码
  code: number
  // 返回消息
  message: string
  // 返回数据
  data: T
  // 时间戳
  timestamp: number
}

/**
 * Http 请求类型
 */
export enum HttpMethod {
  GET = 'GET',
  PUT = 'PUT',
  POST = 'POST',
  DELETE = 'DELETE',
  HEAD = 'HEAD',
  PATCH = 'PATCH',
  OPTIONS = 'OPTIONS',
  CONNECT = 'CONNECT',
  TRACE = 'TRACE',
}

/**
 * Http 请求实体模型
 */
export interface HttpEntity {
  id: string | number
  [prop: string]: any
}
