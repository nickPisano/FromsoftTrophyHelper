//! Steam Web API client. All requests run on background threads and report
//! back to the UI through an mpsc channel.

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Duration;

use eframe::egui;

const API: &str = "https://api.steampowered.com";

/// Messages sent from worker threads back to the UI thread.
pub enum Msg {
    Resolved { result: Result<String, String> },
    Game { appid: u32, result: Result<GameData, String> },
}

/// One achievement, merged from the game schema, the global unlock
/// percentages, and (when available) the player's own progress.
#[derive(Clone, Debug)]
pub struct AchRow {
    pub api_name: String,
    pub name: String,
    pub description: String,
    pub hidden: bool,
    pub icon: String,
    pub icon_gray: String,
    /// Percentage of players who own the game that unlocked this (0-100).
    pub percent: Option<f32>,
    /// None = player data unavailable (private profile, not owned, ...).
    pub unlocked: Option<bool>,
    pub unlock_time: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct GameData {
    pub achievements: Vec<AchRow>,
    /// Set when schema/global data loaded but the player's own progress
    /// could not be fetched (e.g. private profile, game not owned).
    pub player_error: Option<String>,
}

pub enum SteamInput {
    Id(String),
    Vanity(String),
    Empty,
}

/// Accepts a SteamID64, a custom-URL (vanity) name, or a full profile URL.
pub fn parse_steam_input(raw: &str) -> SteamInput {
    let s = raw.trim();
    let s = if let Some(i) = s.find("/profiles/") {
        &s[i + "/profiles/".len()..]
    } else if let Some(i) = s.find("/id/") {
        &s[i + "/id/".len()..]
    } else {
        s
    };
    let s = s.trim_matches('/').trim();
    if s.is_empty() {
        SteamInput::Empty
    } else if s.len() == 17 && s.chars().all(|c| c.is_ascii_digit()) {
        SteamInput::Id(s.to_string())
    } else {
        SteamInput::Vanity(s.to_string())
    }
}

pub fn spawn_resolve_vanity(tx: Sender<Msg>, ctx: egui::Context, key: String, vanity: String) {
    std::thread::spawn(move || {
        let result = resolve_vanity(&key, &vanity);
        let _ = tx.send(Msg::Resolved { result });
        ctx.request_repaint();
    });
}

pub fn spawn_fetch_game(
    tx: Sender<Msg>,
    ctx: egui::Context,
    key: String,
    steamid: String,
    appid: u32,
) {
    std::thread::spawn(move || {
        let result = fetch_game(&key, &steamid, appid);
        let _ = tx.send(Msg::Game { appid, result });
        ctx.request_repaint();
    });
}

fn resolve_vanity(key: &str, vanity: &str) -> Result<String, String> {
    let url = format!("{API}/ISteamUser/ResolveVanityURL/v0001/?key={key}&vanityurl={vanity}");
    let v = get_json(&url)?;
    let r = &v["response"];
    if r["success"].as_i64() == Some(1) {
        match r["steamid"].as_str() {
            Some(id) => Ok(id.to_string()),
            None => Err("Steam returned an empty SteamID.".to_string()),
        }
    } else {
        Err(format!(
            "Could not resolve '{vanity}' to a Steam account. Try your 17-digit SteamID64 \
             instead (you can look it up at steamid.io)."
        ))
    }
}

fn fetch_game(key: &str, steamid: &str, appid: u32) -> Result<GameData, String> {
    // 1. Achievement schema: names, descriptions, icons.
    let url = format!("{API}/ISteamUserStats/GetSchemaForGame/v2/?key={key}&appid={appid}&l=english");
    let schema = get_json(&url).map_err(|e| format!("Failed to load achievement list: {e}"))?;
    let schema_achs = schema["game"]["availableGameStats"]["achievements"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    if schema_achs.is_empty() {
        return Err(
            "Steam returned no achievement data for this game. Double-check that your API key \
             is valid (Settings)."
                .to_string(),
        );
    }

    let mut rows: Vec<AchRow> = schema_achs
        .iter()
        .map(|a| AchRow {
            api_name: a["name"].as_str().unwrap_or_default().to_string(),
            name: a["displayName"].as_str().unwrap_or_default().to_string(),
            description: a["description"].as_str().unwrap_or_default().to_string(),
            hidden: a["hidden"].as_i64().unwrap_or(0) == 1,
            icon: a["icon"].as_str().unwrap_or_default().to_string(),
            icon_gray: a["icongray"].as_str().unwrap_or_default().to_string(),
            percent: None,
            unlocked: None,
            unlock_time: None,
        })
        .collect();

    // 2. Global unlock percentages (public endpoint, no key needed).
    let url = format!(
        "{API}/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?gameid={appid}&format=json"
    );
    if let Ok(global) = get_json(&url) {
        if let Some(arr) = global["achievementpercentages"]["achievements"].as_array() {
            let map: HashMap<String, f32> = arr
                .iter()
                .filter_map(|a| {
                    let name = a["name"].as_str()?.to_string();
                    let pct = a["percent"]
                        .as_f64()
                        .or_else(|| a["percent"].as_str()?.parse().ok())?;
                    Some((name, pct as f32))
                })
                .collect();
            for r in &mut rows {
                r.percent = map.get(&r.api_name).copied();
            }
        }
    }

    // 3. The player's own progress.
    let mut player_error = None;
    let url = format!(
        "{API}/ISteamUserStats/GetPlayerAchievements/v0001/?appid={appid}&key={key}&steamid={steamid}"
    );
    match get_json(&url) {
        Ok(v) => {
            let ps = &v["playerstats"];
            if ps["success"].as_bool().unwrap_or(false) {
                let map: HashMap<String, (bool, i64)> = ps["achievements"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|a| {
                                Some((
                                    a["apiname"].as_str()?.to_string(),
                                    (
                                        a["achieved"].as_i64()? == 1,
                                        a["unlocktime"].as_i64().unwrap_or(0),
                                    ),
                                ))
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                for r in &mut rows {
                    if let Some((achieved, t)) = map.get(&r.api_name) {
                        r.unlocked = Some(*achieved);
                        r.unlock_time = (*achieved && *t > 0).then_some(*t);
                    } else {
                        r.unlocked = Some(false);
                    }
                }
            } else {
                player_error = Some(friendly_player_error(
                    ps["error"].as_str().unwrap_or("unknown error"),
                ));
            }
        }
        Err(e) => player_error = Some(friendly_player_error(&e)),
    }

    Ok(GameData {
        achievements: rows,
        player_error,
    })
}

fn friendly_player_error(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("not public") || lower.contains("private") {
        "Your Steam profile's \"Game details\" are set to private, so your personal progress \
         can't be read. Set Profile > Privacy Settings > Game details to Public, then Refresh."
            .to_string()
    } else if lower.contains("no stats") || lower.contains("requested app") {
        "Steam has no play data for this game on your account (you may not own it on this \
         account). Showing the full achievement list with global stats only."
            .to_string()
    } else {
        format!("Couldn't load your personal progress: {raw}")
    }
}

fn get_json(url: &str) -> Result<serde_json::Value, String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(20))
        .build();
    match agent.get(url).call() {
        Ok(resp) => resp
            .into_json()
            .map_err(|e| format!("Bad response from Steam: {e}")),
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            let detail = serde_json::from_str::<serde_json::Value>(&body)
                .ok()
                .and_then(|v| {
                    v["playerstats"]["error"]
                        .as_str()
                        .map(|s| s.to_string())
                })
                .unwrap_or_default();
            match code {
                401 | 403 => Err(
                    "Steam rejected the request (HTTP 403). Your API key is missing or invalid."
                        .to_string(),
                ),
                _ if !detail.is_empty() => Err(detail),
                _ => Err(format!("Steam returned HTTP {code}.")),
            }
        }
        Err(e) => Err(format!("Network error: {e}")),
    }
}
