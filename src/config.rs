use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{collections::HashMap, io::Write, path::Path};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink;

#[cfg(target_family = "windows")]
use std::fs::metadata;
#[cfg(target_family = "windows")]
use std::os::windows::fs::{symlink_dir, symlink_file};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not open file! {} : {}", filename.display(), source))]
    OpenConfig {
        filename: PathBuf,
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(target_family = "unix")]
fn create_symlink(src: &str, dst: &str) -> std::io::Result<()> {
    symlink(src, dst)?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn create_symlink(src: &str, dst: &str) -> std::io::Result<()> {
    let src_metadata = metadata(src)?;
    if src_metadata.is_dir() {
        symlink_dir(src, dst)?;
    } else if src_metadata.is_file() {
        symlink_file(src, dst)?;
    }
    Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    links: HashMap<String, String>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    path: String,
}

impl Config {
    pub fn new(path_str: &str) -> Result<Config> {
        let path = Path::new(path_str);
        if !path.exists() {
            //println!("Config does not exist at '{:?}'!", config_path_str);
            // TODO: Return an error
        }

        let pathb = PathBuf::from(&path_str);
        let file = read_to_string(path).context(OpenConfig { filename: pathb })?;

        let extension = path.extension().unwrap();
        let config: Option<Config> = if extension == "yaml" {
            Some(serde_yaml::from_str(&file).unwrap())
        } else if extension == "toml" {
            Some(toml::from_str(&file).unwrap())
        } else {
            None
        };

        let mut config = config.unwrap();
        config.path = path.to_str().unwrap().to_string();
        Ok(config)
    }

    pub fn apply(&self, dry_run: bool) -> std::io::Result<()> {
        self.apply_links(dry_run)?;
        Ok(())
    }

    fn apply_links(&self, dry_run: bool) -> std::io::Result<()> {
        let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;

        writeln!(&mut stdout, "Establishing symlinks:")?;
        writeln!(&mut stdout, "======================")?;

        for (src, dst) in &self.links {
            let src_path = Path::new(&self.path).join(src);
            let src_path_display = src_path.to_str().unwrap();
            if src_path.exists() {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                if !dry_run {
                    create_symlink(src_path_display, dst)?;
                }
                writeln!(&mut stdout, "{} --> {}", src_path_display, dst)?;
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                writeln!(&mut stdout, "'{}' does not exist. Skipping...", src)?;
            }
        }
        Ok(())
    }

    // #[allow(dead_code)]
    // pub fn as_yaml(&self) -> Result<String, serde_yaml::Error> {
    //     serde_yaml::to_string(&self)
    // }

    // #[allow(dead_code)]
    // pub fn as_toml(&self) -> Result<String, toml::ser::Error> {
    //     toml::to_string(&self)
    // }
}
