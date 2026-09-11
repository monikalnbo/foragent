# AgentOS (MYagent)

> **100% Pure Rust | Zero JavaScript | Runtime & UI Decoupling | Multi-DLL Modular Architecture**

本工程采用 **Cargo Workspace 多子工程架构**，彻底将**无头核心运行时（Runtime Daemon / Headless Service）**与**外部原生桌面界面（Pure Rust Desktop UI）**物理隔离解耦。

---

## 一、 核心子工程全景树（全部为 40~75 行小文件）

```text
MYagent/
├── myagent.toml                     # ★ 全局极简配置文件 (BaseURL, API Key, 模型清单)
├── .github/workflows/build.yml      # GitHub Actions 跨平台全自动编译打包流水线
├── Cargo.toml                       # Workspace 根配置 (8 个独立子工程)
│
├── runtime/                         # ★ 【独立运行时服务】(myagent_runtime.exe / myagent_runtime.dll)
│   ├── Cargo.toml
│   └── src/
│       ├── protocol.rs              # [RuntimeCommand 与 RuntimeEvent 异步解耦通信协议]
│       ├── service.rs               # [后台无头 Agent 核心事件循环]
│       └── main.rs                  # [无头守护进程启动入口]
│
├── engine/                          # ★ 【内部核心引擎体系】(编译为独立 DLL)
│   ├── types/                       # [myagent_types] 核心实体与统一错误 (rlib)
│   ├── protocol/                    # [myagent_protocol.dll] R1 思考流 / ToolCall / 级联降级
│   ├── capability/                  # [myagent_capability.dll]
│   │   ├── src/response_cache.rs    # ★ [BLAKE3 高效推理响应缓存]
│   │   ├── src/memory.rs            # ★ [跨会话长期持久记忆 PersistentMemory]
│   │   ├── src/fs_patch.rs          # [CRLF 模糊 Patch]
│   │   └── src/task_mgr.rs          # [任务清单状态机]
│   ├── self_healing/                # [myagent_healing.dll] (Tree-sitter AST 自检 + 语义看门狗)
│   ├── sandbox/                     # [myagent_sandbox.dll] (影子工作区与进程防爆杀)
│   └── core/                        # [myagent_core.dll]
│       ├── src/config.rs            # ★ [myagent.toml 简易配置加载器]
│       └── src/ffi.rs               # [C-ABI 操纵句柄导出]
│
└── ui/                              # ★ 【外部独立纯界面客户端】(100% 纯 Rust 原生 GUI)
    └── desktop/                     # [myagent_ui.exe] (只负责渲染，通过 Channel 与 Runtime 通信)
```

---

## 二、 极简配置说明 (`myagent.toml`)

在根目录下可一目了然配置 BaseURL、API Key 和模型列表，支持自动读取系统环境变量：

```toml
default_model = "deepseek-reasoner"
timeout_seconds = 60

# 1. 云端最强推理提供商 (DeepSeek)
[[providers]]
name = "deepseek"
base_url = "https://api.deepseek.com/v1"
api_key = "ENV:DEEPSEEK_API_KEY"  # 自动读取环境变量 DEEPSEEK_API_KEY
models = ["deepseek-reasoner", "deepseek-chat"]

# 2. 本地离线兜底提供商 (Ollama)
[[providers]]
name = "ollama"
base_url = "http://localhost:11434/v1"
api_key = "ollama"
models = ["qwen2.5-coder:7b", "deepseek-r1:14b"]
```

---

## 三、 高效缓存与持久记忆机制

1. **BLAKE3 推理与响应缓存 (`ResponseCache`)**：
   * 对 `(model + prompt)` 生成唯一 BLAKE3 哈希指纹；
   * 相同指令或探索任务命中缓存，**微秒级直出，0 Token 消耗**，大幅节约成本与等待时间。
2. **跨会话长期持久记忆 (`PersistentMemory`)**：
   * 自动在本地存储长期用户偏好、项目架构规约与历史采纳的修复方案；
   * 开新会话时自动提取相关记忆注入 Prompt，**越用越懂你的工程习惯**。

---

## 四、 Runtime 与 UI 彻底分离的极致体验

* **Runtime 纯无头（Headless）**：不仅可以作为后台守护进程独立跑在服务器或终端中，也可以被其它 CLI / IDE 插件调用；
* **UI 纯展示（Pure Presentation）**：UI 只发送指令（`RuntimeCommand`）并订阅事件（`RuntimeEvent`），UI 关掉甚至崩溃，后台任务依然在沙箱中安全运行！
