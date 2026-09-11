# AgentOS (MYagent)

> **100% Pure Rust | Zero JavaScript | Runtime & UI Decoupling | Multi-DLL Modular Architecture**

本工程采用 **Cargo Workspace 多子工程架构**，彻底将**无头核心运行时（Runtime Daemon / Headless Service）**与**外部原生桌面界面（Pure Rust Desktop UI）**物理隔离解耦。

---

## 📦 官方安装包与便携版直链下载 (Release Downloads)

| 下载包类型 | 文件名 | 特性与适用场景 | 官方直接下载直链 |
| :--- | :--- | :--- | :--- |
| 🚀 **现代 EXE 安装向导 (推荐)** | `AgentOS-Setup-x64.exe` | 现代 Fluent 向导 (与 VS Code 同规格)，双击即装，原生中文界面，不缺字 | [👉 **点击直接下载 EXE 安装器**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Setup-x64.exe) |
| 💼 **免安装绿色便携版** | `AgentOS-Studio-Portable-x64.zip` | 无需任何安装过程，解压后双击 `myagent_ui.exe` 直接运行 | [👉 **点击下载免安装便携版**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Studio-Portable-x64.zip) |
| 📦 **Windows 标准 MSI 安装包** | `AgentOS-Setup-x64.msi` | Windows 原生 `msiexec.exe` 企业标准安装数据库 | [👉 **点击下载 MSI 安装包**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Setup-x64.msi) |
| 🌐 **GitHub Releases 资源主页** | `v0.1.0 (Latest)` | 查看历史发布与全部交付物资产清单 | [🔗 **前往 Releases 主页**](https://github.com/monikalnbo/foragent/releases/tag/v0.1.0) |

> 💡 **提示**：推荐首选 **`AgentOS-Setup-x64.exe`**，点击即可直接下载运行；如不想安装，亦可直接下载 **`AgentOS-Studio-Portable-x64.zip`** 解压秒开！

---

## 一、 核心子工程全景树（全部为 40~85 行微文件）

```text
MYagent/
├── myagent.toml                     # ★ 全局极简配置文件 (BaseURL, API Key, 模型清单)
├── .github/workflows/build.yml      # GitHub Actions 跨平台自动打包与 MSI 构建流水线
├── Cargo.toml                       # Workspace 根配置 (8 个独立子工程)
│
├── installer/                       # ★ 【Windows 原生安装体系】
│   ├── wix/main.wxs                 # Windows Installer (MSI) WiX 源码定义
│   └── windows/
│       ├── install.cmd              # Windows 自带双击一键安装脚本
│       ├── install.ps1              # 桌面/开始菜单/PATH/已安装应用注册脚本
│       └── uninstall.ps1            # Windows 原生无残留卸载脚本
│
├── runtime/                         # ★ 【独立运行时服务】(myagent_runtime.exe / myagent_runtime.dll)
│   ├── src/protocol.rs              # [RuntimeCommand 与 RuntimeEvent 异步解耦通信协议]
│   ├── src/react_loop.rs            # [15 轮自主 ReAct 闭环循环执行器]
│   └── src/service.rs               # [后台无头 Agent 核心事件调度器]
│
├── engine/                          # ★ 【内部核心引擎体系】(编译为独立 DLL)
│   ├── types/                       # [myagent_types] 核心实体与统一错误 (rlib)
│   ├── protocol/                    # [myagent_protocol.dll] R1 思考流 / XML Action / Native Tools
│   ├── capability/                  # [myagent_capability.dll]
│   │   ├── src/response_cache.rs    # ★ [BLAKE3 高效推理响应缓存]
│   │   ├── src/memory.rs            # ★ [跨会话长期持久记忆 PersistentMemory]
│   │   ├── src/fs_patch.rs          # [CRLF 模糊 Patch 与安全新建]
│   │   └── src/task_mgr.rs          # [任务清单状态机]
│   ├── self_healing/                # [myagent_healing.dll] (Tree-sitter AST 自检 + 语义防偷删)
│   ├── sandbox/                     # [myagent_sandbox.dll] (影子工作区与命令超时防爆杀)
│   └── core/                        # [myagent_core.dll]
│       ├── src/prompt.rs            # [SystemPrompt 动态注入]
│       ├── src/tool_dispatcher.rs   # [7 大工程工具异步分发器]
│       └── src/config.rs            # ★ [myagent.toml 简易配置加载器]
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

## 三、 Windows 原生安装方式 (Windows 自带 Install)

本工程支持 **Windows 官方原生安装**，完全无需安装任何第三方环境：

1. **Windows Installer 官方安装包 (`.msi`)**：
   - GitHub Actions 自动生成 `AgentOS-Setup-x64.msi`；
   - 双击直接调用 Windows 系统自带的 **Windows Installer (`msiexec.exe`)** 安装向导；
   - 自动写入 Windows“设置/控制面板 -> 已安装的应用”并注册系统级卸载。

2. **Windows 自带一键脚本安装 (`install.cmd` / `install.ps1`)**：
   - 解压包内自带 `install.cmd`，直接双击运行；
   - 调用 Windows 自带 PowerShell 与 WScript COM 组件部署至 `%LOCALAPPDATA%\Programs\AgentOS`；
   - 自动生成桌面快捷方式、开始菜单目录、用户 PATH 变量，并注册一键卸载支持。
