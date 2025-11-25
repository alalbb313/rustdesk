# RustDesk Custom Configuration Documentation

## 目标
本文件记录了对 **RustDesk** 源码所做的自定义修改，目的是在以后克隆或拉取仓库后，能够快速恢复这些配置。

---

## 1. 需求概述
| 编号 | 需求描述 |
|------|----------|
| 1 | 将 **默认访问模式** 设置为 `full`（启用 "允许远程配置修改"） |
| 2 | 设置固定默认密码为 **`Jerry@313`** |
| 3 | 在 **网络** 设置页中默认填入以下服务器地址（界面不显示，后端使用）：
|   | - **ID 服务器**: `rustdesk.alalbb.top` |
|   | - **中继服务器**: `rustdesk.alalbb.top` |
|   | - **API 服务器**: `https://rustdesk.alalbb.top:8443` |
|   | - **Key**: `rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=` |
| 4 | 默认开启 **"Allow direct IP access"**（直接 IP 访问） |
| 5 | 在 **显示 → 其他默认选项** 中默认勾选 **"折叠工具栏"** |
| 6 | **隐藏主窗口和连接管理窗口的任务栏图标** |

---

## 2. 代码修改位置与内容
### 2.1 `flutter/lib/common.dart`
#### 2.1.1 默认访问模式（不再依赖 isCustomClient）
```dart
get defaultOptionAccessMode => 'full';
```

#### 2.1.2 默认直接 IP 访问（不再依赖 isCustomClient）
```dart
get defaultOptionDirectServer => 'Y';
```

#### 2.1.3 默认折叠工具栏（不再依赖 isCustomClient）
```dart
get defaultOptionCollapseToolbar => 'Y';
```

#### 2.1.4 网络服务器默认值（ServerConfig）- 界面显示为空
```dart
ServerConfig.fromOptions(Map<String, dynamic> options)
    : idServer = options['custom-rendezvous-server'] ?? '',
      relayServer = options['relay-server'] ?? '',
      apiServer = options['api-server'] ?? '',
      key = options['key'] ?? '';
```
**说明**: 界面上显示为空，实际默认值由 Rust 后端提供。

---
### 2.2 `src/common.rs`
#### 2.2.1 设置默认服务器配置
在 `load_custom_client()` 函数中添加默认服务器配置：
```rust
pub fn load_custom_client() {
    // ... 现有代码 ...
    
    // 在没有 custom.txt 文件时设置默认服务器配置
    set_default_server_config();
}

fn set_default_server_config() {
    let mut settings = config::DEFAULT_SETTINGS.write().unwrap();
    settings.insert("custom-rendezvous-server".to_string(), "rustdesk.alalbb.top".to_string());
    settings.insert("relay-server".to_string(), "rustdesk.alalbb.top".to_string());
    settings.insert("api-server".to_string(), "https://rustdesk.alalbb.top:8443".to_string());
    settings.insert("key".to_string(), "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=".to_string());
}
```

---
### 2.3 `libs/hbb_common/src/config.rs`
#### 2.3.1 默认密码
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

---
### 2.4 `flutter/lib/main.dart`
#### 2.4.1 隐藏任务栏图标
在 `getHiddenTitleBarWindowOptions` 函数中设置 `skipTaskbar: true`：
```dart
WindowOptions getHiddenTitleBarWindowOptions(
    {bool isMainWindow = false,
    Size? size,
    bool center = false,
    bool? alwaysOnTop}) {
  var defaultTitleBarStyle = TitleBarStyle.hidden;
  if (kUseCompatibleUiMode) {
    defaultTitleBarStyle = TitleBarStyle.normal;
  }
  return WindowOptions(
    size: size,
    center: center,
    backgroundColor: (isMacOS && isMainWindow) ? null : Colors.transparent,
    skipTaskbar: true,  // 隐藏任务栏图标
    titleBarStyle: defaultTitleBarStyle,
    alwaysOnTop: alwaysOnTop,
  );
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
2. **运行**：启动 RustDesk
3. **验证网络设置**：打开 **设置 → 网络**，确认服务器字段显示为空（实际使用默认值）
4. **验证显示选项**：**设置 → 显示 → 其他默认选项**，确认 **折叠工具栏** 已被勾选
5. **验证密码**：如果没有手动设置密码，系统应使用 **Jerry@313** 作为默认密码
6. **验证直接 IP 访问**：**远程连接**时，**Allow direct IP access** 应默认打开
7. **验证访问模式**：默认访问模式应为 **full**（允许远程配置修改）
8. **验证任务栏隐藏**：主窗口和连接管理窗口应该不在任务栏中显示图标

---

## 5. 备注
- 本文档已提交到仓库根目录的 `CUSTOM_CONFIG.md`，后续拉取仓库后即可查看。
- 如需在其他分支或新仓库复用，只需复制上述代码块或运行提供的脚本即可。

---

*文档生成于 2025‑11‑25，供后续快速恢复自定义配置使用。*
