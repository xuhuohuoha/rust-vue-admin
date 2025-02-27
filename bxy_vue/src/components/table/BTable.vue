<template>
  <n-layout class="content" has-sider>
    <n-layout-sider
      bordered
      :show-collapsed-content="false"
      collapse-mode="width"
      :width="240"
      v-show="showTree"
    >
      <!-- 导航树 -->
      <n-tree
        block-line
        cascade
        :data="treeOptions"
        :key-field="'guid'"
        label-field="label"
        expand-on-click
        checkable
        @update-checked-keys="handleTreeCheckKeys"
      />
    </n-layout-sider>
    <n-layout-content class="right">
      <!-- 查询头 -->
      <div v-show="showQuery" class="cls-query">
        <component
          :is="queryComponent"
          v-model:params="params"
          v-model:handle-query="handleQuery"
        ></component>
      </div>
      <!-- 列表工具条 -->
      <div class="cls-toolbar">
        <n-space class="left">
          <!-- <n-dropdown trigger="click">
            <n-button>启动流程</n-button>
          </n-dropdown> -->
          <n-button
            secondary
            v-for="(button, key) in toolbarButtons"
            :key="key"
            :type="button.style"
            size="medium"
            @click="handleClick(button)"
            >{{ button.mname }}</n-button
          >
        </n-space>
        <n-space justify="end" class="right">
          <n-button type="primary" ghost @click="handleReset">重置</n-button>
          <n-button type="primary" ghost @click="handleFullscreen()">{{
            isFullScreen ? '还原' : '全屏'
          }}</n-button>
        </n-space>
      </div>
      <!-- 列表 -->
      <n-data-table
        remote
        :columns="columnsRef"
        :data="dataRef"
        :pagination="pagination"
        :row-key="rowKey"
        @update-page="handlePageChange"
        @update:checked-row-keys="handleCheck"
        @update:page-size="handlePageSizeChange"
        striped
      ></n-data-table>
    </n-layout-content>
  </n-layout>
  <b-form-modal :bxy v-model:handle-query="handleQuery"></b-form-modal>
</template>
<script setup lang="ts" name="BTable">
import { screenfullAction } from '@/hooks/ScreenFullAction'
import { DevGetMenuAuthFn, DevGetRelateTree, DevHttp } from '@/projects/bpm/api'
import { useTabStore, useUserStore } from '@/stores'
import { array2tree } from '@/toolkit'
import { PreFn } from '@/types'
import { createDiscreteApi, NButton, type DataTableRowKey } from 'naive-ui'
import type { BModalProps, OperModel } from '../modal/types'
const { message: msg, dialog } = createDiscreteApi(['message', 'dialog'])
const { isFullScreen, handleFullscreenElement } = screenfullAction()
const userStore = useUserStore()
// 模态窗口属性
const bxy = ref<BModalProps>({
  preset: 'card',
  show: false,
  showIcon: false,
  positiveText: '确 认',
  negativeText: '取 消',
  title: '对话框',
  api: '',
  method: '',
})
// 关联树
const tree = ref<any>()
// 导航树
const treeOptions = ref<any[]>()
// 页签
const { activeTab, currentTab, modules, replaceAlias } = useTabStore()
// 动态加载查询组件
const queryComponent = computed(() => {
  const path = `${currentTab().item.query}.vue`
  return toRaw(modules[replaceAlias(path)])
})
// 是否显示导航树
const showTree = computed(() => {
  return treeOptions.value ? true : false
})
// 是否显示查询头
const showQuery = computed(() => {
  return currentTab().item?.query ? true : false
})
// 列表头
const columnsRef = reactive(new Array<any>())
// 列表数据
const dataRef = reactive(new Array<any>())
// 主键
const rowKey = (row: any) => row.id
// 参数
const params = reactive(new Map())
// 工具条功能按钮
const toolbarButtons = ref<Array<any>>([])
// 操作列功能按钮
const columnButtons = ref<Array<any>>([])
// 当前选中数据
const checkedRows = ref<Array<any>>([])
// 分页
const pagination = reactive({
  page: 1,
  pageSize: 10,
  itemCount: 10,
  showSizePicker: true,
  pageSizes: [10, 20, 50, 100],
  prefix({ itemCount }: any) {
    return `共 ${itemCount} 条`
  },
  onChange: (page: number) => {
    pagination.page = page
  },
  onUpdatePageSize: (pageSize: number) => {
    pagination.pageSize = pageSize
    pagination.page = 1
  },
})

// 翻页操作
const handlePageChange = (page: number) => {
  params.set('page_num', page)
  handleQuery()
}
// 改变分页大小
const handlePageSizeChange = (pageSize: number) => {
  params.set('page_size', pageSize)
  handleQuery()
}

// 通用查询功能
const handleQuery = () => {
  const { api, method } = currentTab().item
  DevHttp(method, api, Object.fromEntries(params)).then((res) => {
    const { success, message, data } = res.data
    if (!success) {
      msg.error(message)
    } else {
      dataRef.length = 0
      data.list.forEach((e: any) => {
        dataRef.push(e)
      })
      pagination.itemCount = data.total
    }
  })
}

// 重置
const handleReset = () => {
  params.clear()
  params.set('page_size', pagination.pageSize)
  params.set('page_num', pagination.page)
}

// 全屏
const handleFullscreen = () => {
  const { buid } = currentTab()
  handleFullscreenElement(buid)
}

// 按钮功能
const handleClick = (item: any) => {
  // 清空数据，防止缓存
  bxy.value.data = undefined
  switch (item.mtype) {
    case PreFn.Default:
      bxy.value.data = checkedRows.value[0]
      initModal(item, 'D')
      break
    case PreFn.Add:
      initModal(item, 'A')
      break
    case PreFn.Edit:
      bxy.value.data = checkedRows.value[0]
      initModal(item, 'E')
      break
    case PreFn.View:
      bxy.value.data = checkedRows.value[0]
      initModal(item, 'V')
      break
    case PreFn.Remove:
      handleRemoveOrDelete(item, 'M')
      break
    case PreFn.RemoveId:
      handleRemoveOrDelete(item, 'S')
      break
    case PreFn.Delete:
      handleRemoveOrDelete(item, 'M')
      break
    case PreFn.DeleteId:
      handleRemoveOrDelete(item, 'S')
      break
    case PreFn.Clean:
      handleClean(item)
      break
    case PreFn.OFFLINE:
      handleOffline(item)
      break
    default:
      break
  }
}
// 初始化模态框参数
const initModal = (item: any, model: OperModel) => {
  bxy.value.show = true
  bxy.value.title = item.alias
  bxy.value.component = item.path
  bxy.value.api = item.api
  bxy.value.method = item.method
  bxy.value.model = model
}
// 软删除或硬删除
const handleRemoveOrDelete = (item: any, type: string) => {
  if (checkedRows.value.length <= 0) {
    msg.warning('请选择需要删除的数据.')
    return false
  }
  let content = ''
  if ('S' === type) {
    content = '确定要删除当前行数据吗？'
  } else {
    content = '确定要删除所有已选中行数据吗？'
  }
  dialog.warning({
    title: '警告',
    content: content,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      let ids: string[] = []
      if (checkedRows.value.map((e) => e.id).length > 0) {
        ids = checkedRows.value.map((e) => e.id)
      } else {
        ids = checkedRows.value.map((e) => e.id_)
      }
      const params = {
        ids: ids,
      }
      // console.log('params:', JSON.stringify(params))
      const { api, method } = item
      DevHttp(method, api, params).then((res) => {
        const { success, message } = res.data
        if (success) {
          handleQuery()
          msg.success('数据删除成功.')
        } else {
          msg.error(message)
        }
      })
    },
  })
}
// 清空
const handleClean = (item: any) => {
  const content = '确定要清空所有数据吗？'
  dialog.warning({
    title: '警告',
    content: content,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      const { api, method } = item
      DevHttp(method, api, {}).then((res) => {
        const { success, message } = res.data
        if (success) {
          handleQuery()
          msg.success('数据已清空.')
        } else {
          msg.error(message)
        }
      })
    },
  })
}
// 强制下线
const handleOffline = (item: any) => {
  const { uname } = checkedRows.value[0]
  const content = `确定要将用户【${uname}】强制下线吗？`
  dialog.warning({
    title: '警告',
    content: content,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      let ids: string[] = []
      if (checkedRows.value.map((e) => e.id).length > 0) {
        ids = checkedRows.value.map((e) => e.id)
      } else {
        ids = checkedRows.value.map((e) => e.id_)
      }
      const params = {
        ids: ids,
      }
      const { api, method } = item
      DevHttp(method, api, params).then((res) => {
        const { success, message } = res.data
        if (success) {
          handleQuery()
          msg.success('强制下线用户成功.')
        } else {
          msg.error(message)
        }
      })
    },
  })
}
// 列表选中
const handleCheck = (rowKeys: DataTableRowKey[]) => {
  checkedRows.value = dataRef.filter((e) => {
    if (rowKeys.indexOf(e.id) >= 0 || rowKeys.indexOf(e.id_) >= 0) {
      return e
    }
  })
}
// 导航树选中
const handleTreeCheckKeys = (keys: Array<string>) => {
  if (keys.length === 0) return false
  params.set(tree.value.mfields, keys)
  handleQuery()
}
onMounted(async () => {
  // 分页
  params.set('page_size', pagination.pageSize)
  params.set('page_num', pagination.page)

  // 获取关联树
  const tree_params = {
    mcode: activeTab,
  }
  DevGetRelateTree(tree_params).then((res: any) => {
    const { success, data } = res.data
    if (success) {
      tree.value = data[0]
      const options = data[1]
      treeOptions.value = array2tree(options, 'guid', 'pguid')
      console.log(treeOptions.value)
    }
  })

  // 加载列表头
  if (currentTab().item?.cols) {
    const path = `${currentTab().item?.cols}.ts`
    const modulePromise = import.meta.glob<any>('@/**/*.ts')[replaceAlias(path)].apply(null)
    modulePromise.then((module: any) => {
      // 是否存在定义操作列，如果存在则采用自定义操作列
      let exitsActions = false
      module.columns.forEach((e: any) => {
        columnsRef.push(e)
        if (e.key === 'actions') {
          exitsActions = true
        }
      })
      if (!exitsActions) {
        // 处理权限功能按钮
        const width = module.action?.width ? module.action?.width : 185
        const op = {
          title: '操作',
          key: 'actions',
          fixed: 'right',
          width: width,
          render(row: any) {
            const ops = columnButtons.value.map((e) => {
              const { mname, style } = e
              return h(
                NButton,
                {
                  quaternary: true,
                  size: 'tiny',
                  type: style,
                  style: 'margin-left:5px;',
                  onClick: () => {
                    // 操作列仅可操作当前行数据
                    checkedRows.value.length = 0
                    checkedRows.value.push(row)
                    handleClick(e)
                  },
                },
                { default: () => mname },
              )
            })
            return ops
          },
        }
        columnsRef.push(op)
      }
    })
  }
  // 加载列表工具条
  const { app_code, u_id } = userStore.userState
  const fnParams = {
    app_code: app_code,
    mcode: activeTab,
    u_id: u_id,
  }
  DevGetMenuAuthFn(fnParams).then((res) => {
    const { success, message, data } = res.data
    if (success) {
      data.forEach((e: any) => {
        if (e.show === 0 || e.show === 2) {
          toolbarButtons.value?.push(e)
        }
        if (e.show === 1 || e.show === 2) {
          columnButtons.value?.push(e)
        }
      })
    } else {
      msg.error(message)
    }
  })
  // 查询
  handleQuery()
})
</script>
<style lang="scss" scoped>
/* 页面样式 */
.content {
  // height: 100vh;

  .right {
    margin-left: 5px;
  }
}

.cls-query {
  padding-top: 3px;
}

/* 列表工具条样式 */
.cls-toolbar {
  display: flex;
  margin: 5px 0 5px 0;
  justify-content: flex-end;

  .left {
    width: 70%;
    justify-content: flex-start;
  }

  .right {
    width: 30%;
    justify-content: flex-end;
  }
}
</style>
