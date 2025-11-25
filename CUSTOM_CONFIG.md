# RustDesk Custom Configuration Documentation

## 目标
本文件记录了对 **RustDesk** 源码所做的自定义修改，目的是在以后克隆或拉取仓库后，能够快速恢复这些配置。

---

## 1. 需求概述
| 编号 | 需求描述 |
|------|----------|
| 1 | 将 **默认访问模式** 设置为 `full`（启用 "允许远程配置修改"） |
| 2 | 设置固定默认密码为 **`Jerry@313`** |
| 3 | 在 **网络** 设置页中默认填入以下服务器地址：
|   | - **ID 服务器**: `rustdesk.alalbb.top` |
|   | - **中继服务器**: `rustdesk.alalbb.top` |
|   | - **API 服务器**: `https://rustdesk.alalbb.top:8443` |
|   | - **Key**: `rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=` |
| 4 | 默认开启 **"Allow direct IP access"**（直接 IP 访问） |
| 5 | 在 **显示 → 其他默认选项** 中默认勾选 **"折叠工具栏"** |

---

## 2. 代码修改位置与内容
### 2.1 `flutter/lib/common.dart`
#### 2.1.1 默认访问模式
```dart
get defaultOptionAccessMode => isCustomClient ? 'full' : '';
```
#### 2.1.2 默认直接 IP 访问
```dart
get defaultOptionDirectServer => isCustomClient ? 'Y' : '';
```
#### 2.1.3 默认折叠工具栏
```dart
get defaultOptionCollapseToolbar => isCustomClient ? 'Y' : '';
```
#### 2.1.4 网络服务器默认值（ServerConfig）
```dart
ServerConfig.fromOptions(Map<String, dynamic> options)
    : idServer = options['custom-rendezvous-server'] ?? "rustdesk.alalbb.top",
      relayServer = options['relay-server'] ?? "rustdesk.alalbb.top",
      apiServer = options['api-server'] ?? "https://rustdesk.alalbb.top:8443",
      key = options['key'] ?? "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=";
```
---
### 2.2 `libs/hbb_common/src/config.rs`
#### 2.2.1 默认密码
```rust
pub fn get_permanent_password() -> String {
    let mut password = CONFIG.read().unwrap().password.clone();
    if password.is_empty() {
        if let Some(v) = HARD_SETTINGS.read().unwrap().get("password") {
            password = v.to_owned();
        } else {
            // Default password
            password = "Jerry@313".to_string();
        }
    }
    password
}
```
#### 2.2.2 服务器默认值（可选）
如果需要在 Rust 端也提供默认服务器，可在 `config.rs` 中添加类似函数（已在本项目中实现）：
```rust
pub fn get_rendezvous_server() -> String {
    let mut server = Self::get_option("custom-rendezvous-server");
    if server.is_empty() {
        server = "rustdesk.alalbb.top".to_string();
    }
    server
}

pub fn get_relay_server() -> String {
    let mut server = Self::get_option("relay-server");
    if server.is_empty() {
        server = "rustdesk.alalbb.top".to_string();
    }
    server
}

pub fn get_api_server() -> String {
    let mut server = Self::get_option("api-server");
    if server.is_empty() {
        server = "https://rustdesk.alalbb.top:8443".to_string();
    }
    server
}

pub fn get_key() -> String {
    let mut key = Self::get_option("key");
    if key.is_empty() {
        key = "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=".to_string();
    }
    key
}
```
---

## 3. 自动化恢复脚本（可选）
将以下脚本保存为 `apply_custom_config.sh`（Linux/macOS）或 `apply_custom_config.ps1`（Windows），在克隆仓库后运行即可自动完成所有修改。
### Bash (Linux/macOS)
```bash
#!/usr/bin/env bash
set -e

# 1. Dart 修改
sed -i '' "s/get defaultOptionAccessMode => .*/get defaultOptionAccessMode => isCustomClient ? 'full' : '';/" flutter/lib/common.dart
sed -i '' "s/get defaultOptionDirectServer => .*/get defaultOptionDirectServer => isCustomClient ? 'Y' : '';/" flutter/lib/common.dart
sed -i '' "s/get defaultOptionCollapseToolbar => .*/get defaultOptionCollapseToolbar => isCustomClient ? 'Y' : '';/" flutter/lib/common.dart

# 2. ServerConfig 默认值
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"rustdesk.alalbb.top\"/" flutter/lib/common.dart
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"rustdesk.alalbb.top\"/" flutter/lib/common.dart
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"https://rustdesk.alalbb.top:8443\"/" flutter/lib/common.dart
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=\"/" flutter/lib/common.dart

# 3. Rust 默认密码
sed -i '' "s/password = .*/password = \"Jerry@313\".to_string();/" libs/hbb_common/src/config.rs

echo "Custom configuration applied."
```
### PowerShell (Windows)
```powershell
# Dart modifications
(Get-Content flutter\lib\common.dart) -replace "get defaultOptionAccessMode => .+;", "get defaultOptionAccessMode => isCustomClient ? 'full' : '';" |
    Set-Content flutter\lib\common.dart

(Get-Content flutter\lib\common.dart) -replace "get defaultOptionDirectServer => .+;", "get defaultOptionDirectServer => isCustomClient ? 'Y' : '';" |
    Set-Content flutter\lib\common.dart

(Get-Content flutter\lib\common.dart) -replace "get defaultOptionCollapseToolbar => .+;", "get defaultOptionCollapseToolbar => isCustomClient ? 'Y' : '';" |
    Set-Content flutter\lib\common.dart

# ServerConfig defaults (replace the four lines after the constructor)
$common = Get-Content flutter\lib\common.dart -Raw
$common = $common -replace "idServer = options\['custom-rendezvous-server'\] \?\? ''", "idServer = options['custom-rendezvous-server'] ?? \"rustdesk.alalbb.top\""
$common = $common -replace "relayServer = options\['relay-server'\] \?\? ''", "relayServer = options['relay-server'] ?? \"rustdesk.alalbb.top\""
$common = $common -replace "apiServer = options\['api-server'\] \?\? ''", "apiServer = options['api-server'] ?? \"https://rustdesk.alalbb.top:8443\""
$common = $common -replace "key = options\['key'\] \?\? ''", "key = options['key'] ?? \"rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=\""
$common | Set-Content flutter\lib\common.dart

# Rust default password
(Get-Content libs\hbb_common\src\config.rs) -replace "password = .+;", "password = \"Jerry@313\".to_string();" |
    Set-Content libs\hbb_common\src\config.rs

Write-Host "Custom configuration applied."
```
---

## 4. 验证步骤
1. **编译**：`cargo build --release`（或使用对应平台的构建脚本）
2. **运行**：启动 RustDesk，打开 **设置 → 网络**，确认服务器字段已填入上述默认值。
3. **设置 → 显示 → 其他默认选项**，确认 **折叠工具栏** 已被勾选。
4. **登录**时，如果没有手动设置密码，系统应使用 **Jerry@313** 作为默认密码。
5. **远程连接**时，**Allow direct IP access** 应默认打开。

---

## 5. 备注
- 本文档已提交到仓库根目录的 `CUSTOM_CONFIG.md`，后续拉取仓库后即可查看。
- 如需在其他分支或新仓库复用，只需复制上述代码块或运行提供的脚本即可。

---

*文档生成于 2025‑11‑25，供后续快速恢复自定义配置使用。*
