use lazy_static::lazy_static;
use crate::pr_file_format::PRSeason;
use std::sync::Mutex;

lazy_static! {
    pub static ref PR_SEASON: Mutex<PRSeason> = Mutex::new(PRSeason::new());
}