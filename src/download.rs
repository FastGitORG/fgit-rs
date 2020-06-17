use anyhow::anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::{header, Client};
use std::error::Error;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

//
#[tokio::main]
pub async fn download(url: &str) -> Result<(), anyhow::Error> {
    let url = reqwest::Url::parse(url).unwrap();
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ));
        }
    };

    let request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg:22!} {percent}% [{bar:40.cyan/blue}] {bytes}/{total_bytes}")
            .progress_chars("=>-"),
    );

    let file = Path::new(
        url.path_segments()
            .and_then(std::iter::Iterator::last)
            .unwrap_or("tmp.bin"),
    );

    println!("Downloading: {}", file.display());

    pb.set_message(file.to_str().unwrap());

    if file.exists() {
        let size = file.metadata()?.len().saturating_sub(1);
        // request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    } else {
        let mut source = request.send().await?;
        let mut dest = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file)
            .await?;
        while let Some(chunk) = source.chunk().await? {
            dest.write_all(&chunk).await?;
            pb.inc(chunk.len() as u64);
        }
    }

    pb.finish();
    println!("Download completed.");

    Ok(())
}

pub fn replace_url(url: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"https://(.*?)/").unwrap();

    let base_url = re.captures(url).unwrap().get(1).unwrap().as_str();

    let url = match base_url {
        "codeload.github.com" => url.replace("codeload.github.com", "download.fastgit.org"),
        "github.com" => url.replace("github.com", "download.fastgit.org"),
        "raw.githubusercontent.com" => url.replace("raw.githubusercontent.com", "raw.fastgit.org"),
        _ => return Err("not a supported url".into()),
    };
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace() {
        let release = "https://github.com/getzola/zola";
        let fastgit = "https://download.fastgit.org/getzola/zola";

        assert_eq!(replace_url(release).unwrap(), fastgit);
    }

    #[test]
    fn replace_codeload() {
        let codeload = "https://codeload.github.com/zeit/next.js/tar.gz/canary";
        let fastgit = "https://download.fastgit.org/zeit/next.js/tar.gz/canary";

        assert_eq!(replace_url(codeload).unwrap(), fastgit);
    }

    #[test]
    fn replace_raw() {
        let raw = "https://raw.githubusercontent.com/getzola/zola/master/build.rs";
        let fastgit = "https://raw.fastgit.org/getzola/zola/master/build.rs";

        assert_eq!(replace_url(raw).unwrap(), fastgit);
    }
}
