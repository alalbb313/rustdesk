import subprocess
import sys

def run(cmd):
    print(f"➡️ 执行：{' '.join(cmd)}")
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    print(result.stdout)
    if result.returncode != 0:
        print("❌ 命令执行失败，已终止")
        sys.exit(1)

def main():
    tag_name = "1.4.4"

    print("\n=== 🚀 强制覆盖 GitHub 远程仓库并重新生成 TAG ===\n")

    # 1. 确保在 git 仓库内
    run(["git", "status"])

    # 2. 添加所有文件
    run(["git", "add", "--all"])

    # 3. 提交
    run(["git", "commit", "-m", f"Force update for tag {tag_name}"])

    # 4. 强制 push 到远程 main/master（自动判断）
    # 获取当前分支
    res = subprocess.run(["git", "rev-parse", "--abbrev-ref", "HEAD"],
                         stdout=subprocess.PIPE, text=True)
    branch = res.stdout.strip()
    print(f"当前分支：{branch}")

    run(["git", "push", "origin", branch, "-f"])

    # 5. 删除远程旧 tag（如果存在）
    run(["git", "tag", "-d", tag_name])
    run(["git", "push", "origin", f":refs/tags/{tag_name}"])

    # 6. 重新创建 tag
    run(["git", "tag", tag_name])
    run(["git", "push", "origin", tag_name])

    print("\n✅ 完成：远程仓库已被强制覆盖并创建新 tag 1.4.4\n")

if __name__ == "__main__":
    main()
