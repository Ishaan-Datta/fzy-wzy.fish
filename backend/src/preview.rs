
use std::process::{Command, Stdio};
use std::ffi::OsStr;
use std::fs::metadata;
use std::path::Path;
use mime_guess::*;
use toml::*;

pub type Config = map::Map<String, Value>;

#[derive(PartialEq)]
enum PathType {
    Directory,
    ImageGIF,
    PDF,
    Process,
    Markdown,
    Archive,
    Video,
    File,
    JSON,
    Text,
}

#[derive(Debug)]
pub struct CommandStruct {
    command: String,
    flags: Vec<String>
}

impl CommandStruct {
    fn new(cmd: &str, flags: Vec<&str>) -> Self {
        Self {
            command: cmd.to_string(),
            flags: flags.iter().map(|e| e.to_string()).collect()
        }
    }
}

impl From<PathType> for CommandStruct {
    fn from(pathtype: PathType) -> Self {
        match pathtype {
            PathType::Markdown => CommandStruct::new("glow", vec!["-s", "light"]), // dark
            PathType::Directory => CommandStruct::new("eza", vec![""]), 
            PathType::Video => CommandStruct::new("timg", vec![""]), // can also do images
            PathType::Archive => CommandStruct::new("p7zip", vec![""]),
            PathType::ImageGIF | PathType::PDF => CommandStruct::new("chafa", vec![""]),
            PathType::File => CommandStruct::new("file", vec!["-mime-type", "-b", "--"]), // `file -mime-type -b -- "path"`
            PathType::JSON => CommandStruct::new("bat", vec!["--color=always", "-l", "--json"]),
            PathType::Process => CommandStruct::new("procs", vec![""]),
            PathType::Text => CommandStruct::new("bat", vec!["--color=always"])
        }
    }
}

// should be able to parse output from fd detailed: `drwx------    2 ishaan users  4.0K Jul 31 15:12 ./.ansible/galaxy_cache`
// do I need to do this on this side? or on fish side?
pub fn get_option_preview(config: &Config, option: &str, rule: &str) {
    if let Some(value) = config.get(rule) {
        if let Some(preview_cmd) = value["preview_cmd"].as_str() { // also need to parse the flags - make that a separate config...
            // preview_option(preview_cmd, option);
        } else {
            // log that you didnt find the preview cmd, defaulting to regular
        }
    } else {
        let preview_cmd = determine_file_type(option).into();
        dbg!(&preview_cmd);
        preview_option(preview_cmd, option);
    }
}

fn preview_option(preview_cmd: CommandStruct, option: &str) {
    let mut flags = preview_cmd.flags.clone();
    flags.push(option.to_string());

    Command::new(preview_cmd.command)
        .args(flags)
        .stdout(Stdio::inherit())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
}

fn determine_file_type(file_path: &str) -> PathType {
    match metadata(file_path) {
        Ok(val) => {
            if val.is_dir() {
                return PathType::Directory;
            }
        }
        Err(e) => {
            // log
        }
    }

    if let Some(extension) = Path::new(file_path).extension().and_then(OsStr::to_str) {
        dbg!(extension.to_string());
        match extension.to_string().as_str() {
            "md" => {
                return PathType::Markdown
            },
            "zip" | "gzip" | "tar" | "7z" | "xz" | "rar" => {
                return PathType::Archive
            },
            _ => ()
        }
    }
    
    match mime_guess::from_path(file_path).first() {
        Some(val) => {
            match_mime_type(val)
        }
        None => PathType::File
    }
}

fn match_mime_type(mime_type: Mime) -> PathType {
    match mime_type.type_() {
        mime::JSON => {
            PathType::JSON
        },
        mime::GIF => {
            PathType::ImageGIF
        },
        mime::IMAGE => {
            PathType::ImageGIF
        },
        mime::VIDEO => {
            PathType::Video
        },
        mime::PDF => {
            PathType::PDF
        }
        mime::TEXT => {
            PathType::Text
        },
        _ => {
            dbg!(mime_type);
            PathType::File
        }
    }
}