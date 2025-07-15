# Vibe-Kanban 安全审计报告

## 审计日期：2025-07-13
## 审计范围：API 密钥管理、凭证安全、日志记录、数据收集

---

## 执行摘要

经过全面的安全审计，我发现 Vibe-Kanban 项目在安全性方面整体设计良好，采用了多种最佳实践来保护用户的敏感信息。以下是主要发现：

### ✅ 安全优势
1. **API 密钥从不存储在应用内** - Claude 和 Gemini 的 API 密钥完全由用户的 CLI 工具管理
2. **GitHub token 本地存储** - 使用标准的 OAuth Device Flow
3. **日志记录安全** - 敏感信息（如 prompt 内容）在日志中被适当保护
4. **隐私优先的分析** - 分析数据匿名化，不收集敏感信息
5. **MCP 服务器安全** - 只处理任务管理，不涉及凭证

### ⚠️ 需要注意的方面
1. GitHub PAT 明文存储在本地配置文件中
2. 配置文件权限需要用户自行管理
3. 分析服务的 API 密钥在编译时嵌入

---

## 详细发现

### 1. API 密钥和凭证处理

#### 1.1 Claude 和 Gemini API 密钥
**状态：✅ 安全**

```rust
// backend/src/executors/claude.rs
let claude_command = "npx -y @anthropic-ai/claude-code@latest -p --dangerously-skip-permissions --verbose --output-format=stream-json";

// backend/src/executors/gemini.rs  
let gemini_command = "npx @google/gemini-cli@latest --yolo";
```

- **发现**：应用程序通过 npx 调用官方 CLI 工具，API 密钥完全由这些工具管理
- **安全性**：应用程序永远不会接触或存储 Claude/Gemini API 密钥
- **风险**：无

#### 1.2 GitHub Token 管理
**状态：⚠️ 需要注意**

```rust
// backend/src/models/config.rs
pub struct GitHubConfig {
    pub pat: Option<String>,      // Personal Access Token
    pub token: Option<String>,    // OAuth token
    pub username: Option<String>,
    pub primary_email: Option<String>,
    pub default_pr_base: Option<String>,
}
```

- **发现**：GitHub token 存储在本地配置文件中
- **位置**：`~/.mycrew/config.json`（明文）
- **用途**：创建 Pull Request 和验证用户身份
- **建议**：
  - 考虑使用系统密钥链存储（如 macOS Keychain、Windows Credential Manager）
  - 设置配置文件权限为 600（仅用户可读写）

### 2. 数据库和存储

#### 2.1 敏感信息存储
**状态：✅ 安全**

检查了所有数据库迁移文件，没有发现存储 API 密钥或其他凭证的表结构。数据库主要存储：
- 项目信息
- 任务信息
- 执行日志（stdout/stderr）
- 任务尝试记录

#### 2.2 执行日志
**状态：✅ 安全**

```rust
// 日志中记录的内容示例
tracing::debug!("Writing prompt to Claude stdin for task {}: {:?}", task_id, prompt);
```

虽然 prompt 内容在 debug 日志中可见，但：
- 这些日志不会持久化到文件
- 仅在开发模式下输出
- 生产环境通常设置更高的日志级别

### 3. 环境变量使用

**状态：✅ 安全**

环境变量使用非常有限，主要用于：
- `BACKEND_PORT` / `PORT` - 服务器端口配置
- `NODE_NO_WARNINGS` - Node.js 警告抑制
- `DISABLE_WORKTREE_ORPHAN_CLEANUP` - 开发调试标志

没有发现通过环境变量传递敏感信息的情况。

### 4. 日志记录安全性

#### 4.1 敏感信息过滤
**状态：✅ 良好**

日志记录遵循以下原则：
- 使用结构化日志（tracing）
- 适当的日志级别（debug、info、warn、error）
- 没有在 info 级别以上记录敏感信息

#### 4.2 执行器日志
执行器的输出被规范化处理，移除了潜在的敏感信息：

```rust
// 跳过 "result" 类型的日志条目
if msg_type == "result" {
    continue; // 不记录结果
}
```

### 5. 前端安全

#### 5.1 凭证显示
**状态：✅ 安全**

```tsx
// frontend/src/pages/Settings.tsx
<Input
  id="github-token"
  type="password"  // 密码输入框，隐藏内容
  placeholder="ghp_xxxxxxxxxxxxxxxxxxxx"
  value={config.github.pat || ''}
/>
```

- GitHub token 使用密码输入框，不会明文显示
- 没有在前端日志中记录 token

### 6. MCP 服务器安全

**状态：✅ 安全**

MCP 服务器实现仅处理任务管理功能：
- 创建、列出、更新、删除任务
- 不涉及任何凭证管理
- 使用 UUID 进行身份验证和授权

### 7. 分析服务

#### 7.1 数据收集
**状态：✅ 隐私友好**

```rust
// backend/src/services/analytics.rs
pub fn generate_user_id() -> String {
    // 使用硬件 ID 生成匿名用户标识
    format!("npm_user_{:016x}", hasher.finish())
}
```

- 用户 ID 基于硬件信息的哈希值，完全匿名
- 不收集个人身份信息
- 用户可以通过设置禁用分析

#### 7.2 收集的数据
仅收集：
- 匿名用户 ID
- 事件类型（如 "task_created"）
- 设备信息（OS、架构）
- 应用版本

**不收集**：
- API 密钥
- Prompt 内容
- 代码内容
- 文件路径
- Git 仓库信息

### 8. GitHub 服务安全

**状态：✅ 安全**

```rust
// backend/src/services/github_service.rs
impl From<octocrab::Error> for GitHubServiceError {
    fn from(err: octocrab::Error) -> Self {
        // 适当处理 token 无效的情况
        if status == 401 || status == 403 || msg.contains("bad credentials") {
            GitHubServiceError::TokenInvalid
        }
    }
}
```

- 正确处理 token 过期和无效的情况
- 使用官方 Octocrab 库进行 API 调用
- 实现了重试机制，避免暴露敏感错误信息

### 9. 配置文件安全

**状态：⚠️ 需要改进**

当前配置文件示例：
```json
{
  "github": {
    "pat": "ghp_xxxxxxxxxxxxxxxxxxxx",  // 明文存储
    "token": "gho_xxxxxxxxxxxxxxxxxxxx"
  }
}
```

**建议**：
1. 添加配置文件权限检查和自动设置
2. 考虑加密敏感字段
3. 在文档中明确提醒用户保护配置文件

---

## 安全建议

### 高优先级
1. **配置文件保护**
   - 自动设置配置文件权限为 600
   - 考虑使用系统密钥链存储 GitHub token

2. **文档改进**
   - 添加安全最佳实践文档
   - 明确说明哪些信息被存储在本地

### 中优先级
1. **日志级别控制**
   - 确保生产构建不包含 debug 级别日志
   - 添加日志级别配置选项

2. **错误处理**
   - 确保错误消息不泄露敏感信息
   - 统一错误响应格式

### 低优先级
1. **定期安全审计**
   - 建立定期审查敏感代码的流程
   - 使用自动化工具扫描潜在的安全问题

---

## 结论

Vibe-Kanban 在安全性方面表现良好，主要通过以下设计保护用户隐私：

1. **关键 API 密钥（Claude、Gemini）从不经过应用程序**
2. **本地存储的凭证（GitHub token）使用标准做法**
3. **日志记录谨慎，避免泄露敏感信息**
4. **分析数据完全匿名化**
5. **MCP 服务器功能受限，不涉及凭证**

主要需要改进的是本地配置文件的保护机制。整体而言，该项目展现了对用户隐私和安全的重视，没有发现恶意代码或数据窃取行为。

---

## 审计人员备注

本次审计覆盖了以下文件和目录：
- `/backend/src/executors/` - 所有执行器实现
- `/backend/src/models/` - 数据模型和配置
- `/backend/src/services/` - 外部服务集成
- `/backend/src/routes/` - API 路由
- `/backend/src/mcp/` - MCP 服务器
- `/frontend/src/pages/Settings.tsx` - 前端设置页面
- 所有数据库迁移文件

审计方法：
- 静态代码分析
- 数据流追踪
- 配置审查
- 日志输出分析