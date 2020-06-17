extern crate ini;
use ini::Ini;
use regex::Regex;
use std::error::Error;
use std::path::Path;
use std::process;

pub struct RemoteUrl {
    pub protocol: GitProtocol,
    pub base: GitSrc,
    pub repo: String,
}

impl RemoteUrl {
    pub fn to_url(&self) -> String {
        match self.protocol {
            GitProtocol::SSH => format!("git@{}:{}", self.base.to_ssh_base(), self.repo),
            GitProtocol::HTTPS => format!("https://{}/{}", self.base.to_https_base(), self.repo),
        }
    }

    pub fn new(url: &str) -> Result<RemoteUrl, Box<dyn Error>> {
        if url.starts_with("git@") {
            let base_re = Regex::new(r"@(.*):").unwrap();
            let base_got = base_re.captures(url).unwrap().get(1).unwrap().as_str();

            let b = match base_got {
                "github.com" => GitSrc::GitHub,
                "fastgit.org" => GitSrc::FastGit,
                _ => return Err("ops, it's remote is not github or fastgit".into()),
            };

            let repo_re = Regex::new(r":(.*)").unwrap();
            let repo_got = repo_re.captures(url).unwrap().get(1).unwrap().as_str();

            Ok(RemoteUrl {
                protocol: GitProtocol::SSH,
                base: b,
                repo: repo_got.to_string(),
            })
        } else {
            let base_re = Regex::new(r"//(.*?)/").unwrap();
            let base_got = base_re.captures(url).unwrap().get(1).unwrap().as_str();

            let b = match base_got {
                "github.com" => GitSrc::GitHub,
                "hub.fastgit.org" => GitSrc::FastGit,
                _ => return Err("ops, it's remote is not github or fastgit".into()),
            };

            let repo_re = Regex::new(r"com/(.*)").unwrap();
            let repo_got = repo_re.captures(url).unwrap().get(1).unwrap().as_str();

            Ok(RemoteUrl {
                protocol: GitProtocol::HTTPS,
                base: b,
                repo: repo_got.to_string(),
            })
        }
    }
}

pub enum GitSrc {
    GitHub,
    FastGit,
}

impl GitSrc {
    fn to_https_base(&self) -> &'static str {
        match self {
            GitSrc::FastGit => "hub.fastgit.org",
            GitSrc::GitHub => "github.com",
        }
    }

    fn to_ssh_base(&self) -> &'static str {
        match self {
            GitSrc::FastGit => "fastgit.org",
            GitSrc::GitHub => "github.com",
        }
    }
}

pub enum GitProtocol {
    HTTPS,
    SSH,
}

impl std::fmt::Display for GitSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GitSrc::GitHub => "GitHub",
            GitSrc::FastGit => "fastgit",
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for GitProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GitProtocol::HTTPS => "HTTPS",
            GitProtocol::SSH => "SSH",
        };

        write!(f, "{}", s)
    }
}

pub fn read_remote_url(path: &Path) -> String {
    let config_path = Path::new("")
        .join(path.to_owned())
        .join(".git")
        .join("config");

    let conf = Ini::load_from_file(config_path).unwrap_or_else(|_| {
        eprintln!("seems this is not a git repo");
        process::exit(1);
    });

    let url = conf
        .section(Some("remote \"origin\""))
        .unwrap()
        .get("url")
        .unwrap_or_else(|| {
            eprintln!("seems this repo not contains a remote url");
            process::exit(1);
        });

    url.into()
}

pub fn write_remote_url(path: &Path, url: &str) {
    let config_path = Path::new("")
        .join(path.to_owned())
        .join(".git")
        .join("config");
    let config_path = config_path.as_path();

    let mut conf = Ini::load_from_file(config_path).unwrap_or_else(|_err| {
        eprintln!("seems this is not a git repo");
        process::exit(1);
    });

    conf.with_section(Some("remote \"origin\"")).set("url", url);

    conf.write_to_file(config_path.to_owned()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_https() {
        let url = "https://github.com/tokiedokie/scoop-search";

        let st = RemoteUrl {
            protocol: GitProtocol::HTTPS,
            base: GitSrc::GitHub,
            repo: "tokiedokie/scoop-search".parse().unwrap(),
        };

        assert_eq!(url, st.to_url())
    }

    #[test]
    fn github_ssh() {
        let url = "git@github.com:tokiedokie/scoop-search.git";

        let st = RemoteUrl {
            protocol: GitProtocol::SSH,
            base: GitSrc::GitHub,
            repo: "tokiedokie/scoop-search.git".to_string(),
        };

        assert_eq!(url, st.to_url())
    }

    #[test]
    fn fastgit_https() {
        let url = "https://hub.fastgit.org/tokiedokie/scoop-search";

        let st = RemoteUrl {
            protocol: GitProtocol::HTTPS,
            base: GitSrc::FastGit,
            repo: "tokiedokie/scoop-search".parse().unwrap(),
        };

        assert_eq!(url, st.to_url())
    }

    #[test]
    fn fastgit_ssh() {
        let url = "git@fastgit.org:tokiedokie/scoop-search.git";

        let st = RemoteUrl {
            protocol: GitProtocol::SSH,
            base: GitSrc::FastGit,
            repo: "tokiedokie/scoop-search.git".to_string(),
        };

        assert_eq!(url, st.to_url())
    }
}
