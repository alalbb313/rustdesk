# macOS 安装指南 / macOS Installation Guide

## 中文

由于此应用未经过 Apple 公证，首次打开时 macOS 会阻止运行。请按以下步骤操作：

### 第一步：安装应用

#### 方法 1：右键打开（推荐）

1. 下载 `.dmg` 文件
2. 双击打开 DMG，将 RustDesk 拖到 Applications 文件夹
3. **不要**直接双击 RustDesk.app
4. 打开 **访达（Finder）** → **应用程序（Applications）**
5. 找到 RustDesk，**右键点击** → 选择 **打开（Open）**
6. 在弹出的警告对话框中，点击 **打开（Open）**
7. 以后就可以正常双击打开了

#### 方法 2：使用终端命令

打开 **终端（Terminal）**，执行以下命令：

```bash
# 移除隔离属性
xattr -cr /Applications/RustDesk.app

# 或者，如果应用在其他位置
xattr -cr ~/Downloads/RustDesk.app
```

#### 方法 3：系统设置（macOS Ventura 13.0+）

1. 尝试打开 RustDesk（会被阻止）
2. 打开 **系统设置（System Settings）** → **隐私与安全性（Privacy & Security）**
3. 滚动到底部，找到 "RustDesk 已被阻止" 的提示
4. 点击 **仍要打开（Open Anyway）**
5. 在弹出的对话框中点击 **打开（Open）**

### 第二步：授予必要权限

RustDesk 需要以下权限才能正常工作。**首次运行时，应用会自动请求这些权限**。

#### 1. 辅助功能（Accessibility）权限

**用途**：允许 RustDesk 控制鼠标和键盘

**自动请求**：首次运行时，RustDesk 会自动弹出权限请求对话框

**手动配置**：
1. 打开 **系统设置** → **隐私与安全性** → **辅助功能**
2. 点击左下角的 🔒 解锁
3. 找到 **RustDesk**，勾选复选框
4. 如果没有看到 RustDesk，点击 **+** 按钮，从应用程序文件夹中添加

#### 2. 屏幕录制（Screen Recording）权限

**用途**：允许 RustDesk 捕获屏幕内容

**自动请求**：首次运行时，RustDesk 会自动弹出权限请求对话框

**手动配置**：
1. 打开 **系统设置** → **隐私与安全性** → **屏幕录制**
2. 点击左下角的 🔒 解锁
3. 找到 **RustDesk**，勾选复选框
4. **重启 RustDesk** 使权限生效

#### 3. 输入监控（Input Monitoring）权限（macOS 11.0+）

**用途**：允许 RustDesk 监控键盘输入

**自动请求**：首次运行时，RustDesk 会自动弹出权限请求对话框

**手动配置**：
1. 打开 **系统设置** → **隐私与安全性** → **输入监控**
2. 点击左下角的 🔒 解锁
3. 找到 **RustDesk**，勾选复选框

### 常见问题

**Q: 为什么需要这么多权限？**
A: RustDesk 是远程桌面软件，需要这些权限才能实现远程控制功能。所有权限都是 macOS 系统要求的标准权限。

**Q: 授予权限后仍然无法使用？**
A: 请尝试：
1. 完全退出 RustDesk（右键 Dock 图标 → 退出）
2. 重新打开 RustDesk
3. 如果仍然不行，重启 Mac

**Q: 可以撤销权限吗？**
A: 可以。在系统设置中取消勾选对应权限即可。

---

## English

Since this app is not notarized by Apple, macOS will block it on first launch. Please follow these steps:

### Step 1: Install the Application

#### Method 1: Right-click to Open (Recommended)

1. Download the `.dmg` file
2. Double-click to open the DMG and drag RustDesk to Applications folder
3. **Do NOT** double-click RustDesk.app directly
4. Open **Finder** → **Applications**
5. Find RustDesk, **right-click** → select **Open**
6. In the warning dialog, click **Open**
7. You can now open it normally in the future

#### Method 2: Using Terminal

Open **Terminal** and run:

```bash
# Remove quarantine attribute
xattr -cr /Applications/RustDesk.app

# Or, if the app is in another location
xattr -cr ~/Downloads/RustDesk.app
```

#### Method 3: System Settings (macOS Ventura 13.0+)

1. Try to open RustDesk (it will be blocked)
2. Open **System Settings** → **Privacy & Security**
3. Scroll to the bottom, find "RustDesk was blocked" message
4. Click **Open Anyway**
5. In the confirmation dialog, click **Open**

### Step 2: Grant Necessary Permissions

RustDesk requires the following permissions to function properly. **The app will automatically request these permissions on first launch.**

#### 1. Accessibility Permission

**Purpose**: Allows RustDesk to control mouse and keyboard

**Automatic Request**: RustDesk will automatically prompt for this permission on first launch

**Manual Configuration**:
1. Open **System Settings** → **Privacy & Security** → **Accessibility**
2. Click the 🔒 lock icon to unlock
3. Find **RustDesk** and check the checkbox
4. If you don't see RustDesk, click **+** and add it from Applications folder

#### 2. Screen Recording Permission

**Purpose**: Allows RustDesk to capture screen content

**Automatic Request**: RustDesk will automatically prompt for this permission on first launch

**Manual Configuration**:
1. Open **System Settings** → **Privacy & Security** → **Screen Recording**
2. Click the 🔒 lock icon to unlock
3. Find **RustDesk** and check the checkbox
4. **Restart RustDesk** for the permission to take effect

#### 3. Input Monitoring Permission (macOS 11.0+)

**Purpose**: Allows RustDesk to monitor keyboard input

**Automatic Request**: RustDesk will automatically prompt for this permission on first launch

**Manual Configuration**:
1. Open **System Settings** → **Privacy & Security** → **Input Monitoring**
2. Click the 🔒 lock icon to unlock
3. Find **RustDesk** and check the checkbox

### FAQ

**Q: Why does it need so many permissions?**
A: RustDesk is remote desktop software and requires these permissions to enable remote control functionality. All permissions are standard macOS system requirements.

**Q: Still not working after granting permissions?**
A: Please try:
1. Completely quit RustDesk (right-click Dock icon → Quit)
2. Reopen RustDesk
3. If still not working, restart your Mac

**Q: Can I revoke permissions?**
A: Yes. Uncheck the corresponding permissions in System Settings.

---

## 为什么会这样？/ Why does this happen?

**中文**：
- 此应用使用自签名证书（ad-hoc signing），未经过 Apple 的公证流程
- Apple 要求所有在 App Store 外分发的应用都需要开发者证书（$99/年）
- 我们选择免费分发，因此需要用户手动信任

**English**:
- This app uses ad-hoc signing and is not notarized by Apple
- Apple requires all apps distributed outside the App Store to have a developer certificate ($99/year)
- We chose to distribute for free, so users need to manually trust the app

---

## 安全性 / Security

**中文**：
- 本项目是开源的，你可以在 GitHub 上查看所有源代码
- 你也可以选择自己编译应用

**English**:
- This project is open source, you can review all source code on GitHub
- You can also choose to compile the app yourself
