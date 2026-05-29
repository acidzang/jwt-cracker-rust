# jwt-cracker-rust

[English](README.md)

`jwt-cracker-rust` 是一个用于授权 JWT 安全测试的命令行工具。

它适用于学习、靶场、本地研究、内部评估，以及已经获得明确授权的安全测试场景。

> **仅限授权使用：** 除非你已经被明确授权测试相关系统、账号、token、服务或数据，否则不要使用本项目。

## 功能

- 使用本地字典测试 JWT 密钥。
- 生成 `alg=none` JWT，用于兼容性测试。
- 以小型独立 CLI 形式运行。
- 通过 GitHub Actions 自动构建发布产物。

## 安装

可优先从 GitHub Releases 页面下载预构建二进制文件。

也可以在本地构建：

```powershell
cargo build --release
```

二进制文件会生成在：

```text
target/release/
```

## 用法

```text
Usage: jwt-cracker-rust COMMAND [options]

crack [brute force jwt key]
  -tf <file>    path of token file
  -kf <file>    path of key file
  -em <method>  encryption method of key [none(default), md5, md5_len16, base64]

encode [generate jwt token(alg=none)]
  -pf <file>    path of payload file
```

示例：

```powershell
jwt-cracker-rust crack -tf token.txt -kf keys.txt
jwt-cracker-rust crack -tf token.txt -kf keys.txt -em base64
jwt-cracker-rust encode -pf payload.json
```

## 安全与伦理

本工具仅用于授权安全测试和教育目的。

你需要自行确保使用行为符合适用法律、组织政策和测试授权范围。如果不确定是否具备授权，请不要使用本工具。

## 开发

提交变更前请运行标准检查：

```powershell
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## 发布

推送版本 tag 即可触发发布：

```powershell
git tag v0.1.0
git push origin v0.1.0
```

Release tag 必须使用 `vMAJOR.MINOR.PATCH` 格式，例如 `v0.1.0`。

发布 workflow 会构建平台产物，并使用 `git-cliff` 生成发布说明。
最终 GitHub Release 页面会基于 `.github/RELEASE_TEMPLATE.md` 渲染生成。

## 贡献

欢迎提交 Issue 和 Pull Request。请保持变更聚焦，为行为变更补充测试，并避免加入超出授权测试定位的功能。

## 许可证

本项目基于 [LICENSE](LICENSE) 文件中的条款授权。
