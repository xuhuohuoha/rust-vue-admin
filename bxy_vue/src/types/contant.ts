// 全局常量定义

// Token 关键字定义
export const TOKEN_KEY = 'TOKEN__'

// 默认缓存期限为 7 天
export const DEFAULT_CACHE_TIME = 60 * 60 * 24 * 7

// 加密方法
export const VITE_SECRET_METHOD = import.meta.env.VITE_SECRET_METHOD

/**
 * 是否已退出
 */
export const IS_LOGOUT = 'LOGOUT__'

/**
 * 布局方式
 */
export const enum BLayout {
  Layout1 = 'L1',
  Layout2 = 'L2',
  Layout3 = 'L3',
  Layout4 = 'L4',
}

/**
 * 全局变量
 */
export interface Bxy {
  [prop: string]: any
}

/**
 * 预定义功能
 */
export enum PreFn {
  // 平台基础部分预定义为 F1XX
  Default = 'F100', // 默认，传参不做其他处理
  Add = 'F110', // 新增
  Edit = 'F120', // 编辑
  View = 'F130', // 浏览
  Remove = 'F140', // 逻辑删除（批量）
  RemoveId = 'F141', // 逻辑删除（指定ID）
  Delete = 'F150', // 物理删除（批量）
  DeleteId = 'F151', // 物理删除（指定ID）
  Clean = 'F159', // 清空
  Import = 'F160', // 导入
  Export = 'F170', // 导出
  Print = 'F180', // 打印
  // 业务扩展部分预定义为 F3XX
  OFFLINE = 'F301', // 强制下线
}
