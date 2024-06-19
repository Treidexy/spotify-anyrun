use std::{fs, os::unix::process::CommandExt, process::Command};

use abi_stable::std_types::{ROption::{RNone, RSome}, RString, RVec};
use anyrun_plugin::*;
use rspotify::{clients::BaseClient, model::{Id, SearchResult}, ClientCredsSpotify, Credentials};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    max_entries: u32,
    offset: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_entries: 4,
            offset: 0,
        }
    }
}

struct State {
    config: Config,
    spotify: ClientCredsSpotify,
}

#[init]
fn init(config_dir: RString) -> State {
    let spotify = ClientCredsSpotify::new(Credentials::from_env().expect("failed to get spotify credentials from env"));
    spotify.request_token().expect("failed to request spotify token");

    State {
        config: match fs::read_to_string(format!("{}/spotify.ron", config_dir)) {
            Ok(content) => ron::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        },
        spotify,
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Spotify".into(),
        icon: "spotify".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    if input.is_empty() {
        return RVec::new()
    }

    let result = state.spotify.search(
        input.as_str(),
        rspotify::model::SearchType::Track,
        None,
        None,
        Some(state.config.max_entries),
        Some(state.config.offset),
    );

    if let SearchResult::Tracks(tracks) = result.expect("spotify search error!") {
        tracks.items.iter().map(|item| Match {
            title: RString::from(format!("{} | {}", item.name, item.artists.iter().fold("".to_string(), |p, artist| p + &artist.name + ", "))),
            description: item.id.as_ref().map(|id| RString::from(id.id())).into(),
            use_pango: false,
            icon: RSome("spotify".into()),
            id: RNone,
        }).collect()
    } else {
        println!("spotify for sum reason don't give us tracks");
        RVec::new()
    }
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    if let RSome(id) = selection.description {
        Command::new("playerctl").arg("--player=spotify").arg("open").arg(format!("spotify:track:{}", id)).exec();
    }
    HandleResult::Close
}
