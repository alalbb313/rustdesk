# RustDesk Custom Configuration Documentation

## 目标
本文件记录了对 **RustDesk** 源码所做的自定义修改，目的是在以后克隆或拉取仓库后，能够快速恢复这些配置。

---

## 1. 需求概述
| 编号 | 需求描述 |
|------|----------|
| 1 | 将 **默认访问模式** 设置为 `full`（启用 "允许远程配置修改"） |
| 2 | 在 **网络** 设置页中默认填入以下服务器地址（界面不显示，后端使用）： |
|   | - **ID 服务器**: `rustdesk.alalbb.top` |
|   | - **中继服务器**: `rustdesk.alalbb.top` |
|   | - **API 服务器**: `https://rustdesk.alalbb.top:8443` |
|   | - **Key**: `rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=` |
| 3 | 默认开启 **"Allow direct IP access"**（直接 IP 访问） |
| 4 | 在 **显示 → 其他默认选项** 中默认勾选 **"折叠工具栏"** |
| 5 | **隐藏主窗口和连接管理窗口的任务栏图标** |
| 6 | 在 **常规** 设置中默认启用 **"启用 UDP 打洞"** 和 **"启用 IPv6 P2P 连接"** |
| 7 | 在 **网络** 设置中默认启用 **"允许回退到不安全的 TLS 连接"** |

---

## 2. 代码修改位置与内容

### 2.1 `libs/hbb_common/src/config.rs`

#### 2.1.1 默认设置配置
在 `DEFAULT_SETTINGS` 中添加所有默认配置：

```rust
pub static ref DEFAULT_SETTINGS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::from([
    ("access-mode".to_owned(), "full".to_owned()),
    ("direct-server".to_owned(), "Y".to_owned()),
    ("collapse_toolbar".to_owned(), "Y".to_owned()),
    ("enable-udp-punch".to_owned(), "Y".to_owned()),
    ("enable-ipv6-punch".to_owned(), "Y".to_owned()),
    ("allow-insecure-tls-fallback".to_owned(), "Y".to_owned()),
]));
```

**说明**：
- `access-mode: full` - 设置完全访问权限
- `direct-server: Y` - 允许直接 IP 访问
- `collapse_toolbar: Y` - 默认折叠工具栏
- `enable-udp-punch: Y` - 启用 UDP 打洞
- `enable-ipv6-punch: Y` - 启用 IPv6 P2P 连接
- `allow-insecure-tls-fallback: Y` - 允许回退到不安全的 TLS 连接

#### 2.1.2 服务器配置方法
添加获取默认服务器配置的方法：

```rust
pub fn get_rendezvous_server() -> String {
    let mut rendezvous_server = EXE_RENDEZVOUS_SERVER.read().unwrap().clone();
    if rendezvous_server.is_empty() {
        rendezvous_server = Self::get_option("custom-rendezvous-server");
    }
    if rendezvous_server.is_empty() {
        rendezvous_server = PROD_RENDEZVOUS_SERVER.read().unwrap().clone();
    }
    if rendezvous_server.is_empty() {
        rendezvous_server = "rustdesk.alalbb.top".to_owned();
    }
    // ... 其余代码
}

pub fn get_relay_server() -> String {
    let v = Self::get_option(keys::OPTION_RELAY_SERVER);
    if v.is_empty() {
        "rustdesk.alalbb.top".to_owned()
    } else {
        v
    }
}

pub fn get_api_server() -> String {
    let v = Self::get_option(keys::OPTION_API_SERVER);
    if v.is_empty() {
        "https://rustdesk.alalbb.top:8443".to_owned()
    } else {
        v
    }
}

pub fn get_key() -> String {
    let v = Self::get_option(keys::OPTION_KEY);
    if v.is_empty() {
        "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=".to_owned()
    } else {
        v
    }
}
```

---

### 2.2 `libs/hbb_common/src/websocket.rs`

更新 WebSocket 配置以使用新的获取方法：

```rust
let relay_server = Config::get_relay_server();
// ...
let api_server = Config::get_api_server();
```

---

### 2.3 `src/client.rs`

更新客户端连接以使用默认 key：

```rust
async fn secure_connection(
    peer_id: &str,
    signed_id_pk: Vec<u8>,
    key: &str,
    conn: &mut Stream,
) -> ResultType<Option<Vec<u8>>> {
    let default_key = Config::get_key();
    let key_to_use = if key.is_empty() {
        &default_key
    } else {
        key
    };
    let rs_pk = get_rs_pk(key_to_use);
    // ... 其余代码
}
```

---

### 2.4 `flutter/lib/main.dart`

#### 2.4.1 隐藏任务栏图标

在主窗口显示时添加：
```dart
await windowManager.show();
await windowManager.setSkipTaskbar(true);
```

在连接管理窗口显示时添加：
```dart
windowManager.show().then((_) => windowManager.setSkipTaskbar(true)),
```

---

## 3. 自动化恢复脚本

### 3.1 PowerShell 脚本 (Windows)

将以下脚本保存为 `apply_custom_config.ps1`：

```powershell
#!/usr/bin/env pwsh
# RustDesk 自定义配置应用脚本

Write-Host "开始应用 RustDesk 自定义配置..." -ForegroundColor Green

# 1. 修改 libs/hbb_common/src/config.rs - DEFAULT_SETTINGS
Write-Host "1. 配置默认设置..." -ForegroundColor Yellow
$configPath = "libs\hbb_common\src\config.rs"
$content = Get-Content $configPath -Raw

# 查找并替换 DEFAULT_SETTINGS
$pattern = '(?s)(pub static ref DEFAULT_SETTINGS.*?RwLock::new\(HashMap::from\(\[)(.*?)(\]\)\);)'
$replacement = @'
$1
        ("access-mode".to_owned(), "full".to_owned()),
        ("direct-server".to_owned(), "Y".to_owned()),
        ("collapse_toolbar".to_owned(), "Y".to_owned()),
        ("enable-udp-punch".to_owned(), "Y".to_owned()),
        ("enable-ipv6-punch".to_owned(), "Y".to_owned()),
        ("allow-insecure-tls-fallback".to_owned(), "Y".to_owned()),
    $3
'@
$content = $content -replace $pattern, $replacement
$content | Set-Content $configPath -NoNewline

# 2. 添加服务器配置方法
Write-Host "2. 添加服务器配置方法..." -ForegroundColor Yellow

# 在 get_rendezvous_server 中添加默认值
$content = Get-Content $configPath -Raw
if ($content -notmatch 'rendezvous_server = "rustdesk\.alalbb\.top"') {
    $pattern = '(if rendezvous_server\.is_empty\(\) \{[\s\S]*?rendezvous_server = PROD_RENDEZVOUS_SERVER\.read\(\)\.unwrap\(\)\.clone\(\);[\s\S]*?\})'
    $replacement = @'
$1
        if rendezvous_server.is_empty() {
            rendezvous_server = "rustdesk.alalbb.top".to_owned();
        }
'@
    $content = $content -replace $pattern, $replacement
}

# 添加 get_relay_server, get_api_server, get_key 方法
if ($content -notmatch 'pub fn get_relay_server') {
    $insertPoint = 'pub fn get_bool_option'
    $newMethods = @'

    pub fn get_relay_server() -> String {
        let v = Self::get_option(keys::OPTION_RELAY_SERVER);
        if v.is_empty() {
            "rustdesk.alalbb.top".to_owned()
        } else {
            v
        }
    }

    pub fn get_api_server() -> String {
        let v = Self::get_option(keys::OPTION_API_SERVER);
        if v.is_empty() {
            "https://rustdesk.alalbb.top:8443".to_owned()
        } else {
            v
        }
    }

    pub fn get_key() -> String {
        let v = Self::get_option(keys::OPTION_KEY);
        if v.is_empty() {
            "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=".to_owned()
        } else {
            v
        }
    }

'@
    $content = $content -replace "(\s+$insertPoint)", "$newMethods`$1"
}
$content | Set-Content $configPath -NoNewline

# 3. 修改 libs/hbb_common/src/websocket.rs
Write-Host "3. 更新 WebSocket 配置..." -ForegroundColor Yellow
$websocketPath = "libs\hbb_common\src\websocket.rs"
$content = Get-Content $websocketPath -Raw
$content = $content -replace 'let relay_server = Config::get_option\(OPTION_RELAY_SERVER\);', 'let relay_server = Config::get_relay_server();'
$content = $content -replace 'let api_server = Config::get_option\("api-server"\);', 'let api_server = Config::get_api_server();'
$content | Set-Content $websocketPath -NoNewline

# 4. 修改 src/client.rs
Write-Host "4. 更新客户端连接配置..." -ForegroundColor Yellow
$clientPath = "src\client.rs"
$content = Get-Content $clientPath -Raw
$pattern = '(?s)(async fn secure_connection\([\s\S]*?\) -> ResultType<Option<Vec<u8>>> \{[\s\S]*?)let rs_pk = get_rs_pk\(if key\.is_empty\(\) \{[\s\S]*?config::RS_PUB_KEY[\s\S]*?\} else \{[\s\S]*?key[\s\S]*?\}\);'
$replacement = @'
$1let default_key = Config::get_key();
        let key_to_use = if key.is_empty() {
            &default_key
        } else {
            key
        };
        let rs_pk = get_rs_pk(key_to_use);
'@
$content = $content -replace $pattern, $replacement
$content | Set-Content $clientPath -NoNewline

# 5. 修改 flutter/lib/main.dart
Write-Host "5. 配置任务栏隐藏..." -ForegroundColor Yellow
$mainPath = "flutter\lib\main.dart"
$content = Get-Content $mainPath -Raw

# 主窗口
if ($content -match 'windowManager\.show\(\);' -and $content -notmatch 'windowManager\.setSkipTaskbar\(true\);') {
    $content = $content -replace '(\s+)(windowManager\.show\(\);)', "`$1await windowManager.show();`r`n`$1await windowManager.setSkipTaskbar(true);"
}

# 连接管理窗口
$content = $content -replace 'windowManager\.show\(\),', 'windowManager.show().then((_) => windowManager.setSkipTaskbar(true)),'

$content | Set-Content $mainPath -NoNewline

Write-Host "配置应用完成！" -ForegroundColor Green
Write-Host "请运行构建命令以编译应用。" -ForegroundColor Cyan
```

### 3.2 Bash 脚本 (Linux/macOS)

将以下脚本保存为 `apply_custom_config.sh`：

```bash
#!/usr/bin/env bash
set -e

echo "开始应用 RustDesk 自定义配置..."

# 检测操作系统以使用正确的 sed 参数
if [[ "$OSTYPE" == "darwin"* ]]; then
    SED_INPLACE=(-i '')
else
    SED_INPLACE=(-i)
fi

# 1. 修改 DEFAULT_SETTINGS
echo "1. 配置默认设置..."
# 这里需要手动编辑，因为 sed 处理多行替换比较复杂
# 建议直接修改 libs/hbb_common/src/config.rs

# 2. 修改 websocket.rs
echo "2. 更新 WebSocket 配置..."
sed "${SED_INPLACE[@]}" 's/let relay_server = Config::get_option(OPTION_RELAY_SERVER);/let relay_server = Config::get_relay_server();/g' libs/hbb_common/src/websocket.rs
sed "${SED_INPLACE[@]}" 's/let api_server = Config::get_option("api-server");/let api_server = Config::get_api_server();/g' libs/hbb_common/src/websocket.rs

# 3. 修改 main.dart
echo "3. 配置任务栏隐藏..."
sed "${SED_INPLACE[@]}" 's/windowManager\.show(),/windowManager.show().then((_) => windowManager.setSkipTaskbar(true)),/g' flutter/lib/main.dart

echo "配置应用完成！"
echo "注意：某些复杂的修改需要手动完成，请参考 CUSTOM_CONFIG.md"
```

---

## 4. 验证步骤

1. **编译**：`cargo build --release`（或使用对应平台的构建脚本）
2. **运行**：启动 RustDesk
3. **验证常规设置**：
   - 打开 **设置 → 常规**
   - 确认 **启用 UDP 打洞** 已勾选
   - 确认 **启用 IPv6 P2P 连接** 已勾选
4. **验证安全设置**：
   - 打开 **设置 → 安全**
   - 确认权限设置为 **完全访问**
5. **验证网络设置**：
   - 打开 **设置 → 网络**
   - 确认 **ID/中继服务器** 字段显示为空（实际使用默认值）
   - 确认 **允许直接 IP 访问** 已启用
   - 确认 **允许回退到不安全的 TLS 连接** 已启用
6. **验证显示选项**：
   - 打开 **设置 → 显示 → 其他默认选项**
   - 确认 **折叠工具栏** 已被勾选
7. **验证任务栏隐藏**：
   - 主窗口和连接管理窗口应该不在任务栏中显示图标

---

## 5. 修改文件清单

| 文件路径 | 修改内容 |
|---------|---------|
| `libs/hbb_common/src/config.rs` | 添加 DEFAULT_SETTINGS 和服务器配置方法 |
| `libs/hbb_common/src/websocket.rs` | 使用新的配置获取方法 |
| `src/client.rs` | 更新 secure_connection 使用默认 key |
| `flutter/lib/main.dart` | 添加 skipTaskbar 设置 |

---

## 6. 备注

- 本文档已提交到仓库根目录的 `CUSTOM_CONFIG.md`，后续拉取仓库后即可查看。
- 所有配置都通过 `DEFAULT_SETTINGS` 实现，用户可以在 UI 中覆盖这些默认值。
- 服务器地址在 UI 中不显示，但后端会使用这些默认值。
- 如需在其他分支或新仓库复用，只需复制上述代码块或运行提供的脚本即可。

---

*文档更新于 2025‑11‑25，供后续快速恢复自定义配置使用。*
