#!/usr/bin/env pwsh
# RustDesk 自定义配置应用脚本
# 使用方法: .\apply_custom_config.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "RustDesk 自定义配置应用脚本" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查是否在正确的目录
if (-not (Test-Path "libs\hbb_common\src\config.rs")) {
    Write-Host "错误: 请在 RustDesk 项目根目录运行此脚本！" -ForegroundColor Red
    exit 1
}

Write-Host "开始应用自定义配置..." -ForegroundColor Green
Write-Host ""

# 备份原始文件
Write-Host "创建备份..." -ForegroundColor Yellow
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$backupDir = "config_backup_$timestamp"
New-Item -ItemType Directory -Path $backupDir -Force | Out-Null

Copy-Item "libs\hbb_common\src\config.rs" "$backupDir\config.rs.bak"
Copy-Item "libs\hbb_common\src\websocket.rs" "$backupDir\websocket.rs.bak"
Copy-Item "src\client.rs" "$backupDir\client.rs.bak"
Copy-Item "flutter\lib\main.dart" "$backupDir\main.dart.bak"

Write-Host "备份已保存到: $backupDir" -ForegroundColor Green
Write-Host ""

# 1. 修改 config.rs - DEFAULT_SETTINGS
Write-Host "1. 配置 DEFAULT_SETTINGS..." -ForegroundColor Yellow
$configPath = "libs\hbb_common\src\config.rs"
$content = Get-Content $configPath -Raw

# 替换 DEFAULT_SETTINGS
$oldPattern = 'pub static ref DEFAULT_SETTINGS: RwLock<HashMap<String, String>> = RwLock::new\(HashMap::from\(\[\s*\]\)\);'
$newSettings = @'
pub static ref DEFAULT_SETTINGS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::from([
        ("access-mode".to_owned(), "full".to_owned()),
        ("direct-server".to_owned(), "Y".to_owned()),
        ("collapse_toolbar".to_owned(), "Y".to_owned()),
        ("enable-udp-punch".to_owned(), "Y".to_owned()),
        ("enable-ipv6-punch".to_owned(), "Y".to_owned()),
        ("allow-insecure-tls-fallback".to_owned(), "Y".to_owned()),
    ]));
'@

if ($content -match $oldPattern) {
    $content = $content -replace $oldPattern, $newSettings
    $content | Set-Content $configPath -NoNewline
    Write-Host "   ✓ DEFAULT_SETTINGS 已更新" -ForegroundColor Green
} else {
    Write-Host "   ⚠ DEFAULT_SETTINGS 可能已经配置或格式不匹配" -ForegroundColor Yellow
}

Write-Host "配置应用完成！" -ForegroundColor Green
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "下一步:" -ForegroundColor Cyan
Write-Host "1. 检查修改的文件" -ForegroundColor White
Write-Host "2. 运行构建命令编译应用" -ForegroundColor White
Write-Host "3. 如需恢复，请使用备份目录: $backupDir" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
