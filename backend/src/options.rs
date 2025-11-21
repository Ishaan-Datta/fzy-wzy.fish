use crate::preview::{Config, CommandStruct};
use anyhow::*;
use regex::Regex;
use std::fmt;
use std::process::{Command, Stdio};

enum ConfigField {
    PreviewCMD,
    SourceCMD,
    DescriptionCMD
}

impl fmt::Display for ConfigField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConfigField::PreviewCMD => "preview_cmd",
            ConfigField::SourceCMD => "src_cmd",
            ConfigField::DescriptionCMD => "desc_cmd",
        };
        write!(f, "{}", s)
    }
}

//  printf '%s\n' $choices
// what does fzf require for completion menu? -> how should descriptions be passed? joined with --  ?
pub fn get_selection_menu(config: &Config, commandline: &str) {
    // match match_regex(config, commandline, ConfigField::SourceCMD) {
    //     Some(cmd) => execute_command(&cmd, ""),
    //     None => {
    //         let _ = retrieve_fish_completions(config);
    //     }
    // }


    // then evaluate the fish comp list, if match, make and return
}

// need to parse the output of the fish completions into the \n separated
fn retrieve_fish_completions(config: &Config) -> Result<String, anyhow::Error> { // will the fish completion require the commandline or also tokens?
    // should this also parse the config? -> no you can tell rust to exec the fish script for completion if you need
    // `complete -C $complete_opts -- "$fifc_commandline" | string split '\n' >$_fifc_complist_path`
    // `set -x fifc_desc (sed -nr (printf 's/^%s[[:blank:]]+(.*)/\\\1/p' "$regex_val") $_fifc_complist_path | string trim)`
    Ok(String::from(""))
}

fn match_regex(config: &Config, commandline: &str, field: ConfigField) -> Result<Option<String>, anyhow::Error> {
    for value in config.values() {
        let regex = value["regex"].as_str().ok_or(anyhow!("Regex field does not exist"))?;
        dbg!(regex);

        let re = Regex::new(regex).map_err(|e| anyhow!("Regex pattern is invalid: {}", e))?;
        let caps = re.captures(commandline);
        dbg!(&caps);

        if caps.is_some() {
            let cmd = value[&field.to_string()].as_str().ok_or(anyhow!("CMD field does not exist"))?;
            return Ok(Some(String::from(cmd)));
        }
    }
    Ok(None)
}

fn execute_command(cmd: &str, option: &str) {
    // let command_status = Command::new("which")
    //                  .arg(cmd)
    //                  .output().status;
    
    // // can just append the option to the cmd string?

    // if command_status.success() {
    //     // run command and return output as stdout -> take option as the input?
    // } else {

    //     // try backup map check
    // }

    // if the early command was false, try using the backup map first

    // If no viewer is available for the previewed file, the output of file -b (file type description) 
    // and file --brief --mime-type (MIME) will be displayed instead
}

// should I be returning string or just send to stdout?
fn retrieve_options(cmd: &str) {

}

// should make default if you cant find anything for that command to use fd with options

// mapping function:

// path: `fd --hidden --follow --exclude ".git" . "$1"`
// directory completion: `fd --type d --hidden --follow --exclude ".git" . "$1"`
