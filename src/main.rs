use anyhow::{Context, Result, bail};
use jwt_cracker_rust::{KeyEncoding, crack_token_file, encode_none_token};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug)]
enum Command {
    Crack {
        token_file: PathBuf,
        key_file: PathBuf,
        encoding: KeyEncoding,
    },
    Encode {
        payload_file: PathBuf,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match parse_args(std::env::args().skip(1))? {
        Command::Crack {
            token_file,
            key_file,
            encoding,
        } => {
            let start = Instant::now();
            match crack_token_file(&token_file, &key_file, encoding)? {
                Some(key) => println!("found key: {key}"),
                None => println!("key not found"),
            }
            println!("Execution time:[{}]", format_duration(start.elapsed()));
        }
        Command::Encode { payload_file } => {
            let payload = std::fs::read(&payload_file)
                .with_context(|| format!("读取 payload 文件失败: {}", payload_file.display()))?;
            println!("{}", encode_none_token(&payload));
        }
    }

    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Command> {
    let mut args = args.into_iter();
    let command = args.next().ok_or_else(usage_error)?;

    match command.as_str() {
        "crack" => parse_crack_args(args),
        "encode" => parse_encode_args(args),
        _ => Err(usage_error()),
    }
}

fn parse_crack_args(mut args: impl Iterator<Item = String>) -> Result<Command> {
    let mut token_file = None;
    let mut key_file = None;
    let mut encoding = KeyEncoding::None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-tf" => token_file = Some(next_path(&mut args, "-tf")?),
            "-kf" => key_file = Some(next_path(&mut args, "-kf")?),
            "-em" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("-em 缺少参数值"))?;
                encoding = KeyEncoding::parse(&value)?;
            }
            _ => bail!("未知参数: {arg}\n\n{}", usage()),
        }
    }

    Ok(Command::Crack {
        token_file: token_file.ok_or_else(|| anyhow::anyhow!("缺少 -tf 参数\n\n{}", usage()))?,
        key_file: key_file.ok_or_else(|| anyhow::anyhow!("缺少 -kf 参数\n\n{}", usage()))?,
        encoding,
    })
}

fn parse_encode_args(mut args: impl Iterator<Item = String>) -> Result<Command> {
    let mut payload_file = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-pf" => payload_file = Some(next_path(&mut args, "-pf")?),
            _ => bail!("未知参数: {arg}\n\n{}", usage()),
        }
    }

    Ok(Command::Encode {
        payload_file: payload_file
            .ok_or_else(|| anyhow::anyhow!("缺少 -pf 参数\n\n{}", usage()))?,
    })
}

fn next_path(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<PathBuf> {
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("{flag} 缺少路径参数"))
}

fn usage_error() -> anyhow::Error {
    anyhow::anyhow!("{}", usage())
}

fn usage() -> &'static str {
    "Usage: jwt-cracker-rust COMMAND [options]\n\n\
crack [brute force jwt key]\n  -tf <file>    path of token file\n  -kf <file>    path of key file\n  -em <method>  encryption method of key [none(default), md5, md5_len16, base64]\n\n\
encode [generate jwt token(alg=none)]\n  -pf <file>    path of payload file"
}

fn format_duration(duration: std::time::Duration) -> String {
    format!("{duration:.3?}")
}
