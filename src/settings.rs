use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn config_dir() -> PathBuf {
    // Use XDG_CONFIG_HOME if set; otherwise default to $HOME/.config
    let base: PathBuf = env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| env::var("HOME").ok().map(|h| Path::new(&h).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));

    base.join("number2text")
}

fn selected_index_path() -> PathBuf {
    config_dir().join("selected_language.txt")
}

pub fn load_selected_index() -> Option<usize> {
    let path = selected_index_path();
    match fs::read_to_string(path) {
        Ok(contents) => contents.trim().parse::<usize>().ok(),
        Err(_) => None,
    }
}

pub fn save_selected_index(index: usize) -> io::Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let path = selected_index_path();
    let mut file = fs::File::create(path)?;
    write!(file, "{}", index)?;
    Ok(())
}
