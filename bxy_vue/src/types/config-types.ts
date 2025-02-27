/**
 * 全局状态
 */
export interface ConfigState {
  /**
   * 是否显示侧边栏
   */
  showSidebar: boolean
  /**
   * 是否显示头部
   */
  showHeader: boolean
  /**
   * 是否显示底部
   */
  showFooter: boolean
  /**
   * 是否显示面包屑
   */
  showBreadcrumb: boolean
  /**
   * 是否显示标签页
   */
  showTabs: boolean
  /**
   * 是否显示全屏
   */
  showFullScreen: boolean
  /**
   * 是否显示搜索
   */
  showSearch: boolean
  /**
   * 是否显示通知
   */
  showNotice: boolean
  /**
   * 是否显示语言
   */
  showLanguage: boolean
  /**
   * 是否显示锁屏
   */
  showLock: boolean
  /**
   * 是否显示反转
   */
  showInvert: boolean
  /**
   * 是否显示主题
   */
  showTheme: string
}
