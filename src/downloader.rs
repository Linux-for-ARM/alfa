//! Download files from internet

use anyhow::{Error, Result};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use md5::compute;
use reqwest::Client;
use std::{
    cmp::min,
    fs::{read, File},
    io::Write,
    path::Path,
};

#[tokio::main]
pub async fn download<U, P>(client: &Client, url: U, path: Option<P>, prefix: P) -> Result<()>
where
    U: ToString,
    P: AsRef<Path>,
{
    let disp_url = url.to_string();
    let res = client
        .get(url.to_string())
        .send()
        .await
        .or(Err(Error::msg(format!(
            "Failed to GET from '{}'",
            &disp_url.dimmed(),
        ))))?;

    let path = match path {
        Some(p) => p.as_ref().display().to_string(),
        None => disp_url
            .rsplit_once('/')
            .unwrap_or(("tmp.bin", "tmp.bin"))
            .1
            .to_string(),
    };
    let prefix = prefix.as_ref();

    let total_size = res.content_length().unwrap_or(u64::MAX) / 1024;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:20}] {percent}% {elapsed}/{eta} {bytes_per_sec}")?
            .progress_chars("#> "),
    );
    let hdr = format!(
        "{:<width$}",
        format!("Downloading '{}'", compress_name(&path, 20).dimmed()),
        width = 45
    );
    pb.set_message(hdr);

    let mut file = File::create(prefix.join(path))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64 / 1024), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    pb.finish();

    Ok(())
}

fn compress_name(name: &str, size_max: usize) -> String {
    let len = name.len() - 1;
    if size_max > len {
        return name.to_string();
    }
    let mut unneeded_chars = if len > size_max { 0 } else { size_max - len };
    if unneeded_chars % 2 == 0 {
        unneeded_chars /= 2;
    }
    unneeded_chars += 1;

    let chars_cnt = len / 2;
    let chars_first = if chars_cnt > unneeded_chars {
        chars_cnt - unneeded_chars
    } else {
        chars_cnt
    };

    let mut i = 0;
    let msg_chars = name.chars().collect::<Vec<_>>();
    let mut msg_new_chars = Vec::with_capacity(size_max);

    while i < chars_first {
        msg_new_chars.push(msg_chars[i]);
        i += 1;
    }

    let mut j = 1;
    while j <= 3 {
        msg_new_chars.push('.');
        j += 1;
    }

    i += 4;
    while i <= len {
        msg_new_chars.push(msg_chars[i]);
        i += 1;
    }

    let mut s = String::new();
    for ch in msg_new_chars {
        s.push(ch);
    }

    if s.contains("....") {
        s = s.replace("....", "...");
    }

    s
}

pub fn check_md5<P: AsRef<Path>>(pth: P, md5: &str) -> Result<bool> {
    let data = read(&pth)?;
    let digest = compute(&data);
    let a = format!("{:?}", digest);

    Ok(&a == md5)
}
