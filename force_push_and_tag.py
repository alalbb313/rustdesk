import subprocess
import sys
import re
import argparse

# ------------------------
# 内置默认参数（当用户没有外部传入时使用）
# ------------------------
DEFAULT_TAG = "1.4.4"          # 默认 tag
ENABLE_AUTO_COMMIT = True      # 自动 git add + commit
ENABLE_FORCE_PUSH = True       # 默认使用 -f 强制 push
ENABLE_BRANCH_PROTECT = False   # 保护 master/main 默认不允许覆盖
ENABLE_AUTO_INCREMENT_TAG = False   # 自动递增 tag（仅当未指定 tag 且未传外部 tag 时）
ENABLE_DELETE_OLD_TAG = True   # 删除远程已有同名 tag
ENABLE_CREATE_TAG = True       # 创建新 tag
ENABLE_QUIET = False           # 静默模式（隐藏 git 输出）

# ------------------------
# 辅助函数
# ------------------------
def run(cmd, quiet=False, exit_on_error=True):
    if not quiet:
        print(f"➡️ {' '.join(cmd)}")
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    if not quiet:
        print(result.stdout)
    if exit_on_error and result.returncode != 0:
        print("❌ 命令执行失败，已终止")
        sys.exit(1)
    return result.stdout.strip()

def get_current_branch():
    return run(["git", "rev-parse", "--abbrev-ref", "HEAD"])

def get_latest_tag():
    output = run(["git", "tag"], exit_on_error=False)
    tags = output.split("\n") if output else []
    tags = [t for t in tags if re.match(r"^\d+\.\d+\.\d+$", t)]
    return sorted(tags, key=lambda x: list(map(int, x.split("."))))[-1] if tags else None

def increment_tag(tag):
    major, minor, patch = map(int, tag.split("."))
    return f"{major}.{minor}.{patch + 1}"

# ------------------------
# 主函数
# ------------------------
def main():
    parser = argparse.ArgumentParser(description="Ultimate Git Push Tool（外部参数优先，内置参数作为默认值）")

    # 外部参数
    parser.add_argument("--tag", help="指定要创建的 tag（优先于内置默认 tag）")
    parser.add_argument("--no-commit", action="store_true", help="禁用自动 add + commit")
    parser.add_argument("--no-force", action="store_true", help="禁用强制 push（-f）")
    parser.add_argument("--no-protect", action="store_true", help="允许覆盖 master/main（默认保护）")
    parser.add_argument("--no-autotag", action="store_true", help="禁用自动递增 tag")
    parser.add_argument("--no-delete-tag", action="store_true", help="禁用删除远程旧 tag")
    parser.add_argument("--no-tag", action="store_true", help="禁用创建新 tag")
    parser.add_argument("--quiet", action="store_true", help="静默模式")

    args = parser.parse_args()

    # ------------------------
    # 合并内置参数和外部参数
    # 外部参数优先，其次才使用内置默认
    # ------------------------
    tag_to_use = args.tag if args.tag else DEFAULT_TAG
    auto_commit = ENABLE_AUTO_COMMIT and not args.no_commit
    force_push = ENABLE_FORCE_PUSH and not args.no_force
    branch_protect = ENABLE_BRANCH_PROTECT and not args.no_protect
    auto_increment_tag = ENABLE_AUTO_INCREMENT_TAG and not args.no_autotag
    delete_old_tag = ENABLE_DELETE_OLD_TAG and not args.no_delete_tag
    create_tag = ENABLE_CREATE_TAG and not args.no_tag
    quiet = ENABLE_QUIET or args.quiet

    print("\n=== 🚀 Ultimate Git Push Tool (外部参数优先 + 内置默认参数) ===\n")

    # 当前仓库状态
    run(["git", "status"], quiet)

    # 当前分支
    branch = get_current_branch()
    print(f"📌 当前分支：{branch}")

    if branch in ("master", "main") and branch_protect:
        print("❌ 默认禁止覆盖 master/main。使用 --no-protect 可关闭保护")
        sys.exit(1)

    # ------------------------
    # 自动提交
    # ------------------------
    if auto_commit:
        run(["git", "add", "--all"], quiet)
        run(["git", "commit", "-m", f"Auto update for tag {tag_to_use}"], quiet, exit_on_error=False)
    else:
        print("⚠️ 已禁用自动 commit")

    # ------------------------
    # 强制/普通 push
    # ------------------------
    push_cmd = ["git", "push", "origin", branch]
    if force_push:
        push_cmd.append("-f")
    run(push_cmd, quiet)

    # ------------------------
    # Tag 管理
    # ------------------------
    if not create_tag:
        print("⚠️ 已禁用 tag 创建，流程结束。")
        sys.exit(0)

    # 自动递增 tag（当没有外部指定 tag 时生效）
    if not args.tag and auto_increment_tag:
        latest_tag = get_latest_tag()
        if latest_tag:
            tag_to_use = increment_tag(latest_tag)
            print(f"📌 自动递增 tag：{latest_tag} → {tag_to_use}")
        else:
            print(f"📌 当前无历史 tag，使用默认内置 tag：{tag_to_use}")

    # 删除远程旧 tag
    if delete_old_tag:
        run(["git", "tag", "-d", tag_to_use], quiet, exit_on_error=False)
        run(["git", "push", "origin", f":refs/tags/{tag_to_use}"], quiet, exit_on_error=False)
    else:
        print("⚠️ 已禁用删除远程旧 tag")

    # 创建新 tag 并 push
    run(["git", "tag", tag_to_use], quiet)
    run(["git", "push", "origin", tag_to_use], quiet)

    print(f"\n🎉 完成：远程仓库已覆盖，tag={tag_to_use}\n")


if __name__ == "__main__":
    main()
