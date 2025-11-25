#!/usr/bin/env bash
# RustDesk 自定义配置应用脚本
# 使用方法: chmod +x apply_custom_config.sh && ./apply_custom_config.sh

set -e

echo "========================================"
echo "RustDesk 自定义配置应用脚本"
echo "========================================"
echo ""

# 检查是否在正确的目录
if [ ! -f "libs/hbb_common/src/config.rs" ]; then
    echo "错误: 请在 RustDesk 项目根目录运行此脚本！"
    exit 1
fi

echo "开始应用自定义配置..."
echo ""

# 检测操作系统以使用正确的 sed 参数
if [[ "$OSTYPE" == "darwin"* ]]; then
    SED_INPLACE=(-i '')
else
    SED_INPLACE=(-i)
fi

# 备份原始文件
echo "创建备份..."
timestamp=$(date +%Y%m%d_%H%M%S)
backupDir="config_backup_$timestamp"
mkdir -p "$backupDir"

cp libs/hbb_common/src/config.rs "$backupDir/config.rs.bak"
cp libs/hbb_common/src/websocket.rs "$backupDir/websocket.rs.bak"
cp src/client.rs "$backupDir/client.rs.bak"
cp flutter/lib/main.dart "$backupDir/main.dart.bak"

echo "备份已保存到: $backupDir"
echo ""

# 1. 修改 websocket.rs
echo "1. 更新 WebSocket 配置..."
sed "${SED_INPLACE[@]}" 's/Config::get_option(OPTION_RELAY_SERVER)/Config::get_relay_server()/g' libs/hbb_common/src/websocket.rs
sed "${SED_INPLACE[@]}" 's/Config::get_option("api-server")/Config::get_api_server()/g' libs/hbb_common/src/websocket.rs
echo "   ✓ WebSocket 配置已更新"

# 2. 修改 main.dart
echo "2. 配置任务栏隐藏..."
sed "${SED_INPLACE[@]}" 's/windowManager\.show(),/windowManager.show().then((_) => windowManager.setSkipTaskbar(true)),/g' flutter/lib/main.dart
echo "   ✓ 任务栏隐藏已配置"

echo ""
echo "配置应用完成！"
echo ""
echo "========================================"
echo "注意事项:"
echo "1. config.rs 中的 DEFAULT_SETTINGS 需要手动修改"
echo "2. client.rs 中的 secure_connection 需要手动修改"
echo "3. 请参考 CUSTOM_CONFIG.md 了解详细修改内容"
echo "4. 如需恢复，请使用备份目录: $backupDir"
echo "========================================"
