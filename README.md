# jwt-cracker-rust

[简体中文](README.zh-CN.md)

`jwt-cracker-rust` is a command-line utility for JWT security testing in authorized environments.

It is intended for learning, lab work, internal assessments, and security testing where you have explicit permission.

> **Authorized use only:** Do not use this project against any system, account, token, service, or dataset unless you are explicitly authorized to test it.

## Features

- Test JWT secrets with a local wordlist.
- Generate an unsigned `alg=none` JWT for compatibility testing.
- Run as a small standalone CLI.
- Build release artifacts automatically through GitHub Actions.

## Installation

Download a prebuilt binary from the GitHub Releases page when available.

You can also build it locally:

```powershell
cargo build --release
```

The binary will be generated under:

```text
target/release/
```

## Usage

```text
Usage: jwt-cracker-rust COMMAND [options]

crack [brute force jwt key]
  -tf <file>    path of token file
  -kf <file>    path of key file
  -em <method>  encryption method of key [none(default), md5, md5_len16, base64]

encode [generate jwt token(alg=none)]
  -pf <file>    path of payload file
```

Examples:

```powershell
jwt-cracker-rust crack -tf token.txt -kf keys.txt
jwt-cracker-rust crack -tf token.txt -kf keys.txt -em base64
jwt-cracker-rust encode -pf payload.json
```

## Safety And Ethics

This tool is provided only for authorized security testing and education.

You are responsible for complying with all applicable laws, policies, and engagement rules. If you are unsure whether you are authorized to test a target, do not use this tool.

## Development

Run the standard checks before submitting changes:

```powershell
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Release

Releases are created by pushing a version tag:

```powershell
git tag v0.1.0
git push origin v0.1.0
```

Release tags must use the `vMAJOR.MINOR.PATCH` format, for example `v0.1.0`.

The release workflow builds platform artifacts and publishes release notes generated with `git-cliff`.
The final GitHub Release page is rendered from `.github/RELEASE_TEMPLATE.md`.

## Contributing

Issues and pull requests are welcome. Please keep changes focused, include tests for behavior changes, and avoid adding features that expand usage beyond authorized testing.

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.
