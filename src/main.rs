mod light_control;

use clap::Parser;
use light_control::LightControl;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: String,

    #[arg(short, long)]
    config: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    match read_config(args.config) {
        Ok(light_control) => light_control.execute_command(args.command),
        Err(msg) => println!("Something went wrong: {}", msg),
    }
}

fn read_config(config: Option<std::path::PathBuf>) -> Result<LightControl, String> {
    if let Some(path) = config {
        return parse_file(&path);
    }

    let home_dir = home::home_dir();

    if let None = home_dir {
        return Err("Couldn't determine home directory!".to_string());
    }

    let mut path = home_dir.unwrap();

    path.push("light_control_config.json");
    parse_file(&path)
}

fn parse_file(path: &std::path::PathBuf) -> Result<LightControl, String> {
    let file_result = fs::read_to_string(path);

    if let Err(msg) = file_result {
        return Err(format!("Couldn't open file {0} : {1}", path.display(), msg));
    }

    let result = serde_json::from_str(&file_result.unwrap());

    if let Err(error) = result {
        return Err(format!("Couldn't parse JSON: {}", error));
    }

    Ok(result.unwrap())
}
