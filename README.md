# AgentOS (MYagent)

> **100% Pure Rust | Zero JavaScript | Runtime & UI Decoupling | Multi-DLL Modular Architecture**

本工程采用 **Cargo Workspace 多子工程架构**，彻底将**无头核心运行时（Runtime Daemon / Headless Service）**与**外部原生桌面界面（Pure Rust Desktop UI）**物理隔离解耦。

---

## 📦 官方安装包与便携版直链下载 (Release Downloads)

| 下载包类型 | 文件名 | 特性与适用场景 | 官方直接下载直链 |
| :--- | :--- | :--- | :--- |
| 🚀 **现代 NSIS 安装向导 (首选)** | `AgentOS-Setup-x64.exe` | NSIS Modern UI 2 极速向导，双击即装，原生简体中文不缺字，自动桌面快捷方式与一键秒开 | [👉 **点击下载 NSIS 默认版**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Setup-x64.exe) |
| ⚡ **NSIS 独立专享安装包** | `AgentOS-Setup-NSIS-x64.exe` | NSIS 独立单文件规格包，支持用户级免提权安装与极速静默卸载 | [👉 **点击下载 NSIS 专享包**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Setup-NSIS-x64.exe) |
| 🛠️ **Inno Setup 经典安装包** | `AgentOS-Setup-Inno-x64.exe` | Inno Setup 6 现代化 Fluent 向导，支持多进程互斥锁检测，桌面与开始菜单图标 | [👉 **点击下载 Inno 安装包**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Setup-Inno-x64.exe) |
| 💼 **免安装绿色便携版** | `AgentOS-Studio-Portable-x64.zip` | 无需任何安装过程，解压后双击 `myagent_ui.exe` 直接运行，自带离线脚本 | [👉 **点击下载绿色便携版**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Studio-Portable-x64.zip) |
| 📦 **Windows 企业标准 MSI** | `AgentOS-Setup-x64.msi` | 微软原生 `msiexec.exe` 企业标准安装数据库，支持域控组策略 GPO 静默推送 | [👉 **点击下载 MSI 安装包**](https://github.com/monikalnbo/foragent/releases/download/v0.1.0/AgentOS-Setup-x64.msi) |
| 🌐 **GitHub Releases 资产主页** | `v0.1.0 (Latest)` | 查看历史发布与全部 5 大交付物资产清单 | [🔗 **前往 Releases 主页**](https://github.com/monikalnbo/foragent/releases/tag/v0.1.0) |

> 💡 **提示**：所有 5 种安装/分发方式均 100% 完整打包所有 5 个核心 DLL，路径与快捷方式全兼容。推荐首选 **`AgentOS-Setup-x64.exe` (NSIS)**，点击即可直接下载运行！

---

## 一、 核心子工程全景树（全部为 40~85 行微文件）

```text
MYagent/
├── myagent.toml                     # ★ 全局极简配置文件 (BaseURL, API Key, 模型清单)
├── .github/workflows/build.yml      # GitHub Actions 跨平台自动打包与构建流水线
├── Cargo.toml                       # Workspace 根配置 (8 个独立子工程)
│
├── installer/                       # ★ 【Windows 原生安装体系】
│   ├── nsis/setup.nsi               # NSIS 现代化单文件 EXE 安装器脚本
│   ├── inno/setup.iss               # Inno Setup 脚本
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
