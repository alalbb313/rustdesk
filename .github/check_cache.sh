#!/bin/bash

# GitHub Actions 缓存快速检查脚本

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  GitHub Actions 缓存配置检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查 workflow 文件
WORKFLOW_FILE=".github/workflows/flutter-build.yml"

if [ ! -f "$WORKFLOW_FILE" ]; then
    echo "❌ 错误：未找到 $WORKFLOW_FILE"
    exit 1
fi

echo "✓ 找到 workflow 文件"
echo ""

# 统计缓存配置
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  缓存配置统计"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

RUST_CACHE_COUNT=$(grep -c "Swatinem/rust-cache@v2" "$WORKFLOW_FILE")
ACTIONS_CACHE_COUNT=$(grep -c "actions/cache@v3" "$WORKFLOW_FILE")
VCPKG_CACHE_COUNT=$(grep -c "lukka/run-vcpkg@v11" "$WORKFLOW_FILE")

echo "Rust 缓存 (Swatinem/rust-cache@v2): $RUST_CACHE_COUNT 处"
echo "Actions 缓存 (actions/cache@v3): $ACTIONS_CACHE_COUNT 处"
echo "VCPKG 缓存 (lukka/run-vcpkg@v11): $VCPKG_CACHE_COUNT 处"
echo ""

# 检查依赖文件的最近修改时间
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  依赖文件状态"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

check_file_status() {
    local file=$1
    if [ -f "$file" ]; then
        local last_commit=$(git log -1 --format="%h %ar" -- "$file" 2>/dev/null)
        if [ -n "$last_commit" ]; then
            echo "✓ $file"
            echo "  最后修改: $last_commit"
        else
            echo "✓ $file (未提交)"
        fi
    else
        echo "⚠ $file (不存在)"
    fi
}

check_file_status "Cargo.lock"
check_file_status "flutter/pubspec.lock"
check_file_status "vcpkg.json"
check_file_status "flutter/web/js/package.json"
echo ""

# 检查最近的 commits
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  最近 5 次提交"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

git log -5 --oneline --decorate 2>/dev/null || echo "无法获取 git 历史"
echo ""

# 建议
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  诊断建议"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "1. 检查 GitHub Actions 运行历史："
echo "   https://github.com/YOUR_USERNAME/rustdesk/actions"
echo ""

echo "2. 查看缓存存储："
echo "   Settings → Actions → Caches"
echo ""

echo "3. 在 Actions 日志中搜索："
echo "   - 'Cache restored' (缓存已恢复)"
echo "   - 'Cache not found' (缓存未找到)"
echo "   - 'Cache saved' (缓存已保存)"
echo ""

echo "4. 对比构建时间："
echo "   - 首次构建: 30-40 分钟（无缓存）"
echo "   - 后续构建: 10-15 分钟（有缓存）"
echo ""

echo "5. 如果依赖文件最近有修改，缓存会部分失效"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  详细文档"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "查看 .github/CACHE_DIAGNOSTICS.md 获取详细诊断指南"
echo ""
