//! Download files from internet

use anyhow::{Error, Result};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{cmp::min, fs::File, io::Write, path::Path};

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

    let total_size = res.content_length().unwrap_or(666) / 1024;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:20}] {percent}% {elapsed}/{eta} {bytes_per_sec}")?
            .progress_chars("#> "),
    );
    pb.set_message(format!("Downloading '{}'", &path.dimmed()));

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
