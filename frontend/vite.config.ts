import { sentryVitePlugin } from "@sentry/vite-plugin";
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig(({ mode }) => {
  const plugins = [react()];
  
  // 只在启用 Sentry 且提供了认证和必要配置时添加插件
  if (process.env.VITE_ENABLE_SENTRY === 'true' && 
      process.env.SENTRY_AUTH_TOKEN && 
      process.env.VITE_SENTRY_ORG && 
      process.env.VITE_SENTRY_PROJECT) {
    plugins.push(sentryVitePlugin({
      org: process.env.VITE_SENTRY_ORG,
      project: process.env.VITE_SENTRY_PROJECT,
      telemetry: false // 禁用遥测
    }));
  }

  return {
    plugins,

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "shared": path.resolve(__dirname, "../shared"),
    },
  },

  server: {
    host: '0.0.0.0',  // 监听所有网络接口
    port: parseInt(process.env.FRONTEND_PORT || '3000'),
    open: false,
    allowedHosts: ['vibe.yangpu.dev'],
    proxy: {
      '/api': {
        target: `http://localhost:${process.env.BACKEND_PORT || '3001'}`,
        changeOrigin: true,
      },
    },
  },

  build: {
    sourcemap: true
  }
  };
})
