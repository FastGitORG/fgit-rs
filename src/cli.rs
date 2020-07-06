use crate::command::{clone, dl, pull};
use clap::{App, Arg};
use regex::Regex;

// cli is a func to handle command-line
pub fn cli() {
    let matches = App::new("fgit")
        .version("0.1.0")
        .author("batkiz")
        .about("fastgit cli")
        .subcommand(
            App::new("pull")
                .about("Fetch from and integrate with another repository or a local branch"),
        )
        .subcommand(
            App::new("dl").about("Download GitHub things").arg(
                Arg::new("url")
                    .about("the file url")
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("clone")
                .about("Clone a repository into a new directory")
                .arg(
                    Arg::new("url")
                        .about("the file url")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("dir")
                        .about("The name of a new directory to clone into.")
                        .index(2)
                        .required(false),
                ),
        )
        .get_matches();

    if matches.is_present("pull") {
        pull()
    }

    if let Some(ref matches) = matches.subcommand_matches("dl") {
        dl(matches.value_of("url").unwrap());
    }

    if let Some(ref matches) = matches.subcommand_matches("clone") {
        let url = matches.value_of("url").unwrap();
        let dir = if matches.is_present("dir") {
            matches.value_of("dir").unwrap()
        } else {
            let re = Regex::new(r".*/(.*?)($|.git)").unwrap();
            re.captures(url).unwrap().get(1).unwrap().as_str()
        };

        clone(url, dir);
    }
}
