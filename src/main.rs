use structopt::StructOpt;

use crate::config::*;

mod config;

#[derive(Debug, StructOpt)]
#[structopt(name = "Homebound", about = "A cross-platform symlink manager")]
struct Opt {
    /// The config file to use.
    #[structopt(short = "c", long = "config")]
    config: Option<String>,

    /// Whether to do a dry run or not. A dry run will not modify system files or create symlinks.
    #[structopt(short = "d", long = "dry")]
    dry: Option<bool>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.config.is_some() {
        let config = Config::new(&opt.config.unwrap()).unwrap();
        config.apply(opt.dry.unwrap()).unwrap();
    } else {
        println!("No config specified.");
    }
}
