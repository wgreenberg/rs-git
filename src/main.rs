use std::{env};
use failure::{Error, format_err, bail};

mod blob;
mod database;
mod repo;
mod workspace;

use crate::repo::Repo;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        bail!(format_err!("Need a command!"));
    }

    let root_dir = env::current_dir().expect("Couldn't get current path");
    let repo = Repo::new(&root_dir);

    match args[1].as_ref() {
        "init" => {
            repo.init()?;
            println!("Initialized empty Jit repository in {}", root_dir.to_string_lossy());
        },
        "commit" => {
            repo.commit()?;
        },
        _ => bail!(format_err!("ERR: {} is not a jit command", args[1])),
    }

    Ok(())
}
