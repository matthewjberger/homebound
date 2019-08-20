use std::{
    fs::File,
    io::Write,
    collections::HashMap,
    path::PathBuf,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use structopt::StructOpt;
use serde::{Serialize, Deserialize};


#[cfg(target_family = "unix")]
use std::os::unix::fs;

#[cfg(target_family = "windows")]
use std::os::windows::fs::{symlink_file, symlink_dir};
#[cfg(target_family = "windows")]
use std::fs;
#[cfg(target_family = "windows")]
use std::fs::metadata;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    links: HashMap<String, String>
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Homebound", about = "A cross-platform symlink manager")]
struct Opt {
    /// The config file to use
    #[structopt(
        short = "c",
        long = "config",
        default_value = "config.yaml",
        parse(from_os_str)
    )]
    config: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let root_folder = opt.config.as_path().parent().unwrap().to_str().unwrap();

    // Read config file to string
    let file = fs::read_to_string("../arch.yaml").expect("could not read config!");
    //println!("{}", file);

    // Serialize config to yaml
    //let s = serde_yaml::to_string(&config).unwrap();
    //println!("{}", s);

    // Deserialize config from yaml
    let config: Config = serde_yaml::from_str(&file).unwrap();
    println!("{:?}", config.links["dunstrc"]);

    // Deserialize config from toml

    // Serialize config to toml
    let config_toml = toml::to_string(&config).unwrap();
    //println!("{:?}", config_toml);

    // Creating a file
    //let mut file = File::create("arch.toml").unwrap();
    //file.write_all(config_toml.as_bytes()).unwrap();

    create_symlinks(&config, root_folder).unwrap();
}

#[cfg(target_family = "unix")]
fn create_symlinks(config: &Config, root_folder: &str) -> std::io::Result<()> {
    //let mut stdout = StandardStream::stdout(ColorChoice::Always);
    //stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    //writeln!(&mut stdout, "green text!")?;
    // fs::symlink("a", "b")?
    Ok(())
}

#[cfg(target_family = "windows")]
fn create_symlinks(config: &Config, root_folder: &str) -> std::io::Result<()> {
    // Check if path is file or folder
    //let md = metadata("path").unwrap();
    //md.is_file()

    // fs::symlink_dir("a", "b")?;
    // fs::symlink_file("a", "b")?;

    for (src, dst) in &config.links {
        //let src_metadata = metadata
        println!("{} --> {}", src, dst);
    }

    Ok(())
}

