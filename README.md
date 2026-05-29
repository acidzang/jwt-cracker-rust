# jwt-cracker-rust

`jwt-cracker-rust` 是一个用 Rust 编写的 JWT 安全测试工具，参考 Go 版本 `alwaystest18/jwtCracker` 实现。

当前支持两类功能：

- `crack`：基于字典暴力测试 JWT HMAC 密钥。
- `encode`：生成 `alg=none` 的 JWT token。

> 仅限在明确授权的安全测试、靶场或本地研究环境中使用。

## 功能特性

- 支持 JWT 签名算法：
  - `HS256`
  - `HS384`
  - `HS512`
- 支持密钥字典变换：
  - `none`：原始字典值
  - `md5`：32 位 MD5
  - `md5_len16`：16 位 MD5，即 32 位 MD5 的第 9 到 24 位
  - `base64`：标准 Base64 编码
- 字典逐行读取，适合较大的 key 文件。
- 使用 Rust HMAC 实现本地签名校验，不依赖外部 JWT 命令行工具。

## 环境要求

- Rust toolchain
- Cargo

查看版本：

```powershell
rustc --version
cargo --version
```

## 构建

开发构建：

```powershell
cargo build
```

发布构建：

```powershell
cargo build --release
```

生成的可执行文件位置：

```text
target\release\jwt-cracker-rust.exe
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

## 暴力破解 JWT 密钥

准备 token 文件，例如 `token.txt`：

```text
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjMifQ.xxxxx
```

准备 key 字典文件，例如 `keys.txt`：

```text
123
123456
admin
secret
```

使用原始字典值测试：

```powershell
cargo run -- crack -tf token.txt -kf keys.txt
```

对字典值做 Base64 后测试：

```powershell
cargo run -- crack -tf token.txt -kf keys.txt -em base64
```

对字典值做 32 位 MD5 后测试：

```powershell
cargo run -- crack -tf token.txt -kf keys.txt -em md5
```

对字典值做 16 位 MD5 后测试：

```powershell
cargo run -- crack -tf token.txt -kf keys.txt -em md5_len16
```

找到密钥时输出：

```text
found key: secret
Execution time:[1.234s]
```

未找到密钥时输出：

```text
key not found
Execution time:[1.234s]
```

## 生成 alg=none Token

准备 payload 文件，例如 `payload.txt`：

```json
{
  "sub": "1234567890",
  "name": "John Doe",
  "iat": 1516239022
}
```

生成 token：

```powershell
cargo run -- encode -pf payload.txt
```

输出格式：

```text
<base64url(header)>.<base64url(payload)>.
```

## 测试与检查

格式化代码：

```powershell
cargo fmt
```

运行单元测试：

```powershell
cargo test
```

运行 Clippy 静态检查：

```powershell
cargo clippy --all-targets --all-features -- -D warnings
```

## 项目结构

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src
    ├── lib.rs      # JWT 解析、HMAC 校验、密钥变换、alg=none 生成
    └── main.rs     # CLI 参数解析和命令分发
```

## 说明

本项目用于学习和授权安全测试。请勿对未授权目标使用，否则可能违反法律法规。
