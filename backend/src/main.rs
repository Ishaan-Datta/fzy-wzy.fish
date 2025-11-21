use std::env;
use toml::Table;
use std::fs;
use std::result::Result::Ok;
use backend::options::*;
use backend::preview::*;

// should add logging to a file...
// should read the config file path from the env vars
static CONFIG_FILE:  &str = "/home/ishaan/fifc/backend/settings.toml";

fn main() {
    let config = match parse_config(CONFIG_FILE) {
        Ok(config_val) => config_val,
        Err(e) => {
            println!("{e}");
            // log
            std::process::exit(1);
        }
    };

    let mut args = env::args().skip(1);
    println!("{args:#?}");
    let function = args.next().expect("Missing function arg");

    match function.as_str() {
        "GetMenuOptions" => {
            println!("GetMenuOptions");
            let commandline = args.next().expect("Missing commandline arg");
            get_selection_menu(&config, &commandline);
        }

        "PreviewOption" => {
            println!("PreviewOptions");
            let match_rule = args.next().expect("Missing match_rule arg");
            let selection = args.next().expect("Missing selection arg");
            get_option_preview(&config, &selection, &match_rule);
        }

        _ => {
            // log
            std::process::exit(1);
        }
    }
}

fn parse_config(file_path: &str) -> Result<Config, anyhow::Error> {
    let config_text = fs::read_to_string(file_path)?;
    let config = config_text.parse::<Table>()?;
    dbg!(&config);

    // verify that every rule set is complete

    Ok(config)
}