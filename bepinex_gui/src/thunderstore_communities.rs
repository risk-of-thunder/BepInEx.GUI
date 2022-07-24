use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sysinfo::{Pid, ProcessExt, SystemExt};

use crate::egui_utils;

pub const URL: &str = "https://thunderstore.io/api/experimental/community/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Communities {
    pub pagination: Option<Pagination>,
    pub results: Option<Vec<Result>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub next_link: Option<serde_json::Value>,
    pub previous_link: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub discord_url: Option<String>,
    pub wiki_url: Option<String>,
    pub require_package_listing_approval: Option<bool>,
}

fn find_modding_discord_from_target_process_name(
    target_process_id: Pid,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let json = reqwest::blocking::get(URL).and_then(|resp| resp.text())?;
    let communities = serde_json::from_str::<Communities>(&json)
        .and_then(|c| Ok(c.results))?
        .ok_or("no communities.results")?;
    let sys = sysinfo::System::new_all();
    let proc = sys
        .process(target_process_id)
        .ok_or("no proc matching pid")?;
    let proc_name_osstring = Path::new(&proc.name().to_lowercase())
        .file_stem()
        .and_then(|s| Some(s.to_os_string()))
        .ok_or("failed getting proc name from proc")?
        .into_string();
    if proc_name_osstring.is_err() {
        return Err("Could not convert OsString to String".into());
    }
    let proc_name = proc_name_osstring.unwrap();
    for community in communities {
        let community_name_lower = community
            .name
            .and_then(|n| Some(n.to_lowercase().to_string()))
            .ok_or("failed lowercasing")?;
        if proc_name.contains(&community_name_lower) || community_name_lower.contains(&proc_name) {
            match community.discord_url {
                Some(discord_url) => return Ok(discord_url),
                None => return Err("no discord url".into()),
            }
        }
    }

    Err(format!("No community matching target process name {}", proc_name).into())
}

pub fn open_modding_discord(target_process_id: Pid) {
    match find_modding_discord_from_target_process_name(target_process_id) {
        Ok(discord_name) => {
            egui_utils::open_folder(&PathBuf::from(discord_name));
        }
        Err(err) => {
            tracing::error!("Failed finding discord, {}", err);
        }
    }
}
