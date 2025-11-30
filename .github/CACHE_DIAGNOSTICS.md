# GitHub Actions 缓存诊断指南

## 缓存配置检查清单

### ✅ 已配置的缓存

根据当前配置，以下缓存已启用：

#### Windows Flutter 构建
- ✅ Rust 缓存 (`Swatinem/rust-cache@v2`)
- ✅ VCPKG 缓存 (`lukka/run-vcpkg@v11`)
- ✅ Flutter 自定义引擎缓存 (`actions/cache@v3`)
- ✅ Flutter 依赖缓存 (`actions/cache@v3`)

#### macOS 构建
- ✅ Rust 缓存 (`Swatinem/rust-cache@v2`)
- ✅ VCPKG 缓存 (`lukka/run-vcpkg@v11`)
- ✅ Flutter 依赖缓存 (`actions/cache@v3`)

#### iOS 构建
- ✅ Rust 缓存 (`Swatinem/rust-cache@v2`)
- ✅ VCPKG 缓存 (`lukka/run-vcpkg@v11`)
- ✅ Flutter 依赖缓存 (`actions/cache@v3`)
- ✅ CocoaPods 缓存 (`actions/cache@v3`)

#### Android 构建
- ✅ Rust 缓存 (`Swatinem/rust-cache@v2`)
- ✅ VCPKG 缓存 (`lukka/run-vcpkg@v11`)
- ✅ Gradle 缓存 (`actions/cache@v3`)
- ✅ Flutter 依赖缓存 (`actions/cache@v3`)

#### Linux 构建
- ✅ Rust 缓存 (`Swatinem/rust-cache@v2`)
- ✅ VCPKG 缓存 (`lukka/run-vcpkg@v11`)

#### Linux Sciter 构建
- ✅ Rust 缓存 (`Swatinem/rust-cache@v2`)
- ✅ VCPKG 缓存 (`lukka/run-vcpkg@v11`)

#### Web 构建
- ✅ npm 缓存 (`actions/cache@v3`)

---

## 为什么缓存可能不生效？

### 1. 首次构建（最常见原因）⭐
**现象**：第一次运行时间和之前一样长
**原因**：缓存需要先建立，第一次运行时没有缓存可用
**解决**：等待第二次构建，应该会明显加速

### 2. 缓存键变化
**现象**：每次构建都重新下载依赖
**原因**：缓存键（key）发生变化，导致缓存失效
**常见原因**：
- `Cargo.lock` 文件变化
- `pubspec.lock` 文件变化
- `package-lock.json` 或 `yarn.lock` 变化
- `vcpkg.json` 变化

### 3. 缓存大小限制
**现象**：旧缓存被删除
**原因**：GitHub Actions 对每个仓库的缓存总大小有 10GB 限制
**解决**：
- 旧缓存会自动被删除
- 最常用的缓存会被保留
- 这是正常行为

### 4. 分支隔离
**现象**：切换分支后缓存失效
**原因**：不同分支的缓存是隔离的
**解决**：
- 主分支（master/main）的缓存可以被其他分支使用
- 但其他分支的缓存不能互相共享

### 5. Runner 类型变化
**现象**：缓存无法恢复
**原因**：不同的 runner OS 版本（如 ubuntu-20.04 vs ubuntu-22.04）缓存不兼容
**解决**：确保 `runs-on` 配置一致

---

## 如何验证缓存是否工作？

### 方法 1：查看 GitHub Actions 日志

在 Actions 运行日志中搜索以下关键词：

#### Rust 缓存
```
Post Rust Cache
```
如果看到 `Cache saved successfully`，说明缓存已保存。

下次运行时搜索：
```
Rust Cache
```
如果看到 `Cache restored successfully`，说明缓存已恢复。

#### Flutter/Gradle/npm 缓存
搜索：
```
Cache restored from key:
```
或
```
Cache not found for input keys:
```

### 方法 2：对比构建时间

#### 首次构建（无缓存）
- **Windows**: 30-40 分钟
- **macOS**: 25-35 分钟
- **Android**: 20-30 分钟
- **Linux**: 25-35 分钟

#### 后续构建（有缓存）
- **Windows**: 10-15 分钟 ⚡
- **macOS**: 8-12 分钟 ⚡
- **Android**: 8-12 分钟 ⚡
- **Linux**: 10-15 分钟 ⚡

**注意**：如果代码变化较大，即使有缓存也需要重新编译变更的部分。

### 方法 3：检查缓存使用情况

1. 进入 GitHub 仓库
2. **Settings** → **Actions** → **Caches**
3. 查看已保存的缓存列表和大小

---

## 常见问题排查

### Q1: 为什么第二次构建还是很慢？

**可能原因**：
1. **依赖文件变化**：检查 `Cargo.lock`, `pubspec.lock` 等是否变化
2. **代码大量变更**：即使有缓存，大量代码变更仍需重新编译
3. **VCPKG 缓存失效**：VCPKG 缓存较大，可能被清理

**检查方法**：
```bash
# 查看最近的 commit 是否修改了依赖文件
git diff HEAD~1 Cargo.lock
git diff HEAD~1 flutter/pubspec.lock
```

### Q2: 如何强制重建缓存？

**方法 1**：修改缓存键
在 workflow 文件中修改缓存的 `key`，例如添加版本号：
```yaml
key: v2-${{ runner.os }}-rust-${{ hashFiles('**/Cargo.lock') }}
```

**方法 2**：清除所有缓存
1. GitHub 仓库 → **Settings** → **Actions** → **Caches**
2. 删除所有缓存
3. 重新运行 workflow

### Q3: VCPKG 缓存为什么经常失效？

**原因**：
- VCPKG 缓存非常大（5-8 GB）
- 容易超过 GitHub 的 10GB 限制
- 会被自动清理

**优化建议**：
- 已经使用了 `lukka/run-vcpkg@v11` 的二进制缓存
- 这是最优方案，无需额外优化

---

## 进一步优化建议

### 1. 使用 Matrix 策略并行构建
当前已使用，无需修改。

### 2. 减少不必要的构建
- Tag 构建：只构建核心平台（已配置 ✅）
- Release 构建：构建所有平台（已配置 ✅）

### 3. 考虑自托管 Runner
如果构建频繁，可以考虑使用自托管 Runner：
- ✅ 缓存持久化，不受 10GB 限制
- ✅ 可以使用更强大的硬件
- ❌ 需要自己维护服务器
- ❌ 有安全风险（需要隔离环境）

---

## 诊断步骤

### 步骤 1：检查是否是首次构建
```bash
# 查看 GitHub Actions 历史
# 如果这是第一次运行，缓存还未建立
```

### 步骤 2：查看缓存日志
1. 打开最近的 Actions 运行
2. 展开任意构建任务
3. 搜索 "cache" 或 "Cache"
4. 查看是否有 "Cache restored" 或 "Cache not found"

### 步骤 3：对比两次构建时间
1. 记录第一次构建的总时间
2. 不修改代码，重新触发构建
3. 对比第二次构建时间
4. 应该有 50-70% 的时间节省

### 步骤 4：检查缓存存储
1. GitHub 仓库 → Settings → Actions → Caches
2. 查看是否有缓存条目
3. 查看缓存大小和创建时间

---

## 预期效果总结

| 平台 | 首次构建 | 缓存构建 | 节省时间 |
|------|---------|---------|---------|
| Windows | 35 分钟 | 12 分钟 | ~65% |
| macOS | 30 分钟 | 10 分钟 | ~67% |
| Android | 25 分钟 | 10 分钟 | ~60% |
| Linux | 30 分钟 | 12 分钟 | ~60% |
| iOS | 28 分钟 | 10 分钟 | ~64% |

**重要提示**：
- 首次构建时间不会减少
- 第二次及以后的构建才会加速
- 如果修改了依赖文件，部分缓存会失效
