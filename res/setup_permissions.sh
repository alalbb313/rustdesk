#!/bin/bash

# RustDesk 权限配置助手
# RustDesk Permission Setup Helper

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 应用路径
APP_PATH="/Applications/RustDesk.app"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  RustDesk 权限配置助手"
echo "  RustDesk Permission Setup Helper"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查应用是否已安装
if [ ! -d "$APP_PATH" ]; then
    echo -e "${RED}❌ 错误：未找到 RustDesk.app${NC}"
    echo -e "${RED}❌ Error: RustDesk.app not found${NC}"
    echo ""
    echo -e "${YELLOW}请先将 RustDesk 拖到 Applications 文件夹${NC}"
    echo -e "${YELLOW}Please drag RustDesk to Applications folder first${NC}"
    echo ""
    read -p "按任意键退出 / Press any key to exit..." -n1 -s
    exit 1
fi

echo -e "${GREEN}✓ 找到 RustDesk.app${NC}"
echo -e "${GREEN}✓ Found RustDesk.app${NC}"
echo ""

# 步骤 1: 移除隔离属性
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "步骤 1/4: 移除隔离属性"
echo "Step 1/4: Removing quarantine attribute"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if xattr -cr "$APP_PATH" 2>/dev/null; then
    echo -e "${GREEN}✓ 隔离属性已移除${NC}"
    echo -e "${GREEN}✓ Quarantine attribute removed${NC}"
else
    echo -e "${YELLOW}⚠ 需要管理员权限${NC}"
    echo -e "${YELLOW}⚠ Administrator permission required${NC}"
    echo ""
    echo "请输入管理员密码 / Please enter administrator password:"
    sudo xattr -cr "$APP_PATH"
    echo -e "${GREEN}✓ 隔离属性已移除${NC}"
    echo -e "${GREEN}✓ Quarantine attribute removed${NC}"
fi

echo ""
sleep 1

# 步骤 2: 打开辅助功能设置
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "步骤 2/4: 配置辅助功能权限"
echo "Step 2/4: Configuring Accessibility permission"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${BLUE}即将打开系统设置...${NC}"
echo -e "${BLUE}Opening System Settings...${NC}"
echo ""
echo -e "${YELLOW}请在系统设置中：${NC}"
echo "1. 点击左下角的 🔒 解锁"
echo "2. 找到 RustDesk 并勾选"
echo "3. 如果没有 RustDesk，点击 + 添加"
echo ""
echo -e "${YELLOW}Please in System Settings:${NC}"
echo "1. Click 🔒 to unlock"
echo "2. Find RustDesk and check it"
echo "3. If not found, click + to add"
echo ""

read -p "按回车键打开辅助功能设置 / Press Enter to open Accessibility settings..." -n1 -s
echo ""

# 打开辅助功能设置
open "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"

echo -e "${GREEN}✓ 已打开辅助功能设置${NC}"
echo -e "${GREEN}✓ Accessibility settings opened${NC}"
echo ""
read -p "配置完成后按回车继续 / Press Enter after configuration..." -n1 -s
echo ""
echo ""

# 步骤 3: 打开屏幕录制设置
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "步骤 3/4: 配置屏幕录制权限"
echo "Step 3/4: Configuring Screen Recording permission"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${BLUE}即将打开系统设置...${NC}"
echo -e "${BLUE}Opening System Settings...${NC}"
echo ""
echo -e "${YELLOW}请在系统设置中：${NC}"
echo "1. 点击左下角的 🔒 解锁"
echo "2. 找到 RustDesk 并勾选"
echo ""
echo -e "${YELLOW}Please in System Settings:${NC}"
echo "1. Click 🔒 to unlock"
echo "2. Find RustDesk and check it"
echo ""

read -p "按回车键打开屏幕录制设置 / Press Enter to open Screen Recording settings..." -n1 -s
echo ""

# 打开屏幕录制设置
open "x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture"

echo -e "${GREEN}✓ 已打开屏幕录制设置${NC}"
echo -e "${GREEN}✓ Screen Recording settings opened${NC}"
echo ""
read -p "配置完成后按回车继续 / Press Enter after configuration..." -n1 -s
echo ""
echo ""

# 步骤 4: 打开输入监控设置 (macOS 11.0+)
MACOS_VERSION=$(sw_vers -productVersion | cut -d '.' -f 1)
if [ "$MACOS_VERSION" -ge 11 ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "步骤 4/4: 配置输入监控权限"
    echo "Step 4/4: Configuring Input Monitoring permission"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo -e "${BLUE}即将打开系统设置...${NC}"
    echo -e "${BLUE}Opening System Settings...${NC}"
    echo ""
    echo -e "${YELLOW}请在系统设置中：${NC}"
    echo "1. 点击左下角的 🔒 解锁"
    echo "2. 找到 RustDesk 并勾选"
    echo ""
    echo -e "${YELLOW}Please in System Settings:${NC}"
    echo "1. Click 🔒 to unlock"
    echo "2. Find RustDesk and check it"
    echo ""
    
    read -p "按回车键打开输入监控设置 / Press Enter to open Input Monitoring settings..." -n1 -s
    echo ""
    
    # 打开输入监控设置
    open "x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent"
    
    echo -e "${GREEN}✓ 已打开输入监控设置${NC}"
    echo -e "${GREEN}✓ Input Monitoring settings opened${NC}"
    echo ""
    read -p "配置完成后按回车继续 / Press Enter after configuration..." -n1 -s
    echo ""
    echo ""
else
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "步骤 4/4: 跳过（macOS 版本 < 11.0）"
    echo "Step 4/4: Skipped (macOS version < 11.0)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
fi

# 完成
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ 配置完成！${NC}"
echo -e "${GREEN}✓ Configuration Complete!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${BLUE}现在可以启动 RustDesk 了${NC}"
echo -e "${BLUE}You can now launch RustDesk${NC}"
echo ""
echo -e "${YELLOW}提示：首次运行时，RustDesk 可能还会请求权限${NC}"
echo -e "${YELLOW}Note: RustDesk may still request permissions on first launch${NC}"
echo ""

read -p "是否现在启动 RustDesk? (y/n) / Launch RustDesk now? (y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo -e "${GREEN}正在启动 RustDesk...${NC}"
    echo -e "${GREEN}Launching RustDesk...${NC}"
    open "$APP_PATH"
    sleep 2
fi

echo ""
echo -e "${GREEN}感谢使用 RustDesk！${NC}"
echo -e "${GREEN}Thank you for using RustDesk!${NC}"
echo ""
