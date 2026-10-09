# aichat 话题版 · 用法说明

基于上游 sigoden/aichat v0.30.0 的本地改造：**把 session 改成"话题"**——聊天自带上下文，
`/new` 一键换话题，全程零落盘，命令全面改 `/` 前缀短词风格。

## 构建

```bash
cd ~/work/aichat
cargo build --release
# 产物：target/release/aichat
```

## 核心变化

```
启动 aichat
    │
    ▼
自动挂一个内存话题（唯一，不用创建，聊天即有上下文）
    │
    ├── /new ──► 旧话题整个丢弃，换全新空话题（无确认、不写盘）
    │
    └── 退出 ──► 内存回收，什么都不留
```

- **磁盘上永远不产生 session 文件**（无自动保存、无自动命名、无 `Save session?` 弹框）
- CLI 的 `-s <名字>` 持久 session 与 agent 内部机制保持原样（agent 模式依赖它）
- 旧 `sessions/` 目录里的历史文件不受影响，想清理手动删

## 命令表（`/` 和 `.` 前缀都认）

| 命令 | 作用 |
|---|---|
| `/new` | **开新话题**：清空上下文，瞬间重开 |
| `/model <name>` | 临时换模型 |
| `/role <name>` | 切角色；**话题聊到一半切换时自动开新话题挂新角色** |
| `/role <name> <text>` | 一次性用角色发消息，不影响当前话题 |
| `/prompt <text>` | 临时人设（话题聊到一半时同样自动开新话题） |
| `/file <path\|url\|\`cmd\`> [-- 问题]` | 文件/网页/命令输出贴进对话 |
| `/info` `/info role` `/info rag` `/info agent` | 状态信息 |
| `/copy` `/continue` `/regenerate` | 复制上条回复 / 续写 / 重新生成 |
| `/compress` | 压缩当前话题上下文（话题聊太长时省 token） |
| `/set <key> <value>` | 临时改运行时配置 |
| `/edit config` `/edit role` `/edit rag-docs` `/edit agent-config` | 编辑器打开对应文件 |
| `/save role <name>` `/delete <role\|rag\|macro\|agent-data>` | 角色存盘 / 删除 |
| `/rag` `/rebuild rag` `/sources rag` `/exit rag` | RAG 文档问答 |
| `/agent` `/starter` `/exit agent` | agent 模式 |
| `/macro` | 提示词模板 |
| `/help` `/exit` | 帮助 / 退出 |

**已移除**（话题持久化全家桶）：`.session` `.save session` `.edit session`
`.info session` `.delete session` `.empty session` `.exit session`
（旧习惯 `.session xxx` 会提示"named sessions removed"并开新话题）

## 边界规则

- `/` 开头但不是已知命令 → **整行当普通提问发出**，如 `/etc/hosts 是什么` 直接问模型
- 话题里切换 `/role` / `/prompt` 时自动开新话题（旧话题丢弃），并打印 `(new topic, role: xxx)` 提示
- 提示符不再出现 `temp)`；自定义 `left_prompt` 无需改动

## 改动文件

- `src/repl/mod.rs` —— 命令表、`/`+`.` 双前缀解析、`/new` 分发、未知命令降级为聊天
- `src/config/mod.rs` —— `new_topic()` / `init_default_topic()`（启动挂内存话题）/ 话题内切角色 / prompt 变量隐藏 temp
- `src/repl/completer.rs` `highlighter.rs` —— 补全高亮支持 `/` 前缀
- `src/main.rs` —— REPL 启动钩子
- 死代码清理：`session_info` `save_session` `edit_session` `exit_agent_session` `Session::render` `render_input` 等

验证：`cargo build --release` ✔　`cargo test`（23 通过）✔　tmux 实测 REPL 交互 ✔
（含：话题聊天带上下文、`/new` 静默重开、未知 `/` 命令当提问、退出后 sessions 目录 0 文件）
