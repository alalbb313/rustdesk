# 如何获取和配置 RustDesk 服务器公钥

## 问题
如果您遇到错误：`Failed to secure tcp: Signature mismatch in key exchange`

这表示客户端配置的公钥与服务器的公钥不匹配。

## 解决步骤

### 1. 获取服务器公钥

在您的 RustDesk 服务器上运行以下命令：

```bash
# 方法1: 直接读取公钥文件
cat /var/lib/rustdesk-server/id_ed25519.pub

# 方法2: 从服务器日志中查看
# 服务器启动时会显示公钥
journalctl -u rustdesk-hbbs -n 100 | grep "Public Key"

# 方法3: 使用 hbbs 命令
cd /path/to/rustdesk-server
./hbbs --key
```

输出示例：
```
YOURKEY1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ=
```

### 2. 更新客户端代码

打开文件：`libs/hbb_common/src/config.rs`

找到 `get_key()` 函数（大约在第1084行），将您的服务器公钥替换进去：

```rust
pub fn get_key() -> String {
    let v = Self::get_option(keys::OPTION_KEY);
    if v.is_empty() {
        // 替换为您的实际服务器公钥
        "YOUR_ACTUAL_SERVER_PUBLIC_KEY_HERE".to_owned()
    } else {
        v
    }
}
```

### 3. 当前配置（临时）

**当前代码已设置为空密钥，这意味着不进行密钥验证。**

这是为了测试连接，**不建议在生产环境使用**。

### 4. 生产环境配置

获取到正确的服务器公钥后，请：

1. 替换 `get_key()` 函数中的空字符串为实际公钥
2. 重新编译客户端
3. 测试连接

### 5. 验证配置

连接成功后，您应该能够：
- 正常建立连接
- 不再看到 "Signature mismatch" 错误
- 安全地进行远程控制

## 安全说明

⚠️ **重要**: 
- 空密钥（当前配置）= 不验证服务器身份 = 安全风险
- 生产环境必须配置正确的公钥
- 公钥验证可以防止中间人攻击

## 备选方案

如果您不想硬编码公钥，可以：
1. 在UI中手动输入公钥
2. 通过配置文件设置
3. 使用环境变量

当前实现支持通过UI设置，用户可以在"网络"设置中输入Key。
