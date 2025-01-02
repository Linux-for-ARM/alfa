//! Download files from internet

use anyhow::{Error, Result};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{cmp::min, fs::File, io::Write, path::Path};

pub async fn download<U, P>(client: &Client, url: U, path: P) -> Result<()>
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

    // Size of downloaded file in Kbytes
    let total_size = res.content_length().ok_or(Error::msg(format!(
        "Failed to get content length from '{}'",
        &disp_url.dimmed()
    )))? / 1024;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:10}] {percent} {elapsed}/{eta} {bytes_per_sec}")?
            .progress_chars("#> "),
    );
    pb.set_message(format!("Downloading '{}'", disp_url.dimmed()));

    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64 / 1024), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    Ok(())
}
