import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import path, { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  base: './', // 指定项目根目录
  publicDir: 'public', // 静态资源目录
  plugins: [
    vue(),
    vueJsx(),
    AutoImport({
      imports: [
        'vue',
        {
          'naive-ui': ['useDialog', 'useMessage', 'useNotification', 'useLoadingBar'],
        },
      ],
    }),
    Components({
      dirs: ['./src/components', './src/projects/bpm/components', './src/projects/tcm/components', './src/projects/tcms/components',],
      resolvers: [NaiveUiResolver()],
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      '@project': path.resolve(__dirname, 'src/projects'),
      '@bpm': path.resolve(__dirname, 'src/projects/bpm'),
      '@tcm': path.resolve(__dirname, 'src/projects/tcm'),
      '@tcms': path.resolve(__dirname, 'src/projects/tcms'),
    },
  },
  server: {
    host: '127.0.0.1', // 指定服务器主机名
    port: 3000, // 指定服务器端口
    hmr: true, // 开启热更新
    open: true, // 在服务器启动时自动在浏览器中打开应用程序
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8080', //后台端口
        changeOrigin: true,
        // rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        api: 'modern-compiler',
      },
    },
  },
  build: {
    assetsDir: 'assets',
    outDir: '../dist', // 指定输出路径
    cssCodeSplit: true,
    sourcemap: false,
    assetsInlineLimit: 4096, //小于此阈值的导入或引用资源将内联为 base64 编码，以避免额外的 http 请求
    emptyOutDir: true, //Vite 会在构建时清空该目录
    terserOptions: {
      compress: {
        keep_infinity: true, // 防止 Infinity 被压缩成 1/0，这可能会导致 Chrome 上的性能问题
        drop_console: true, // 生产环境去除 console
        drop_debugger: true, // 生产环境去除 debugger
      },
      format: {
        comments: false, // 删除注释
      },
    },
    rollupOptions: {
      //自定义底层的 Rollup 打包配置
      input: {
        bpm: resolve(__dirname, 'src/projects/bpm/index.html'),
        tcm: resolve(__dirname, 'src/projects/tcm/index.html'),
        tcms: resolve(__dirname, 'src/projects/tcms/index.html'),
      },
      // buildEnd: buildEndFn(npm_config_page),
      output: {
        assetFileNames: '[ext]/[name]-[hash].[ext]', //静态文件输出的文件夹名称
        chunkFileNames: 'js/[name]-[hash].js', //chunk包输出的文件夹名称
        entryFileNames: 'js/[name]-[hash].js', //入口文件输出的文件夹名称
        compact: true,
        manualChunks: (id) => {
          if (id.includes('node_modules')) {
            return id.toString().split('node_modules/')[1].split('/')[0].toString() // 拆分多个vendors
          }
        },
      },
    },
  },
})
