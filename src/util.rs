use anyhow::{anyhow, Result};
use foundationdb::directory::{Directory, DirectoryError, DirectoryLayer, DirectoryOutput};
use foundationdb::tuple::{Element, TupleUnpack};
use foundationdb::{RangeOption, Transaction};
use futures_util::TryStreamExt;
use owo_colors::OwoColorize;

pub fn parse_path(s: &str) -> Vec<String> {
    let trimmed = s.trim();
    if trimmed == "/" || trimmed.is_empty() {
        return vec![];
    }
    trimmed
        .trim_start_matches('/')
        .split('/')
        .filter(|p| !p.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub async fn dir_for_path(
    trx: &Transaction,
    path: &[String],
) -> Result<DirectoryOutput, DirectoryError> {
    let dl = DirectoryLayer::default();
    if path.is_empty() {
        dl.open(trx, &[], None).await
    } else {
        dl.open(trx, path, None).await
    }
}

pub async fn ls_path(db: &foundationdb::Database, path: Vec<String>) -> Result<()> {
    const SAMPLE: usize = 50;
    db.run(|trx, _| {
        let path = path.clone();
        async move {
            let dl = DirectoryLayer::default();
            if path.is_empty() {
                println!("/:");
            } else {
                println!("/{}:", path.join("/"));
            }

            // Directories
            println!("{}", "Directories:".bold());
            let items = dl.list(&trx, &path).await?;
            if items.is_empty() {
                println!("(none)");
            }
            for name in items {
                let display = format!("{}/", name);
                println!("{}", display.blue().bold());
            }

            // Keys (first N). Skip at root (no content keys at the directory layer root).
            if path.is_empty() {
                return Ok(());
            }
            println!("{}", format!("Keys (first {SAMPLE}):").bold());
            let dir = dir_for_path(&trx, &path).await?;
            let (begin, end) = dir.range()?;
            let mut opt: RangeOption = (begin, end).into();
            opt.limit = Some(SAMPLE + 1);
            let mut i = 0usize;
            let mut more = false;
            let mut stream = trx.get_ranges_keyvalues(opt, true);
            while let Some(item) = stream.try_next().await? {
                i += 1;
                if i > SAMPLE {
                    more = true;
                    break;
                }
                let key = item.key();
                let val = item.value();

                let key_fmt = match dir.unpack::<Element>(key) {
                    Ok(Ok(el)) => format_element(&el),
                    _ => format_bytes(key),
                };
                let val_fmt = match Element::unpack_root(val) {
                    Ok(el) => format_element(&el),
                    Err(_) => try_utf8_or_bytes(val),
                };
                println!(
                    "{} {} {} {}",
                    format!("{i:>4}.").dimmed(),
                    key_fmt.cyan(),
                    "=>".dimmed(),
                    val_fmt.green()
                );
            }
            if i == 0 {
                println!("(none)");
            }
            if more {
                println!(
                    "{} {}",
                    "…".dimmed(),
                    "use 'scan [limit]' to see more".dimmed()
                );
            }

            Ok(())
        }
    })
    .await
    .map_err(|e| anyhow!("{:?}", e))
}

pub async fn scan_path(
    db: &foundationdb::Database,
    path: Vec<String>,
    limit: usize,
    prefix: Option<Vec<u8>>,
    raw_keys: bool,
) -> Result<()> {
    db.run(|trx, _| {
        let path = path.clone();
        let prefix = prefix.clone();
        async move {
            let dir = dir_for_path(&trx, &path).await?;
            let (begin, end) = if let Some(pfx) = prefix.as_ref() {
                let mut start = dir.bytes()?.to_vec();
                start.extend_from_slice(pfx);
                let end = strinc(start.clone());
                (start, end)
            } else {
                dir.range()?
            };

            let mut opt: RangeOption = (begin, end).into();
            opt.limit = Some(limit);

            let mut i = 0usize;
            let mut stream = trx.get_ranges_keyvalues(opt, true);
            println!(
                "-- scanning {} (limit {limit}{}) --",
                display_path(&path).yellow(),
                prefix
                    .as_ref()
                    .map(|p| format!(", prefix {}", format_bytes(p)))
                    .unwrap_or_default()
            );
            while let Some(item) = stream.try_next().await? {
                i += 1;
                let key = item.key();
                let val = item.value();

                let key_fmt = if raw_keys {
                    format_bytes(key)
                } else {
                    match dir.unpack::<Element>(key) {
                        Ok(Ok(el)) => format_element(&el),
                        _ => format_bytes(key),
                    }
                };

                let val_fmt = match Element::unpack_root(val) {
                    Ok(el) => format_element(&el),
                    Err(_) => try_utf8_or_bytes(val),
                };

                println!(
                    "{} {} {} {}",
                    format!("{i:>4}.").dimmed(),
                    key_fmt.cyan(),
                    "=>".dimmed(),
                    val_fmt.green()
                );
            }
            Ok(())
        }
    })
    .await
    .map_err(|e| anyhow!("{:?}", e))
}

pub fn display_path(path: &[String]) -> String {
    if path.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", path.join("/"))
    }
}

pub fn format_element(el: &Element<'_>) -> String {
    match el {
        Element::Nil => "nil".to_string(),
        Element::Bytes(b) => format!("{}", b),
        Element::String(s) => format!("\"{}\"", s),
        Element::Tuple(items) => {
            let mut parts = Vec::with_capacity(items.len());
            for it in items {
                parts.push(format_element(it));
            }
            format!("({})", parts.join(", "))
        }
        Element::Int(i) => format!("{i}"),
        Element::Float(f) => format!("{}f32", f),
        Element::Double(d) => format!("{}f64", d),
        Element::Bool(b) => format!("{b}"),
        Element::Uuid(u) => format!("uuid:{u}"),
        Element::Versionstamp(vs) => format!("versionstamp:{}", hex::encode(vs.as_bytes())),
    }
}

pub fn try_utf8_or_bytes(b: &[u8]) -> String {
    match std::str::from_utf8(b) {
        Ok(s)
            if s.chars()
                .all(|c| !c.is_control() || c == '\n' || c == '\r' || c == '\t') =>
        {
            format!("\"{}\"", s)
        }
        _ => format_bytes(b),
    }
}

pub fn format_bytes(b: &[u8]) -> String {
    const MAX: usize = 64;
    let mut out = String::new();
    out.push_str("b\"");
    for (idx, byte) in b.iter().enumerate() {
        if idx >= MAX {
            out.push_str("…");
            break;
        }
        if *byte == b'\\' {
            out.push_str(r"\\");
        } else if byte.is_ascii_alphanumeric() || *byte == b'-' || *byte == b'_' {
            out.push(*byte as char);
        } else {
            out.push_str(&format!("\\x{:02x}", byte));
        }
    }
    out.push('"');
    out
}

pub fn parse_bytes_literal(s: &str) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\\' && i + 1 < bytes.len() {
            let n = bytes[i + 1];
            match n {
                b'x' | b'X' => {
                    if i + 3 >= bytes.len() {
                        return Err(anyhow!("incomplete hex escape"));
                    }
                    let h1 = bytes[i + 2] as char;
                    let h2 = bytes[i + 3] as char;
                    let v = (hex_val(h1)? << 4) | hex_val(h2)?;
                    out.push(v);
                    i += 4;
                    continue;
                }
                b'n' => {
                    out.push(b'\n');
                    i += 2;
                    continue;
                }
                b'r' => {
                    out.push(b'\r');
                    i += 2;
                    continue;
                }
                b't' => {
                    out.push(b'\t');
                    i += 2;
                    continue;
                }
                b'\\' => {
                    out.push(b'\\');
                    i += 2;
                    continue;
                }
                b'"' => {
                    out.push(b'"');
                    i += 2;
                    continue;
                }
                _ => {
                    out.push(n);
                    i += 2;
                    continue;
                }
            }
        }
        out.push(c);
        i += 1;
    }
    Ok(out)
}

fn hex_val(c: char) -> Result<u8> {
    match c {
        '0'..='9' => Ok((c as u8) - b'0'),
        'a'..='f' => Ok((c as u8) - b'a' + 10),
        'A'..='F' => Ok((c as u8) - b'A' + 10),
        _ => Err(anyhow!("invalid hex digit")),
    }
}

fn strinc(mut key: Vec<u8>) -> Vec<u8> {
    for i in (0..key.len()).rev() {
        if key[i] != 0xff {
            key[i] += 1;
            key.truncate(i + 1);
            return key;
        }
    }
    Vec::new()
}
