use std::path::PathBuf;

use std::io;

use tangle::{absolute_lize, dir_tangle, dir_tangle_rec, file_tangle};

use clap as cmd;

fn main() -> io::Result<()> {
    let matches = cmd::App::new("org-tangle")
        .setting(cmd::AppSettings::ArgRequiredElseHelp)
        .author(cmd::crate_authors!())
        .version(cmd::crate_version!())
        .arg(
            cmd::Arg::with_name("-r")
                .short("-r")
                .long("--recursive")
                .help("recursively traverse <DIR>")
                .multiple(true),
        )
        .arg(
            cmd::Arg::with_name("PATH")
                .help(
                    "\
                     <PATH> can be <FILE> or <DIR>\n\
                     ignore non unicode <PATH>\n\
                     ignore non `.org` files\n\
                     ignore `.org` files without tangle property\n\
                     ",
                )
                .multiple(true),
        )
        .get_matches();
    if let Some(paths) = matches.values_of("PATH") {
        for path_str in paths {
            let mut path = PathBuf::new();
            path.push(path_str);
            let path = absolute_lize(&path);
            if path.is_file() {
                file_tangle(&path)?;
            } else if path.is_dir() {
                if matches.is_present("-r") {
                    dir_tangle_rec(&path)?;
                } else {
                    dir_tangle(&path)?;
                }
            }
        }
    }
    Ok(())
}
