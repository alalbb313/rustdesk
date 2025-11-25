# PowerShell script to apply custom RustDesk configuration (Windows)
# Save as apply_custom_config.ps1 and run from repository root.

# 1. Dart – default options
(Get-Content flutter\lib\common.dart) -replace "get defaultOptionAccessMode => .+;", "get defaultOptionAccessMode => isCustomClient ? 'full' : '';" | Set-Content flutter\lib\common.dart

(Get-Content flutter\lib\common.dart) -replace "get defaultOptionDirectServer => .+;", "get defaultOptionDirectServer => isCustomClient ? 'Y' : '';" | Set-Content flutter\lib\common.dart

(Get-Content flutter\lib\common.dart) -replace "get defaultOptionCollapseToolbar => .+;", "get defaultOptionCollapseToolbar => isCustomClient ? 'Y' : '';" | Set-Content flutter\lib\common.dart

# 2. ServerConfig default values (replace the four lines after the constructor)
$common = Get-Content flutter\lib\common.dart -Raw
$common = $common -replace "idServer = options\['custom-rendezvous-server'\] \?\? ''", "idServer = options['custom-rendezvous-server'] ?? \"rustdesk.alalbb.top\""
$common = $common -replace "relayServer = options\['relay-server'\] \?\? ''", "relayServer = options['relay-server'] ?? \"rustdesk.alalbb.top\""
$common = $common -replace "apiServer = options\['api-server'\] \?\? ''", "apiServer = options['api-server'] ?? \"https://rustdesk.alalbb.top:8443\""
$common = $common -replace "key = options\['key'\] \?\? ''", "key = options['key'] ?? \"rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=\""
$common | Set-Content flutter\lib\common.dart

# 3. Rust – default permanent password
(Get-Content libs\hbb_common\src\config.rs) -replace "password = .+;", "password = \"Jerry@313\".to_string();" | Set-Content libs\hbb_common\src\config.rs

Write-Host "Custom configuration applied successfully."
