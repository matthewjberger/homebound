use structopt::StructOpt;

use crate::config::*;

mod config;

#[derive(Debug, StructOpt)]
#[structopt(name = "Homebound", about = "A cross-platform symlink manager")]
enum Homebound {
    #[structopt(name = "install")]
    Install {
        /// The config file to use.
        #[structopt(short = "c", long = "config", default_value = "config.yaml")]
        config: String,

        /// Whether to do a dry run or not. A dry run will not modify system files or create symlinks.
        #[structopt(short = "d", long = "dry")]
        dry: bool,
    },
}

fn main() {
    match Homebound::from_args() {
        Homebound::Install { config, dry } => {
            let config_data = Config::new(&config).expect("Could not create configuration");
            config_data
                .apply(dry)
                .expect("Error applying configuration");
        }
    }
}
