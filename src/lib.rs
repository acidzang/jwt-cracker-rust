use anyhow::{Context, Result, anyhow, bail};
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha384, Sha512};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;
type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEncoding {
    None,
    Md5,
    Md5Len16,
    Base64,
}

impl KeyEncoding {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "none" => Ok(Self::None),
            "md5" => Ok(Self::Md5),
            "md5_len16" => Ok(Self::Md5Len16),
            "base64" => Ok(Self::Base64),
            _ => bail!("不支持的密钥编码方式: {value}"),
        }
    }

    pub fn apply(self, key: &str) -> String {
        match self {
            Self::None => key.to_owned(),
            Self::Md5 => format!("{:x}", md5::compute(key.as_bytes())),
            Self::Md5Len16 => {
                let digest = format!("{:x}", md5::compute(key.as_bytes()));
                digest[8..24].to_owned()
            }
            Self::Base64 => STANDARD.encode(key.as_bytes()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JwtAlgorithm {
    Hs256,
    Hs384,
    Hs512,
}

impl JwtAlgorithm {
    fn parse(header_json: &[u8]) -> Result<Self> {
        let header: serde_json::Value =
            serde_json::from_slice(header_json).context("JWT header 不是有效 JSON")?;
        let alg = header
            .get("alg")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| anyhow!("JWT header 缺少 alg 字段"))?;

        if alg == "HS256" {
            Ok(Self::Hs256)
        } else if alg == "HS384" {
            Ok(Self::Hs384)
        } else if alg == "HS512" {
            Ok(Self::Hs512)
        } else {
            bail!("仅支持 HS256、HS384、HS512 签名算法")
        }
    }
}

#[derive(Debug)]
struct ParsedJwt<'a> {
    signing_input: &'a str,
    signature: Vec<u8>,
    algorithm: JwtAlgorithm,
}

impl<'a> ParsedJwt<'a> {
    fn parse(token: &'a str) -> Result<Self> {
        let token = token.trim();
        let mut parts = token.split('.');
        let header = parts.next().ok_or_else(|| anyhow!("JWT 缺少 header"))?;
        let payload = parts.next().ok_or_else(|| anyhow!("JWT 缺少 payload"))?;
        let signature = parts.next().ok_or_else(|| anyhow!("JWT 缺少 signature"))?;

        if parts.next().is_some() {
            bail!("JWT 格式错误: 只能包含 header.payload.signature 三段");
        }

        let header_json = URL_SAFE_NO_PAD
            .decode(header)
            .context("JWT header base64url 解码失败")?;
        let algorithm = JwtAlgorithm::parse(&header_json)?;
        let signature = URL_SAFE_NO_PAD
            .decode(signature)
            .context("JWT signature base64url 解码失败")?;

        Ok(Self {
            signing_input: &token[..header.len() + 1 + payload.len()],
            signature,
            algorithm,
        })
    }
}

pub fn crack_token_file(
    token_path: impl AsRef<Path>,
    key_path: impl AsRef<Path>,
    encoding: KeyEncoding,
) -> Result<Option<String>> {
    let token = std::fs::read_to_string(token_path.as_ref())
        .with_context(|| format!("读取 token 文件失败: {}", token_path.as_ref().display()))?;
    let parsed = ParsedJwt::parse(&token)?;
    let file = File::open(key_path.as_ref())
        .with_context(|| format!("读取 key 文件失败: {}", key_path.as_ref().display()))?;

    crack_with_reader(&parsed, file, encoding)
}

fn crack_with_reader(
    parsed: &ParsedJwt<'_>,
    key_reader: impl std::io::Read,
    encoding: KeyEncoding,
) -> Result<Option<String>> {
    for line in BufReader::new(key_reader).lines() {
        let raw_key = line.context("读取 key 字典行失败")?;
        let key = encoding.apply(raw_key.trim_end_matches('\r'));

        if verify_signature(parsed, key.as_bytes())? {
            return Ok(Some(key));
        }
    }

    Ok(None)
}

fn verify_signature(parsed: &ParsedJwt<'_>, key: &[u8]) -> Result<bool> {
    match parsed.algorithm {
        JwtAlgorithm::Hs256 => verify_hmac::<HmacSha256>(parsed, key),
        JwtAlgorithm::Hs384 => verify_hmac::<HmacSha384>(parsed, key),
        JwtAlgorithm::Hs512 => verify_hmac::<HmacSha512>(parsed, key),
    }
}

fn verify_hmac<M>(parsed: &ParsedJwt<'_>, key: &[u8]) -> Result<bool>
where
    M: Mac + hmac::digest::KeyInit,
{
    let mut mac = <M as hmac::digest::KeyInit>::new_from_slice(key)
        .map_err(|_| anyhow!("初始化 HMAC 失败"))?;
    mac.update(parsed.signing_input.as_bytes());
    Ok(mac.verify_slice(&parsed.signature).is_ok())
}

pub fn encode_none_token(payload: &[u8]) -> String {
    let header = br#"{"alg":"none","typ":"JWT"}"#;
    format!(
        "{}.{}.",
        URL_SAFE_NO_PAD.encode(header),
        URL_SAFE_NO_PAD.encode(payload)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sign_hs256(signing_input: &str, key: &[u8]) -> String {
        let mut mac = HmacSha256::new_from_slice(key).expect("测试 key 有效");
        mac.update(signing_input.as_bytes());
        URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
    }

    #[test]
    fn key_encoding_matches_go_behavior() {
        assert_eq!(KeyEncoding::None.apply("123"), "123");
        assert_eq!(
            KeyEncoding::Md5.apply("123"),
            "202cb962ac59075b964b07152d234b70"
        );
        assert_eq!(KeyEncoding::Md5Len16.apply("123"), "ac59075b964b0715");
        assert_eq!(KeyEncoding::Base64.apply("zzzr"), "enp6cg==");
    }

    #[test]
    fn encode_none_token_contains_trailing_signature_separator() {
        let token = encode_none_token(br#"{"sub":"123"}"#);
        assert_eq!(
            token,
            "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjMifQ."
        );
    }

    #[test]
    fn crack_finds_matching_key() {
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(br#"{"sub":"123"}"#);
        let signing_input = format!("{header}.{payload}");
        let signature = sign_hs256(&signing_input, b"secret");
        let token = format!("{signing_input}.{signature}");
        let parsed = ParsedJwt::parse(&token).expect("JWT 可解析");

        let found = crack_with_reader(&parsed, "admin\nsecret\n".as_bytes(), KeyEncoding::None)
            .expect("爆破流程成功");

        assert_eq!(found.as_deref(), Some("secret"));
    }
}
