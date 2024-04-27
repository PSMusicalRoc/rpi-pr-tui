use lazy_static::lazy_static;
use crate::pr_file_format::PRSeason;
use std::{path::{Path, PathBuf}, sync::Mutex};

lazy_static! {
    pub static ref PR_SEASON: Mutex<PRSeason> = Mutex::new(PRSeason::new());
    pub static ref CURR_DIR: Mutex<PathBuf> = Mutex::new(Path::new("./").to_path_buf());
}