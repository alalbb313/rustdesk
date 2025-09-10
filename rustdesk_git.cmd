:: rustdesk_convert_and_sync.cmd
@echo off
setlocal enabledelayedexpansion

:: ===== 配置 =====
set REPO_ROOT=C:\Users\Jerry\Desktop\rustdesk\rustdesk
set SUBMODULE_PATH=libs\hbb_common
set SUBMODULE_BRANCH=main
set SUBMODULE_REMOTE_URL=https://github.com/alalbb313/hbb_common.git

set PARENT_BRANCH=master
:: 可选标签（父仓库）。留空则不打
set TAG_NAME=1.4.1
set TAG_MESSAGE=Release with new defaults
:: ===============

cd /d "%REPO_ROOT%" || (echo Repo root not found: %REPO_ROOT% & exit /b 1)
if "%SUBMODULE_REMOTE_URL%"=="" (echo SUBMODULE_REMOTE_URL is required & exit /b 1)
if not exist "%SUBMODULE_PATH%" (echo Submodule path not found: %SUBMODULE_PATH% & exit /b 1)

:: 检测当前是否已是子模块（gitlink 160000）
set IS_GITLINK=0
for /f "delims=" %%l in ('git ls-files -s "%SUBMODULE_PATH%" 2^>nul') do set LS=%%l
if defined LS (
  if "!LS:~0,6!"=="160000" set IS_GITLINK=1
)

if "%IS_GITLINK%"=="1" (
  echo [detect] %SUBMODULE_PATH% is already a submodule
  goto :PUSH_SUBMODULE_AND_PARENT
)

:: 不是子模块：检测是否已有独立 git 仓库（嵌套 .git）
set HAS_DOTGIT=0
if exist "%SUBMODULE_PATH%\.git" set HAS_DOTGIT=1

if "%HAS_DOTGIT%"=="0" (
  echo [convert] initializing git repo inside %SUBMODULE_PATH%
  pushd "%SUBMODULE_PATH%" || exit /b 1
  git init || exit /b 1
  git add -A
  git commit -m "Initial import from super repo" || echo [submodule] nothing to commit for initial import
  git branch -M "%SUBMODULE_BRANCH%"
  git remote add origin "%SUBMODULE_REMOTE_URL%" 1>nul 2>nul || git remote set-url origin "%SUBMODULE_REMOTE_URL%"
  git push -u origin "refs/heads/%SUBMODULE_BRANCH%:refs/heads/%SUBMODULE_BRANCH%" || echo [submodule] initial push may be up to date
  popd
) else (
  echo [convert] existing nested git detected; syncing it to remote
  pushd "%SUBMODULE_PATH%" || exit /b 1
  git remote set-url origin "%SUBMODULE_REMOTE_URL%" || exit /b 1
  git fetch origin 1>nul 2>nul
  for /f "delims=" %%i in ('git rev-parse --abbrev-ref HEAD 2^>nul') do set CURBR=%%i
  if not defined CURBR (
    git checkout -B "%SUBMODULE_BRANCH%" origin/%SUBMODULE_BRANCH% || git checkout -B "%SUBMODULE_BRANCH%" || exit /b 1
  ) else (
    if /i not "%CURBR%"=="%SUBMODULE_BRANCH%" (
      git checkout "%SUBMODULE_BRANCH%" || git checkout -B "%SUBMODULE_BRANCH%" || exit /b 1
    )
  )
  git merge --ff-only origin/%SUBMODULE_BRANCH% 1>nul 2>nul
  git add -A
  git commit -m "Sync local changes before submodule conversion" || echo [submodule] no changes to commit
  git push -u origin "refs/heads/%SUBMODULE_BRANCH%:refs/heads/%SUBMODULE_BRANCH%" || echo [submodule] push up to date
  popd
)

echo [convert] removing folder from parent index (keep working tree)
git rm -r --cached "%SUBMODULE_PATH%" || echo [parent] nothing to rm from index
git commit -m "Prepare to convert %SUBMODULE_PATH% into submodule" 1>nul 2>nul

echo [convert] adding submodule and linking to remote
git submodule add -f -b "%SUBMODULE_BRANCH%" "%SUBMODULE_REMOTE_URL%" "%SUBMODULE_PATH%" || (
  git submodule add -f "%SUBMODULE_REMOTE_URL%" "%SUBMODULE_PATH%" || exit /b 1
)
git submodule update --init -- "%SUBMODULE_PATH%"
git add .gitmodules "%SUBMODULE_PATH%"
git commit -m "Convert %SUBMODULE_PATH% into submodule tracking %SUBMODULE_BRANCH%" 1>nul 2>nul

:PUSH_SUBMODULE_AND_PARENT
echo [sync] pushing submodule changes (if any)
pushd "%SUBMODULE_PATH%" || exit /b 1
git remote set-url origin "%SUBMODULE_REMOTE_URL%" || exit /b 1
git fetch origin 1>nul 2>nul
for /f "delims=" %%i in ('git rev-parse --abbrev-ref HEAD 2^>nul') do set CURBR=%%i
if not defined CURBR (
  git checkout -B "%SUBMODULE_BRANCH%" origin/%SUBMODULE_BRANCH% || git checkout -B "%SUBMODULE_BRANCH%" || exit /b 1
) else (
  if /i not "%CURBR%"=="%SUBMODULE_BRANCH%" (
    git checkout "%SUBMODULE_BRANCH%" || git checkout -B "%SUBMODULE_BRANCH%" || exit /b 1
  )
)
git merge --ff-only origin/%SUBMODULE_BRANCH% 1>nul 2>nul
git add -A
git commit -m "Sync local changes in hbb_common" 1>nul 2>nul
git push -u origin "refs/heads/%SUBMODULE_BRANCH%:refs/heads/%SUBMODULE_BRANCH%" || echo [submodule] push up to date
popd

echo [sync] recording new submodule pointer and pushing parent
git add "%SUBMODULE_PATH%"
git commit -m "Bump hbb_common submodule pointer" 1>nul 2>nul
git add -A
git commit -m "Sync local changes in super repo" 1>nul 2>nul

:: 规范化父仓库 origin URL（移除可能存在的反引号字符）
for /f "delims=" %%u in ('git remote get-url origin 2^>nul') do set "CUR_ORIGIN_URL=%%u"
set "CLEAN_ORIGIN_URL=!CUR_ORIGIN_URL:`=!"
if not "!CLEAN_ORIGIN_URL!"=="!CUR_ORIGIN_URL!" (
  echo [parent] normalizing origin url: !CLEAN_ORIGIN_URL!
  git remote set-url origin "!CLEAN_ORIGIN_URL!"
)

git push origin "refs/heads/%PARENT_BRANCH%:refs/heads/%PARENT_BRANCH%" || echo [parent] push up to date

if not "%TAG_NAME%"=="" (
  echo [parent] tagging %TAG_NAME%
  git tag -d "%TAG_NAME%" 1>nul 2>nul
  git push -f origin :refs/tags/%TAG_NAME% 1>nul 2>nul
  git tag -a "%TAG_NAME%" -m "%TAG_MESSAGE%"
  git push origin --tags
)

echo Done.
endlocal