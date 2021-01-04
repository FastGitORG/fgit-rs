use crate::config::{read_remote_url, write_remote_url, GitSrc, RemoteUrl};
use crate::download::{download, replace_url};
use std::error::Error;
use std::process::Command;
use std::{env, str};

pub fn dl(url: &str) {
    let url = replace_url(url).unwrap();
    println!("url: {}", url);

    download(&url).unwrap();
}

pub fn clone(url: &str, dir: &str) {
    let current_path = env::current_dir().unwrap();
    let current_path = current_path.as_path();

    let mut ori = RemoteUrl::new(url).unwrap();
    ori.base = GitSrc::FastGit;

    let stat = git_clone(ori.to_url().as_str()).unwrap();

    if stat {
        ori.base = GitSrc::GitHub;
        write_remote_url(current_path.join(dir).as_ref(), ori.to_url().as_str());
    }
}

pub fn pull() {
    let current_path = env::current_dir().unwrap();
    let current_path = current_path.as_path();

    let mut ori = RemoteUrl::new(read_remote_url(current_path).as_str()).unwrap();

    ori.base = GitSrc::FastGit;
    write_remote_url(current_path, ori.to_url().as_str());

    let stat = git_pull().unwrap();

    if stat {
        ori.base = GitSrc::GitHub;
        write_remote_url(current_path, ori.to_url().as_str());
    }
}

fn git_clone(url: &str) -> Result<bool, Box<dyn Error>> {
    let child = Command::new("git").arg("clone").arg(url).spawn().unwrap();

    let output = child.wait_with_output().unwrap();

    Ok(output.status.success())
}

fn git_pull() -> Result<bool, Box<dyn Error>> {
    let child = Command::new("git").arg("pull").spawn().unwrap();

    let output = child.wait_with_output().unwrap();

    Ok(output.status.success())
}
