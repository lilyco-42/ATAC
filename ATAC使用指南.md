# ATAC 使用指南

ATAC（**A**rguably a **T**erminal **A**PI **C**lient）是一款终端下的 API 客户端工具，类似 Postman、Insomnia 但在终端中运行，无需图形环境。永久免费、无需账户、完全离线。

本分支（`lilyco-42/ATAC`）增加了**简体中文界面支持**。

---

## 快速开始

### 安装

```bash
# 方式一：cargo binstall（推荐，下载预编译包）
cargo binstall --git https://github.com/lilyco-42/ATAC atac

# 方式二：cargo install（从源码编译）
cargo install --git https://github.com/lilyco-42/ATAC atac
```

### 启动

```bash
# 中文界面
atac --lang zh

# 英文界面（默认）
atac

# 查看帮助
atac --help
```

### 语言设置

三种方式设置语言，优先级从高到低：

1. **命令行参数**：`atac --lang zh`
2. **配置文件** `atac.toml`：
   ```toml
   language = "zh"
   ```
3. **环境变量**：
   ```bash
   export ATAC_LANG=zh
   atac
   ```

---

## 界面概览

启动后分为三个区域：

```
┌────── 左侧 ──────┬────── 右侧（请求编辑）──────┬── 右侧（响应）──┐
│  集合列表         │  请求方法 / URL            │  响应结果       │
│  ├ 我的 API       │  参数 / 认证 / 请求头      │  响应体         │
│  │ ├ 获取用户     │  请求体 / 脚本 / 设置      │  Cookie        │
│  │ └ 创建用户     │                            │  控制台        │
│  └ 测试           │                            │                │
├──────── 底部状态栏 ────────────────────────────────┤
│  快捷键提示 / 状态信息                             │
└──────────────────────────────────────────────────┘
```

### 快捷键

| 按键 | 功能 |
|------|------|
| `q` / `Esc` | 退出 / 返回 |
| `Ctrl-h` | 显示帮助 |
| `↑` `↓` | 上下移动 |
| `←` `→` | 左右切换标签页 |
| `Enter` | 选中 / 展开 |
| `c` | 创建集合 / 请求 |
| `d` | 删除 |
| `r` | 重命名 |
| `Ctrl-r` | 发送请求 / 取消 |
| `Tab` | 下一个标签页 |
| `^a` | 切换认证方式 |
| `^b` | 切换请求体类型 |
| `^m` | 切换请求方法 |

完整快捷键列表按 `Ctrl-h` 查看。

---

## 基本操作

### 创建集合

1. 在主界面按 `c`
2. 选择 "集合"
3. 输入集合名称后回车

### 创建请求

1. 选中一个集合按 `Enter` 展开
2. 按 `c`，选择 "请求"
3. 输入请求名称后回车

### 发送请求

1. 选中请求，按 `Enter` 进入请求编辑
2. 输入 URL（如 `https://api.example.com/users`）
3. 按 `Ctrl-r` 发送
4. 右侧显示响应结果

### 编辑请求参数

在请求编辑界面，使用 `←` `→` 在以下标签页间切换：

- **参数** — URL 查询参数
- **认证** — 支持 No auth / Basic / Bearer / Digest / JWT
- **请求头** — 自定义 HTTP 头部
- **请求体** — 支持 No body / Form / File / Text / JSON / XML / HTML / Javascript
- **消息** — WebSocket 消息
- **脚本** — 前置 / 后置 JavaScript 脚本
- **设置** — 代理、超时、重定向、Cookie 等

---

## 认证方式

按 `^a` 切换：

| 方式 | 说明 |
|------|------|
| No auth | 无认证 |
| Basic Auth | 用户名 + 密码 |
| Bearer Token | 直接在请求头加 `Authorization: Bearer <token>` |
| Digest Auth | HTTP Digest 摘要认证 |
| JWT | JSON Web Token，支持 HS256/HS384/HS512/RS256/RS384/RS512/ES256/ES384/ES512 |

---

## 请求体类型

按 `^b` 切换：

| 类型 | 说明 |
|------|------|
| No body | 无请求体（GET/HEAD 等） |
| Form | URL 编码表单 |
| File | 从文件读取 |
| Multipart | 多部分表单 |
| Text | 纯文本 |
| JSON | JSON（带语法高亮） |
| XML | XML |
| HTML | HTML |
| Javascript | JavaScript |

---

## 脚本

ATAC 支持 JavaScript 脚本（基于 Boa JS 引擎）：

- **前置脚本** — 在发送请求前执行，可用于修改请求参数
- **后置脚本** — 在收到响应后执行，可用于处理响应数据

---

## 导出

按 `e` 可将请求导出为其他格式：

- HTTP
- cURL
- PHP Guzzle
- Node.js Axios
- Rust Reqwest

---

## 环境变量

按 `e` 进入环境编辑器，可以创建和管理环境变量，支持多环境切换。

---

## 数据文件

所有数据存储在指定目录（默认 `~/.local/share/atac/` 或 `--directory` 指定的路径）：

- `*.json` / `*.yaml` — 集合和请求
- `.env.*` — 环境变量文件
- `atac.toml` — 配置文件
- `atac.log` — 日志文件

数据文件是纯文本，可直接用 Git 管理版本。

---

## 导入

支持导入其他工具的数据：

- **Postman** v2.1 集合和环境文件
- **OpenAPI** 规范文件
- **cURL** 命令文件

---

## 命令行用法

```bash
# 直接发送请求（无需进入 TUI）
atac GET https://api.example.com/users
atac POST https://api.example.com/users -b '{"name":"test"}'

# 更多 CLI 选项
atac --help
```

---

## 配置文件

`atac.toml` 示例：

```toml
language = "zh"
[global]
proxy = "http://127.0.0.1:7890"
syntax_highlighting = true
save_responses = false
```

---

## 主题

ATAC 支持自定义主题文件，详情见[官方主题文档](https://github.com/Julien-cpsn/ATAC/releases/tag/v0.18.0)。

---

## 常见问题

**Q: 启动后界面乱码或显示异常？**
确保终端支持 UTF-8，且字体包含中文字符。

**Q: 如何切回英文界面？**
不加 `--lang` 参数启动，或将配置文件中 `language` 设为 `"en"`。

**Q: 如何更新到最新版？**
```bash
cargo binstall --git https://github.com/lilyco-42/ATAC atac -y
```

**Q: 原版和汉化版能共存吗？**
可以。原版 `cargo install atac` 安装在 `~/.cargo/bin/atac`，把汉化版装到不同路径或手动替换即可。
