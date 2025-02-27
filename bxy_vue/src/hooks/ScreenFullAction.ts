import screenfull from 'screenfull'

/**
 * 封装：整个页面、元素切换全屏
 */
export const screenfullAction = () => {
  const isFullScreen = ref(false)
  const handleFullScreen = () => {
    if (screenfull.isEnabled) {
      // 检测当前是否全屏，如果是全屏就退出，否则就全屏
      if (screenfull.isFullscreen) {
        screenfull.toggle()
        screenfull.exit()
      } else {
        // 进入全屏
        screenfull.toggle()
      }
    } else {
      alert('提示：不支持切换全屏。')
    }
  }
  // 点击当前元素进入全屏，一般为图片
  const handleFullscreenElement = (elementId: string) => {
    const element = document.getElementById(elementId) as HTMLElement // 指定全屏元素
    if (screenfull.isEnabled) {
      if (screenfull.isFullscreen) {
        screenfull.toggle(element)
        screenfull.exit()
      } else {
        screenfull.toggle(element)
      }
    } else {
      alert('提示：不支持切换全屏。')
    }
  }

  onMounted(() => {
    screenfull.onchange(() => {
      isFullScreen.value = !isFullScreen.value
    })
  })
  return { isFullScreen, handleFullScreen, handleFullscreenElement }
}
