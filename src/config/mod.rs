use std::io::BufRead;

#[derive(Debug)]
pub enum ClipboardManager {
    None,
    WlCopy,
    XClip,
}

#[derive(Debug)]
pub struct Config {
    pub notes_dir: String,
    pub editor_cmd: String,
    pub clipboard: ClipboardManager,
}

impl Config {
    pub fn new() -> Config {
        return Config {
            notes_dir: String::new(),
            editor_cmd: String::new(),
            clipboard: ClipboardManager::None,
        };
    }
}

static CONFIG_FILEPATH: &'static str = "/etc/ternotes/config.txt";

pub fn get_config_filepath() -> String {
    return String::from(CONFIG_FILEPATH);
}

pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(CONFIG_FILEPATH)?;

    let mut conf = Config::new();

    let reader = std::io::BufReader::new(f);

    for line in reader.lines() {
        if line.is_err() {
            return Err(line.unwrap_err())?;
        }

        let l = line.unwrap();

        let chunks: Vec<&str> = l.split(": ").collect();

        if chunks.iter().count() != 2 {
            continue;
        }

        let key = chunks[0];
        let val = chunks[1];

        // Comments start with #
        if key.find("#").is_some() {
            continue;
        }

        match key {
            "editor_cmd" => conf.editor_cmd = String::from(val),
            "notes_dir" => conf.notes_dir = String::from(val),
            "clipboard" => {
                match val {
                    "wlcopy" => conf.clipboard = ClipboardManager::WlCopy,
                    "xclip" => conf.clipboard = ClipboardManager::XClip,
                    _ => {},
                }
            },
            _ => continue,
        }
    }

    return Ok(conf);
}
