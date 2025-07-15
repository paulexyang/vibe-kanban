# Sentry 配置指南

## 当前状态

已将 Sentry 配置改为可选，不再默认发送数据到 bloop-ai 的账户。

## 禁用 Sentry（默认）

不需要任何操作，Sentry 默认是禁用的。

## 使用自己的 Sentry

### 选项 1：使用 Sentry 官方服务

1. 在 [sentry.io](https://sentry.io) 注册账户（有免费套餐）
2. 创建项目，获取 DSN
3. 配置环境变量

### 选项 2：自托管 Sentry（推荐）

1. **使用 Docker 部署**：
```bash
# 克隆官方自托管仓库
git clone https://github.com/getsentry/self-hosted.git
cd self-hosted

# 运行安装脚本
./install.sh

# 启动服务
docker compose up -d
```

2. **访问管理界面**：
- 打开 http://localhost:9000
- 创建管理员账户
- 创建组织和项目
- 获取项目的 DSN

3. **配置 Vibe Kanban**：

**后端配置** (`backend/.env`):
```env
SENTRY_DSN=http://your-key@localhost:9000/1
```

**前端配置** (`frontend/.env`):
```env
VITE_ENABLE_SENTRY=true
VITE_SENTRY_DSN=http://your-key@localhost:9000/1
VITE_SENTRY_ORG=your-org
VITE_SENTRY_PROJECT=vibe-kanban

# 如果需要上传 source maps（可选）
SENTRY_AUTH_TOKEN=your-auth-token
```

## Sentry 功能说明

### 错误追踪
- 自动捕获未处理的异常
- 记录错误堆栈、用户信息、环境变量
- 支持错误分组和趋势分析

### 性能监控
- 追踪 API 响应时间
- 监控数据库查询性能
- 前端页面加载性能

### 告警规则
- 错误数量阈值告警
- 性能降级告警
- 自定义告警规则

## 隐私保护建议

1. **自托管部署**：完全控制数据
2. **数据脱敏**：配置 Sentry 过滤敏感信息
3. **最小化数据收集**：只收集必要的错误信息

```javascript
// 前端数据脱敏示例
Sentry.init({
  beforeSend(event) {
    // 过滤敏感信息
    if (event.request?.cookies) {
      delete event.request.cookies;
    }
    return event;
  }
});
```

## 相关文件

- `frontend/src/main.tsx` - 前端 Sentry 初始化
- `backend/src/main.rs` - 后端 Sentry 初始化
- `frontend/vite.config.ts` - 构建时 Sentry 插件配置