use structopt::StructOpt;

use crate::config::*;

mod config;

#[derive(Debug, StructOpt)]
#[structopt(name = "Homebound", about = "A cross-platform symlink manager")]
struct Opt {
    /// The config file to use.
    #[structopt(short = "c", long = "config")]
    config: String,

    /// Whether to do a dry run or not. A dry run will not modify system files or create symlinks.
    #[structopt(short = "d", long = "dry")]
    dry: bool,
}

fn main() {
    let opt = Opt::from_args();
    let config = Config::new(&opt.config).unwrap();
    config.apply(opt.dry).unwrap();
}
