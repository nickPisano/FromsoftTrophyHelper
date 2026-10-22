use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

use crate::areas;
use crate::games::{self, GAMES};
use crate::guides;
use crate::steam::{self, AchRow, GameData, Msg};
use crate::theme::{self, BLUE, DIM, GOLD, GREEN, ORANGE, PURPLE, RED};

const OVERVIEW_LABEL: &str = "\u{2302} Overview — all games";

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(default)]
struct Settings {
    api_key: String,
    steam_input: String,
    steam_id: String,
    auto_refresh: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            steam_input: String::new(),
            steam_id: String::new(),
            auto_refresh: true,
        }
    }
}

/// How stale a scan may get before auto-refresh re-fetches it.
const AUTO_REFRESH_AFTER_SECS: i64 = 300;

/// A completed scan plus when it happened — this is what gets cached to disk.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ScanEntry {
    data: GameData,
    fetched_at: i64,
}

enum Loaded {
    Loading,
    Ready(ScanEntry),
    Failed(String),
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Steam's storefront header art (460x215) — loaded via the http image loader.
fn header_art_url(appid: u32) -> String {
    format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{appid}/header.jpg")
}

/// "3m ago" / "2h ago" / "5d ago"
fn ago(unix: i64) -> String {
    let secs = (now_unix() - unix).max(0);
    if secs < 60 {
        "just now".to_string()
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86_400 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86_400)
    }
}

#[derive(PartialEq, Clone, Copy)]
enum StatusFilter {
    All,
    Locked,
    Unlocked,
}

#[derive(PartialEq, Clone, Copy)]
enum GameView {
    Achievements,
    Planner,
    Missables,
    Areas,
}

#[derive(PartialEq, Clone, Copy)]
enum SortBy {
    MostCommon,
    Rarest,
    Name,
    RecentUnlocks,
}

impl SortBy {
    fn label(&self) -> &'static str {
        match self {
            SortBy::MostCommon => "Most common first",
            SortBy::Rarest => "Rarest first",
            SortBy::Name => "Name (A-Z)",
            SortBy::RecentUnlocks => "Recently unlocked",
        }
    }
}

pub struct App {
    settings: Settings,
    settings_open: bool,
    connect_status: Option<Result<String, String>>,
    resolving: bool,
    detected: Option<steam::LocalAccount>,

    tx: Sender<Msg>,
    rx: Receiver<Msg>,

    data: HashMap<u32, Loaded>,
    /// `None` = the all-games overview dashboard.
    selected_game: Option<usize>,
    selected_ach: HashMap<u32, String>,
    /// Ticked sub-checklist items, keyed by "appid/api_name" → set of item
    /// texts (robust against list reordering/insertion). Persisted.
    checklist_done: HashMap<String, BTreeSet<String>>,
    /// Quick-filter text for long checklists (transient).
    checklist_filter: String,
    /// By Area view: hide items already collected.
    areas_hide_done: bool,
    /// By Area view: restrict the list to one area picked on the map.
    areas_focus: Option<usize>,
    /// By Area view: narrow further to one spot inside that area.
    areas_spot_focus: Option<usize>,
    /// Free-text personal notes, keyed by "appid/api_name". Persisted.
    notes: HashMap<String, String>,

    search: String,
    status_filter: StatusFilter,
    missable_only: bool,
    sort_by: SortBy,
    game_view: GameView,

    /// Games with a background re-scan in flight (old data stays on screen).
    refreshing: std::collections::HashSet<u32>,
    /// Games whose next completed scan should announce itself with a toast.
    announce_scan: std::collections::HashSet<u32>,
    /// (message, expiry) — transient in-app notifications.
    toasts: Vec<(String, std::time::Instant)>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        theme::apply(&cc.egui_ctx);

        let settings: Settings = cc
            .storage
            .and_then(|s| eframe::get_value(s, "settings"))
            .unwrap_or_default();
        let checklist_done: HashMap<String, BTreeSet<String>> = cc
            .storage
            .and_then(|s| eframe::get_value(s, "checklists_v2"))
            .unwrap_or_default();
        let notes: HashMap<String, String> = cc
            .storage
            .and_then(|s| eframe::get_value(s, "notes"))
            .unwrap_or_default();
        // Last scan per game — render instantly, refresh in the background.
        let cache: HashMap<u32, ScanEntry> = cc
            .storage
            .and_then(|s| eframe::get_value(s, "scan_cache"))
            .unwrap_or_default();
        let data = cache
            .into_iter()
            .map(|(appid, entry)| (appid, Loaded::Ready(entry)))
            .collect();
        let settings_open = settings.api_key.is_empty() || settings.steam_id.is_empty();

        // Prefill the account field from the local Steam client install.
        let detected = steam::detect_local_account();
        let mut settings = settings;
        if settings.steam_input.is_empty()
            && let Some(acc) = &detected
        {
            settings.steam_input = acc.steamid.clone();
        }

        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            settings,
            settings_open,
            connect_status: None,
            resolving: false,
            detected,
            tx,
            rx,
            data,
            selected_game: None,
            selected_ach: HashMap::new(),
            checklist_done,
            checklist_filter: String::new(),
            areas_hide_done: true,
            areas_focus: None,
            areas_spot_focus: None,
            notes,
            search: String::new(),
            status_filter: StatusFilter::All,
            missable_only: false,
            sort_by: SortBy::MostCommon,
            game_view: GameView::Achievements,
            refreshing: std::collections::HashSet::new(),
            announce_scan: std::collections::HashSet::new(),
            toasts: Vec::new(),
        }
    }

    fn push_toast(&mut self, msg: impl Into<String>) {
        self.toasts.push((
            msg.into(),
            std::time::Instant::now() + std::time::Duration::from_secs(5),
        ));
    }

    fn notify_unlock(&mut self, game: &str, name: &str, pct: Option<f32>) {
        let rarity = pct
            .map(|p| format!(" — {p:.1}% of players have this"))
            .unwrap_or_default();
        self.push_toast(format!("\u{1F3C6} {name} unlocked!{rarity}"));
        let _ = notify_rust::Notification::new()
            .summary(&format!("Achievement unlocked — {game}"))
            .body(&format!("{name}{rarity}"))
            .show();
    }

    /// Background re-scan of stale games in view. Cached data stays visible.
    fn auto_refresh(&mut self, ctx: &egui::Context) {
        if !self.settings.auto_refresh || !self.connected() {
            return;
        }
        let now = now_unix();
        let stale: Vec<u32> = self
            .data
            .iter()
            .filter_map(|(appid, loaded)| match loaded {
                Loaded::Ready(entry)
                    if now - entry.fetched_at > AUTO_REFRESH_AFTER_SECS
                        && !self.refreshing.contains(appid) =>
                {
                    Some(*appid)
                }
                _ => None,
            })
            .filter(|appid| match self.selected_game {
                Some(i) => GAMES[i].appid == *appid,
                None => true, // dashboard keeps everything fresh
            })
            .collect();
        for appid in stale {
            self.refreshing.insert(appid);
            steam::spawn_fetch_game(
                self.tx.clone(),
                ctx.clone(),
                self.settings.api_key.clone(),
                self.settings.steam_id.clone(),
                appid,
            );
        }
        // Keep the staleness check ticking even when idle.
        ctx.request_repaint_after(std::time::Duration::from_secs(30));
    }

    fn draw_toasts(&mut self, ctx: &egui::Context) {
        let now = std::time::Instant::now();
        self.toasts.retain(|(_, expiry)| *expiry > now);
        if self.toasts.is_empty() {
            return;
        }
        egui::Area::new(egui::Id::new("toast_area"))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-16.0, -16.0))
            .show(ctx, |ui| {
                for (msg, _) in &self.toasts {
                    egui::Frame::new()
                        .fill(theme::TOAST_BG)
                        .stroke(egui::Stroke::new(1.0, GOLD.gamma_multiply(0.7)))
                        .corner_radius(8.0)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(msg);
                        });
                }
            });
        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }

    fn connected(&self) -> bool {
        !self.settings.api_key.is_empty() && !self.settings.steam_id.is_empty()
    }

    fn current_appid(&self) -> Option<u32> {
        self.selected_game.map(|i| GAMES[i].appid)
    }

    fn poll_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Msg::Resolved { result } => {
                    self.resolving = false;
                    match result {
                        Ok(id) => {
                            self.settings.steam_id = id.clone();
                            self.connect_status = Some(Ok(format!("Connected as SteamID {id}")));
                        }
                        Err(e) => self.connect_status = Some(Err(e)),
                    }
                }
                Msg::Game { appid, result } => {
                    let was_refreshing = self.refreshing.remove(&appid);
                    let game_title: &'static str = GAMES
                        .iter()
                        .find(|g| g.appid == appid)
                        .map(|g| g.title)
                        .unwrap_or("Unknown game");
                    match result {
                        Ok(gd) => {
                            // Achievements that flipped to unlocked since the last scan.
                            let newly: Vec<(String, Option<f32>)> = match self.data.get(&appid) {
                                Some(Loaded::Ready(old))
                                    if old.data.player_error.is_none()
                                        && gd.player_error.is_none() =>
                                {
                                    let before: std::collections::HashSet<&str> = old
                                        .data
                                        .achievements
                                        .iter()
                                        .filter(|a| a.unlocked == Some(true))
                                        .map(|a| a.api_name.as_str())
                                        .collect();
                                    gd.achievements
                                        .iter()
                                        .filter(|a| {
                                            a.unlocked == Some(true)
                                                && !before.contains(a.api_name.as_str())
                                        })
                                        .map(|a| (a.name.clone(), a.percent))
                                        .collect()
                                }
                                _ => Vec::new(),
                            };
                            for (name, pct) in newly {
                                self.notify_unlock(game_title, &name, pct);
                            }
                            if self.announce_scan.remove(&appid) {
                                self.push_toast(format!("{game_title}: scan complete"));
                            }
                            self.data.insert(
                                appid,
                                Loaded::Ready(ScanEntry {
                                    data: gd,
                                    fetched_at: now_unix(),
                                }),
                            );
                        }
                        Err(e) => {
                            // A failed background refresh keeps the cached scan on screen.
                            if was_refreshing
                                && matches!(self.data.get(&appid), Some(Loaded::Ready(_)))
                            {
                                self.push_toast(format!("{game_title}: refresh failed — {e}"));
                            } else {
                                self.data.insert(appid, Loaded::Failed(e));
                            }
                        }
                    }
                }
            }
        }
    }

    fn ensure_loaded(&mut self, ctx: &egui::Context) {
        if !self.connected() {
            return;
        }
        match self.current_appid() {
            Some(appid) => self.ensure_game_loaded(ctx, appid),
            // Dashboard: everything.
            None => {
                for g in GAMES {
                    self.ensure_game_loaded(ctx, g.appid);
                }
            }
        }
    }

    fn ensure_game_loaded(&mut self, ctx: &egui::Context, appid: u32) {
        if let std::collections::hash_map::Entry::Vacant(entry) = self.data.entry(appid) {
            entry.insert(Loaded::Loading);
            steam::spawn_fetch_game(
                self.tx.clone(),
                ctx.clone(),
                self.settings.api_key.clone(),
                self.settings.steam_id.clone(),
                appid,
            );
        }
    }

    fn connect(&mut self, ctx: &egui::Context) {
        self.connect_status = None;
        if self.settings.api_key.trim().is_empty() {
            self.connect_status = Some(Err("Enter your Steam Web API key first.".into()));
            return;
        }
        self.settings.api_key = self.settings.api_key.trim().to_string();
        match steam::parse_steam_input(&self.settings.steam_input) {
            steam::SteamInput::Empty => {
                self.connect_status = Some(Err(
                    "Enter your SteamID64, custom URL name, or profile URL.".into(),
                ));
            }
            steam::SteamInput::Id(id) => {
                self.settings.steam_id = id.clone();
                self.connect_status = Some(Ok(format!("Connected as SteamID {id}")));
                self.data.clear();
            }
            steam::SteamInput::Vanity(v) => {
                self.resolving = true;
                self.settings.steam_id.clear();
                self.data.clear();
                steam::spawn_resolve_vanity(
                    self.tx.clone(),
                    ctx.clone(),
                    self.settings.api_key.clone(),
                    v,
                );
            }
        }
    }

    fn refresh(&mut self, ctx: &egui::Context) {
        match self.current_appid() {
            Some(appid) => {
                self.data.remove(&appid);
                self.announce_scan.insert(appid);
            }
            None => {
                self.data.clear();
                self.push_toast("Re-scanning all games…");
            }
        }
        self.ensure_loaded(ctx);
    }

    // ------------------------------------------------------------------ UI

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("top_bar").show(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(theme::display("\u{2694} FromSoft Trophy Helper", 20.0));
                ui.separator();

                let prev = self.selected_game;
                egui::ComboBox::from_id_salt("game_select")
                    .selected_text(match self.selected_game {
                        None => OVERVIEW_LABEL,
                        Some(i) => GAMES[i].title,
                    })
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_game, None, OVERVIEW_LABEL);
                        ui.separator();
                        for (i, g) in GAMES.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_game, Some(i), g.title);
                        }
                    });
                if prev != self.selected_game {
                    self.search.clear();
                }

                if ui
                    .add_enabled(self.connected(), egui::Button::new("\u{27F3} Refresh"))
                    .on_hover_text("Re-scan your achievements from Steam")
                    .clicked()
                {
                    self.refresh(&ctx);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("\u{2699} Settings").clicked() {
                        self.settings_open = !self.settings_open;
                    }
                    if self.connected() {
                        ui.label(
                            egui::RichText::new(format!("\u{1F7E2} {}", self.settings.steam_id))
                                .color(GREEN)
                                .small(),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("\u{26AA} Not connected — open Settings")
                                .color(DIM)
                                .small(),
                        );
                    }
                });
            });
            ui.add_space(6.0);
        });
    }

    fn settings_window(&mut self, ctx: &egui::Context) {
        let mut open = self.settings_open;
        let mut do_connect = false;
        egui::Window::new("Steam connection")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(440.0)
            .show(ctx, |ui| {
                ui.label("This app reads your achievements through the official Steam Web API.");
                ui.add_space(8.0);

                ui.label(egui::RichText::new("1. Steam Web API key").strong());
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.settings.api_key)
                            .password(true)
                            .desired_width(300.0)
                            .hint_text("paste your API key"),
                    );
                    ui.hyperlink_to("Get a free key", "https://steamcommunity.com/dev/apikey");
                });
                ui.label(
                    egui::RichText::new(
                        "Any domain name works when registering (e.g. \"localhost\"). \
                         The key is stored only on this computer.",
                    )
                    .small()
                    .color(DIM),
                );
                ui.add_space(8.0);

                ui.label(egui::RichText::new("2. Your Steam account").strong());
                ui.add(
                    egui::TextEdit::singleline(&mut self.settings.steam_input)
                        .desired_width(300.0)
                        .hint_text("SteamID64, custom URL name, or full profile URL"),
                );
                if let Some(acc) = self.detected.clone() {
                    ui.horizontal(|ui| {
                        let who = if acc.persona.is_empty() {
                            acc.steamid.clone()
                        } else {
                            format!("{} ({})", acc.persona, acc.steamid)
                        };
                        ui.label(
                            egui::RichText::new(format!(
                                "\u{1F50D} Detected from your Steam client: {who}"
                            ))
                            .small()
                            .color(GREEN),
                        );
                        if self.settings.steam_input != acc.steamid
                            && ui.small_button("Use this account").clicked()
                        {
                            self.settings.steam_input = acc.steamid;
                        }
                    });
                }
                ui.label(
                    egui::RichText::new(
                        "Your profile's \"Game details\" must be public for progress to show \
                         (Steam > Profile > Edit Profile > Privacy Settings).",
                    )
                    .small()
                    .color(DIM),
                );
                ui.add_space(8.0);
                ui.checkbox(
                    &mut self.settings.auto_refresh,
                    "Auto-rescan every 5 minutes while the app is open",
                );
                ui.label(
                    egui::RichText::new(
                        "You'll get a notification when a new achievement pops mid-session.",
                    )
                    .small()
                    .color(DIM),
                );
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(!self.resolving, egui::Button::new("Connect & scan"))
                        .clicked()
                    {
                        do_connect = true;
                    }
                    if self.resolving {
                        ui.spinner();
                        ui.label("Resolving Steam account…");
                    }
                });
                match &self.connect_status {
                    Some(Ok(msg)) => {
                        ui.label(egui::RichText::new(msg).color(GREEN));
                    }
                    Some(Err(msg)) => {
                        ui.label(egui::RichText::new(msg).color(RED));
                    }
                    None => {}
                }
            });
        if do_connect {
            self.connect(ctx);
        }
        self.settings_open = open;
    }

    fn left_panel(&mut self, ui: &mut egui::Ui) {
        let Some(appid) = self.current_appid() else {
            return; // Dashboard has no achievement list panel.
        };
        egui::Panel::left("ach_list")
            .resizable(true)
            .default_size(430.0)
            .min_size(340.0)
            .show(ui, |ui| {
                ui.add_space(6.0);

                // Filters
                ui.horizontal(|ui| {
                    ui.label("\u{1F50D}");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .desired_width(f32::INFINITY)
                            .hint_text("Search achievements…"),
                    );
                });
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.status_filter, StatusFilter::All, "All");
                    ui.selectable_value(
                        &mut self.status_filter,
                        StatusFilter::Locked,
                        "\u{1F512} Locked",
                    );
                    ui.selectable_value(
                        &mut self.status_filter,
                        StatusFilter::Unlocked,
                        "\u{2714} Unlocked",
                    );
                    ui.separator();
                    ui.checkbox(&mut self.missable_only, "\u{26A0} Missable only");
                });
                ui.horizontal(|ui| {
                    ui.label("Sort:");
                    egui::ComboBox::from_id_salt("sort_by")
                        .selected_text(self.sort_by.label())
                        .show_ui(ui, |ui| {
                            for s in [
                                SortBy::MostCommon,
                                SortBy::Rarest,
                                SortBy::Name,
                                SortBy::RecentUnlocks,
                            ] {
                                ui.selectable_value(&mut self.sort_by, s, s.label());
                            }
                        });
                });
                ui.add_space(4.0);
                ui.separator();

                let mut clicked: Option<String> = None;
                let current_sel = self.selected_ach.get(&appid).cloned();

                match self.data.get(&appid) {
                    None => {
                        ui.add_space(20.0);
                        ui.vertical_centered(|ui| {
                            ui.label("Connect your Steam account to scan achievements.");
                            if ui.button("Open Settings").clicked() {
                                self.settings_open = true;
                            }
                        });
                    }
                    Some(Loaded::Loading) => {
                        ui.add_space(20.0);
                        ui.vertical_centered(|ui| {
                            ui.spinner();
                            ui.label("Scanning Steam achievements…");
                        });
                    }
                    Some(Loaded::Failed(e)) => {
                        ui.add_space(20.0);
                        ui.label(egui::RichText::new(e).color(RED));
                    }
                    Some(Loaded::Ready(entry)) => {
                        let gd = &entry.data;
                        // Progress header
                        let total = gd.achievements.len();
                        let unlocked = gd
                            .achievements
                            .iter()
                            .filter(|a| a.unlocked == Some(true))
                            .count();
                        if gd.player_error.is_none() && total > 0 {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{unlocked} / {total} unlocked ({:.0}%)",
                                        unlocked as f32 / total as f32 * 100.0
                                    ))
                                    .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "· scanned {}",
                                        ago(entry.fetched_at)
                                    ))
                                    .small()
                                    .color(DIM),
                                );
                            });
                            ui.add(
                                egui::ProgressBar::new(unlocked as f32 / total as f32)
                                    .desired_height(8.0),
                            );
                        } else if let Some(err) = &gd.player_error {
                            ui.label(
                                egui::RichText::new(format!("\u{26A0} {err}"))
                                    .color(ORANGE)
                                    .small(),
                            );
                        }
                        ui.add_space(4.0);

                        let rows = filter_sort(
                            &gd.achievements,
                            appid,
                            &self.search,
                            self.status_filter,
                            self.missable_only,
                            self.sort_by,
                        );
                        ui.label(
                            egui::RichText::new(format!("{} shown", rows.len()))
                                .small()
                                .color(DIM),
                        );

                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                for row in rows {
                                    let selected =
                                        current_sel.as_deref() == Some(row.api_name.as_str());
                                    if ach_list_row(ui, appid, row, selected, &self.checklist_done)
                                    {
                                        clicked = Some(row.api_name.clone());
                                    }
                                }
                            });
                    }
                }

                if let Some(c) = clicked {
                    self.selected_ach.insert(appid, c);
                }
            });
    }

    fn central_panel(&mut self, ui: &mut egui::Ui) {
        let Some(gi) = self.selected_game else {
            self.dashboard(ui);
            return;
        };
        let game = &GAMES[gi];
        let appid = game.appid;
        let selected: Option<AchRow> = match (self.data.get(&appid), self.selected_ach.get(&appid))
        {
            (Some(Loaded::Ready(entry)), Some(sel)) => entry
                .data
                .achievements
                .iter()
                .find(|a| &a.api_name == sel)
                .cloned(),
            _ => None,
        };

        // Planner and overview need their own copy of the rows to sidestep
        // split borrows against &mut self UI state.
        let view_rows: Option<Vec<AchRow>> = match self.data.get(&appid) {
            Some(Loaded::Ready(entry))
                if self.game_view != GameView::Achievements || selected.is_none() =>
            {
                Some(entry.data.achievements.clone())
            }
            _ => None,
        };

        let mut nav_clicked: Option<String> = None;
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.game_view,
                    GameView::Achievements,
                    "\u{1F3C6} Achievements",
                );
                ui.selectable_value(&mut self.game_view, GameView::Planner, "\u{1F5FA} Planner");
                ui.selectable_value(
                    &mut self.game_view,
                    GameView::Missables,
                    "\u{26A0} Missables",
                );
                if !areas::areas_for(appid).is_empty() {
                    ui.selectable_value(&mut self.game_view, GameView::Areas, "\u{1F4CD} By Area");
                }
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| match self.game_view {
                    GameView::Planner => match &view_rows {
                        Some(rows) => {
                            nav_clicked = planner_view(ui, game, rows, &self.checklist_done);
                        }
                        None => {
                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new(
                                    "Scan the game first — the planner organizes your \
                                     remaining achievements by playthrough.",
                                )
                                .color(DIM),
                            );
                        }
                    },
                    GameView::Areas => match &view_rows {
                        Some(rows) => {
                            nav_clicked = areas_view(
                                ui,
                                game,
                                rows,
                                &mut self.checklist_done,
                                &mut self.areas_hide_done,
                                &mut self.areas_focus,
                                &mut self.areas_spot_focus,
                            );
                        }
                        None => {
                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new(
                                    "Scan the game first — this view routes every outstanding \
                                     checklist item to the area you'll find it in.",
                                )
                                .color(DIM),
                            );
                        }
                    },
                    GameView::Missables => match &view_rows {
                        Some(rows) => {
                            nav_clicked = missables_view(ui, game, rows);
                        }
                        None => {
                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new(
                                    "Scan the game first — this view lists everything you \
                                     can permanently miss this playthrough.",
                                )
                                .color(DIM),
                            );
                        }
                    },
                    GameView::Achievements => match selected {
                        Some(row) => ach_details(
                            ui,
                            game,
                            &row,
                            &mut self.checklist_done,
                            &mut self.checklist_filter,
                            &mut self.notes,
                        ),
                        None => {
                            nav_clicked = game_overview(
                                ui,
                                game,
                                view_rows.as_deref(),
                                self.connected(),
                                &mut self.settings_open,
                            );
                        }
                    },
                });
        });

        if let Some(api_name) = nav_clicked {
            self.selected_ach.insert(appid, api_name);
            self.game_view = GameView::Achievements;
        }
    }
}

/// One outstanding checklist item, with enough context to route it to an
/// area and a spot, tick it, and jump to its achievement.
struct RouteItem<'a> {
    ach: &'a str,
    api: &'a str,
    item: &'static str,
    section: &'static str,
}

fn item_done(done: &HashMap<String, BTreeSet<String>>, appid: u32, api: &str, item: &str) -> bool {
    done.get(&format!("{appid}/{api}"))
        .is_some_and(|d| d.contains(item))
}

/// One area drawn as a board of its notable spots, each card carrying how
/// much is still waiting there. Spots with nothing in them are left out, so
/// the board stays stable as you tick things off. Returns a clicked spot.
fn area_map(
    ui: &mut egui::Ui,
    appid: u32,
    area: &areas::Area,
    items: &[RouteItem],
    checklist_done: &HashMap<String, BTreeSet<String>>,
    spot_focus: Option<usize>,
) -> Option<usize> {
    let mut total = vec![0usize; area.spots.len()];
    let mut left = vec![0usize; area.spots.len()];
    for it in items {
        if let Some(s) = areas::resolve_spot(area, it.item, it.section) {
            total[s] += 1;
            if !item_done(checklist_done, appid, it.api, it.item) {
                left[s] += 1;
            }
        }
    }
    if total.iter().all(|t| *t == 0) {
        return None;
    }

    const CARD: egui::Vec2 = egui::vec2(176.0, 34.0);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 320.0),
        egui::Sense::click(),
    );
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8.0, theme::MAP_BG);
    painter.rect_stroke(
        rect,
        8.0,
        egui::Stroke::new(1.0, theme::MAP_BORDER),
        egui::StrokeKind::Inside,
    );

    // Keep every card fully inside the panel.
    let inner = rect.shrink2(CARD / 2.0 + egui::vec2(12.0, 12.0));
    let card_at = |p: (f32, f32)| -> egui::Rect {
        egui::Rect::from_center_size(
            egui::pos2(
                inner.left() + p.0 * inner.width(),
                inner.top() + p.1 * inner.height(),
            ),
            CARD,
        )
    };

    let card_hit = |p: egui::Pos2| -> Option<usize> {
        area.spots
            .iter()
            .enumerate()
            .find(|(s, spot)| total[*s] > 0 && card_at(spot.pos).contains(p))
            .map(|(s, _)| s)
    };
    let hovered = response.hover_pos().and_then(card_hit);
    if hovered.is_some() {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }

    for (s, spot) in area.spots.iter().enumerate() {
        if total[s] == 0 {
            continue;
        }
        let card = card_at(spot.pos);
        let clear = left[s] == 0;
        painter.rect_filled(
            card,
            6.0,
            if clear {
                theme::MAP_CARD_CLEAR
            } else {
                theme::MAP_CARD
            },
        );
        let edge = if spot_focus == Some(s) || hovered == Some(s) {
            egui::Stroke::new(2.0, GOLD)
        } else {
            egui::Stroke::new(1.0, theme::MAP_BORDER)
        };
        painter.rect_stroke(card, 6.0, edge, egui::StrokeKind::Inside);

        painter.text(
            egui::pos2(card.left() + 12.0, card.center().y),
            egui::Align2::LEFT_CENTER,
            spot.name,
            egui::FontId::proportional(11.5),
            if clear { DIM } else { theme::PARCHMENT },
        );
        painter.text(
            egui::pos2(card.right() - 12.0, card.center().y),
            egui::Align2::RIGHT_CENTER,
            if clear {
                "\u{2714}".to_string()
            } else {
                left[s].to_string()
            },
            egui::FontId::proportional(12.0),
            if clear { GREEN } else { ORANGE },
        );
    }

    if let Some(s) = hovered {
        let (l, t) = (left[s], total[s]);
        response.clone().on_hover_text(if l == 0 {
            format!("{} — all {t} collected", area.spots[s].name)
        } else {
            format!("{} — {l} of {t} left · click to focus", area.spots[s].name)
        });
    }

    if response.clicked() {
        response.interact_pointer_pos().and_then(card_hit)
    } else {
        None
    }
}

/// The world drawn as a live node map: each area's node grows and glows with
/// how much you still need there, and the edges are how the areas actually
/// connect. Returns a clicked area index.
///
/// Drawn with egui's painter rather than shipping an image — the layout is
/// ours, so there's no third-party artwork involved, and unlike a static map
/// it updates as you tick items off.
fn world_map(ui: &mut egui::Ui, appid: u32, remaining: &[usize]) -> Option<usize> {
    let all = areas::areas_for(appid);
    if all.is_empty() || remaining.len() != all.len() {
        return None;
    }

    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 420.0),
        egui::Sense::click(),
    );
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8.0, theme::MAP_BG);
    painter.rect_stroke(
        rect,
        8.0,
        egui::Stroke::new(1.0, theme::MAP_BORDER),
        egui::StrokeKind::Inside,
    );

    // Inset so the labels under the outermost nodes stay inside the panel.
    let inner = rect.shrink2(egui::vec2(70.0, 26.0));
    let at = |p: (f32, f32)| -> egui::Pos2 {
        egui::pos2(
            inner.left() + p.0 * inner.width(),
            inner.top() + p.1 * inner.height(),
        )
    };

    for (a, b) in areas::links_for(appid) {
        let (Some(ia), Some(ib)) = (areas::index_of(appid, a), areas::index_of(appid, b)) else {
            continue;
        };
        painter.line_segment(
            [at(all[ia].pos), at(all[ib].pos)],
            egui::Stroke::new(1.0, theme::MAP_EDGE),
        );
    }

    let busiest = remaining.iter().copied().max().unwrap_or(0).max(1);
    let radius_of = |n: usize| -> f32 {
        if n == 0 {
            5.0
        } else {
            8.0 + 12.0 * (n as f32 / busiest as f32).sqrt()
        }
    };

    // Hit-test a point against the nodes. Hover uses the pointer's resting
    // position; a click uses the position the click actually happened at,
    // which is the only one guaranteed to be set on the click frame.
    let node_at = |p: egui::Pos2| -> Option<usize> {
        let mut best: Option<(usize, f32)> = None;
        for (i, area) in all.iter().enumerate() {
            let d = at(area.pos).distance(p);
            if d <= radius_of(remaining[i]) + 7.0 && best.is_none_or(|(_, bd)| d < bd) {
                best = Some((i, d));
            }
        }
        best.map(|(i, _)| i)
    };
    let hovered = response.hover_pos().and_then(node_at);
    if hovered.is_some() {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }

    for (i, area) in all.iter().enumerate() {
        let c = at(area.pos);
        let left = remaining[i];
        let r = radius_of(left);
        painter.circle_filled(c, r, if left == 0 { theme::MAP_CLEAR } else { ORANGE });
        if hovered == Some(i) {
            painter.circle_stroke(c, r + 3.0, egui::Stroke::new(2.0, GOLD));
        }
        if left > 0 {
            painter.text(
                c,
                egui::Align2::CENTER_CENTER,
                left.to_string(),
                egui::FontId::proportional(11.0),
                egui::Color32::from_rgb(24, 20, 15),
            );
        }
        painter.text(
            c + egui::vec2(0.0, r + 10.0),
            egui::Align2::CENTER_CENTER,
            area.name,
            egui::FontId::proportional(10.0),
            if left == 0 { DIM } else { theme::PARCHMENT },
        );
    }

    if let Some(i) = hovered {
        let left = remaining[i];
        response.clone().on_hover_text(if left == 0 {
            format!("{} — all clear", all[i].name)
        } else {
            format!("{} — {left} left · click to focus", all[i].name)
        });
    }

    if response.clicked() {
        response.interact_pointer_pos().and_then(node_at)
    } else {
        None
    }
}

/// Every outstanding checklist item routed to the area you'll find it in —
/// the "I'm here now, what do I need?" view. Returns a clicked achievement.
fn areas_view(
    ui: &mut egui::Ui,
    game: &games::Game,
    rows: &[AchRow],
    checklist_done: &mut HashMap<String, BTreeSet<String>>,
    hide_done: &mut bool,
    focus: &mut Option<usize>,
    spot_focus: &mut Option<usize>,
) -> Option<String> {
    let mut clicked = None;
    let all_areas = areas::areas_for(game.appid);

    let mut grouped: Vec<Vec<RouteItem>> = Vec::new();
    grouped.resize_with(all_areas.len(), Vec::new);
    let mut unrouted: Vec<RouteItem> = Vec::new();

    for row in rows {
        // An achievement you already earned has nothing left to route.
        if row.unlocked == Some(true) {
            continue;
        }
        let Some(guide) = guides::find(game.appid, &row.name) else {
            continue;
        };
        let mut section = "";
        for item in guide.checklist {
            if let Some(heading) = item.strip_prefix("## ") {
                section = heading;
                continue;
            }
            let entry = RouteItem {
                ach: row.name.as_str(),
                api: row.api_name.as_str(),
                item,
                section,
            };
            match areas::resolve(game.appid, item, section) {
                Some(i) => grouped[i].push(entry),
                None => unrouted.push(entry),
            }
        }
    }

    ui.add_space(8.0);
    ui.label(theme::display(format!("{} — By Area", game.title), 22.0));
    if grouped.iter().all(|g| g.is_empty()) && unrouted.is_empty() {
        ui.label(
            egui::RichText::new(
                "Nothing left to route — the collect-them-all achievements here are done \
                 (or not scanned yet).",
            )
            .color(DIM),
        );
        return None;
    }
    ui.label(
        egui::RichText::new(
            "Every item you still need, grouped by where you go to get it. Tick things off \
             right here as you pick them up.",
        )
        .small()
        .color(DIM),
    );
    ui.add_space(4.0);
    ui.checkbox(hide_done, "Hide items I've already collected");

    let remaining: Vec<usize> = grouped
        .iter()
        .map(|items| {
            items
                .iter()
                .filter(|it| !item_done(checklist_done, game.appid, it.api, it.item))
                .count()
        })
        .collect();

    ui.add_space(6.0);
    match *focus {
        None => {
            if let Some(i) = world_map(ui, game.appid, &remaining) {
                *focus = Some(i);
                *spot_focus = None;
            }
            ui.label(
                egui::RichText::new(
                    "Bigger, brighter nodes have more left. Click an area to open its map.",
                )
                .small()
                .color(DIM),
            );
        }
        Some(i) => {
            let area = &all_areas[i];
            ui.horizontal(|ui| {
                if ui.button("\u{2190} World map").clicked() {
                    *focus = None;
                    *spot_focus = None;
                }
                ui.label(theme::display(area.name, 18.0));
                ui.label(
                    egui::RichText::new(format!("{} left", remaining[i]))
                        .small()
                        .color(if remaining[i] > 0 { ORANGE } else { GREEN }),
                );
                ui.hyperlink_to(
                    egui::RichText::new("\u{1F5FA} area guide & map").small(),
                    area.map_url,
                );
            });
            ui.add_space(4.0);
            if let Some(s) = area_map(
                ui,
                game.appid,
                area,
                &grouped[i],
                checklist_done,
                *spot_focus,
            ) {
                // Clicking the focused spot again clears the filter.
                *spot_focus = if *spot_focus == Some(s) {
                    None
                } else {
                    Some(s)
                };
            }
            ui.horizontal(|ui| {
                if let Some(s) = *spot_focus {
                    ui.label(
                        egui::RichText::new(format!("Showing {} only", area.spots[s].name))
                            .color(GOLD),
                    );
                    if ui.small_button("Show whole area").clicked() {
                        *spot_focus = None;
                    }
                } else {
                    ui.label(
                        egui::RichText::new("Click a spot to narrow the list below to it.")
                            .small()
                            .color(DIM),
                    );
                }
            });
        }
    }
    ui.add_space(4.0);

    let render_group = |ui: &mut egui::Ui,
                        title: &str,
                        map_url: Option<&str>,
                        items: &[&RouteItem],
                        checklist_done: &mut HashMap<String, BTreeSet<String>>,
                        clicked: &mut Option<String>| {
        let remaining = items
            .iter()
            .filter(|it| !item_done(checklist_done, game.appid, it.api, it.item))
            .count();
        if *hide_done && remaining == 0 {
            return;
        }

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(theme::display(title, 17.0));
            ui.label(
                egui::RichText::new(format!("{remaining} left"))
                    .small()
                    .color(if remaining > 0 { ORANGE } else { GREEN }),
            );
            if let Some(url) = map_url {
                ui.hyperlink_to(
                    egui::RichText::new("\u{1F5FA} map & area guide").small(),
                    url,
                );
            }
        });
        ui.separator();

        for it in items {
            let key = format!("{}/{}", game.appid, it.api);
            let mut checked = checklist_done
                .get(&key)
                .is_some_and(|d| d.contains(it.item));
            if *hide_done && checked {
                continue;
            }
            ui.horizontal(|ui| {
                let label = if checked {
                    egui::RichText::new(it.item).strikethrough().color(DIM)
                } else {
                    egui::RichText::new(it.item)
                };
                if ui.checkbox(&mut checked, label).changed() {
                    let done = checklist_done.entry(key.clone()).or_default();
                    if checked {
                        done.insert(it.item.to_string());
                    } else {
                        done.remove(it.item);
                    }
                }
                if ui
                    .link(egui::RichText::new(it.ach).small().color(DIM))
                    .on_hover_text("Open this achievement's guide")
                    .clicked()
                {
                    *clicked = Some(it.api.to_string());
                }
            });
        }
    };

    match *focus {
        // Drilled into an area: mirror the map and list spot by spot.
        Some(i) => {
            let area = &all_areas[i];
            let mut by_spot: Vec<Vec<&RouteItem>> = Vec::new();
            by_spot.resize_with(area.spots.len(), Vec::new);
            let mut elsewhere: Vec<&RouteItem> = Vec::new();
            for it in &grouped[i] {
                match areas::resolve_spot(area, it.item, it.section) {
                    Some(s) => by_spot[s].push(it),
                    None => elsewhere.push(it),
                }
            }
            for (s, items) in by_spot.iter().enumerate() {
                if items.is_empty() || spot_focus.is_some_and(|f| f != s) {
                    continue;
                }
                render_group(
                    ui,
                    area.spots[s].name,
                    None,
                    items,
                    checklist_done,
                    &mut clicked,
                );
            }
            if !elsewhere.is_empty() && spot_focus.is_none() {
                render_group(
                    ui,
                    &format!("Elsewhere in {}", area.name),
                    None,
                    &elsewhere,
                    checklist_done,
                    &mut clicked,
                );
            }
        }
        // World view: one group per area.
        None => {
            for (i, items) in grouped.iter().enumerate() {
                if items.is_empty() {
                    continue;
                }
                let refs: Vec<&RouteItem> = items.iter().collect();
                render_group(
                    ui,
                    all_areas[i].name,
                    Some(all_areas[i].map_url),
                    &refs,
                    checklist_done,
                    &mut clicked,
                );
            }
            if !unrouted.is_empty() {
                let refs: Vec<&RouteItem> = unrouted.iter().collect();
                render_group(
                    ui,
                    "No fixed location / general steps",
                    None,
                    &refs,
                    checklist_done,
                    &mut clicked,
                );
            }
        }
    }

    clicked
}

/// Everything permanently lockable this playthrough, deadlines up front.
/// Returns a clicked achievement.
fn missables_view(ui: &mut egui::Ui, game: &games::Game, rows: &[AchRow]) -> Option<String> {
    let mut clicked = None;

    let mut missables: Vec<(&AchRow, &'static guides::Guide)> = rows
        .iter()
        .filter_map(|r| {
            guides::find(game.appid, &r.name)
                .filter(|g| g.missable)
                .map(|g| (r, g))
        })
        .collect();
    // Locked (still at risk) first, then secured.
    missables.sort_by_key(|(r, _)| r.unlocked == Some(true));
    let at_risk = missables
        .iter()
        .filter(|(r, _)| r.unlocked != Some(true))
        .count();

    ui.add_space(8.0);
    ui.label(theme::display(format!("{} — Missables", game.title), 22.0));
    if missables.is_empty() {
        ui.label(
            egui::RichText::new("Nothing here can be permanently missed. Play on.").color(DIM),
        );
        return None;
    }
    ui.label(
        egui::RichText::new(format!(
            "{at_risk} still at risk this playthrough. Check this screen before every major boss."
        ))
        .small()
        .color(if at_risk > 0 { ORANGE } else { GREEN }),
    );
    ui.add_space(6.0);
    ui.separator();

    for (row, guide) in missables {
        let unlocked = row.unlocked == Some(true);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(if unlocked {
                egui::RichText::new("\u{2714}").color(GREEN)
            } else {
                egui::RichText::new("\u{26A0}").color(ORANGE)
            });
            let name = if unlocked {
                egui::RichText::new(&row.name).strikethrough().color(DIM)
            } else {
                egui::RichText::new(&row.name).strong()
            };
            if ui.link(name).clicked() {
                clicked = Some(row.api_name.clone());
            }
            if let Some(p) = row.percent {
                ui.label(egui::RichText::new(format!("{p:.1}%")).small().color(DIM));
            }
        });
        if !guide.deadline.is_empty() && !unlocked {
            ui.horizontal(|ui| {
                ui.add_space(24.0);
                ui.label(egui::RichText::new(guide.deadline).color(ORANGE).italics());
            });
        }
    }
    clicked
}

/// Phase-grouped roadmap for one game. Returns a clicked achievement.
fn planner_view(
    ui: &mut egui::Ui,
    game: &games::Game,
    rows: &[AchRow],
    checklist_done: &HashMap<String, BTreeSet<String>>,
) -> Option<String> {
    let mut clicked = None;

    ui.add_space(8.0);
    ui.label(theme::display(format!("{} — Roadmap", game.title), 22.0));
    ui.label(
        egui::RichText::new(
            "Grouped by the earliest playthrough each achievement realistically lands in. \
             Click one to open its guide.",
        )
        .small()
        .color(DIM),
    );

    let phases: [(&str, &str); 5] = [
        ("NG", "\u{2694} This playthrough"),
        ("NG+", "\u{27F3} New Game+"),
        ("NG++", "\u{27F3}\u{27F3} NG++ and beyond"),
        ("Multiple runs", "\u{1F4BE} Multiple runs / save backups"),
        ("", "\u{2726} Anytime"),
    ];

    for (phase_key, phase_label) in phases {
        let group: Vec<&AchRow> = rows
            .iter()
            .filter(|r| {
                guides::find(game.appid, &r.name)
                    .map(|g| g.phase)
                    .unwrap_or("")
                    == phase_key
            })
            .collect();
        if group.is_empty() {
            continue;
        }
        let done = group.iter().filter(|r| r.unlocked == Some(true)).count();

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label(theme::display(phase_label, 17.0));
            ui.label(
                egui::RichText::new(format!("{done}/{}", group.len()))
                    .small()
                    .color(DIM),
            );
        });
        ui.separator();

        for row in group {
            let unlocked = row.unlocked == Some(true);
            let guide = guides::find(game.appid, &row.name);
            ui.horizontal(|ui| {
                ui.label(if unlocked {
                    egui::RichText::new("\u{2714}").color(GREEN)
                } else {
                    egui::RichText::new("\u{1F512}").color(DIM)
                });
                let name = if unlocked {
                    egui::RichText::new(&row.name).strikethrough().color(DIM)
                } else {
                    egui::RichText::new(&row.name)
                };
                if ui.link(name).clicked() {
                    clicked = Some(row.api_name.clone());
                }
                if guide.is_some_and(|g| g.missable) && !unlocked {
                    ui.label(egui::RichText::new("\u{26A0}").color(ORANGE))
                        .on_hover_text("Missable — open the guide before playing on");
                }
                if let Some(g) = guide
                    && !g.checklist.is_empty()
                    && !unlocked
                {
                    let key = format!("{}/{}", game.appid, row.api_name);
                    let (n, total) = checklist_progress(checklist_done.get(&key), g.checklist);
                    ui.label(
                        egui::RichText::new(format!("\u{2611} {n}/{total}"))
                            .small()
                            .color(BLUE),
                    );
                }
                if let Some(p) = row.percent {
                    ui.label(egui::RichText::new(format!("{p:.1}%")).small().color(DIM));
                }
            });
        }
    }
    clicked
}

impl App {
    fn dashboard(&mut self, ui: &mut egui::Ui) {
        let connected = self.connected();
        let mut clicked: Option<usize> = None;
        let mut open_settings = false;

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    ui.label(theme::display("Your 100% Hunt at a Glance", 24.0));
                    if !connected {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Connect your Steam account to scan your collection.");
                            if ui.button("\u{2699} Open Settings").clicked() {
                                open_settings = true;
                            }
                        });
                    } else {
                        ui.label(
                            egui::RichText::new(
                                "Sorted by how close you are to 100%. Click a game to dig in.",
                            )
                            .small()
                            .color(DIM),
                        );
                    }
                    ui.add_space(10.0);

                    // Completion fraction for sorting; no-data games sink to the bottom
                    // in catalog order.
                    let frac = |i: &usize| -> f32 {
                        match self.data.get(&GAMES[*i].appid) {
                            Some(Loaded::Ready(entry))
                                if entry.data.player_error.is_none()
                                    && !entry.data.achievements.is_empty() =>
                            {
                                let unlocked = entry
                                    .data
                                    .achievements
                                    .iter()
                                    .filter(|a| a.unlocked == Some(true))
                                    .count();
                                unlocked as f32 / entry.data.achievements.len() as f32
                            }
                            Some(Loaded::Ready(_)) => -1.0,
                            _ => -2.0,
                        }
                    };
                    let mut order: Vec<usize> = (0..GAMES.len()).collect();
                    order.sort_by(|a, b| {
                        frac(b)
                            .partial_cmp(&frac(a))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                    for i in order {
                        let game = &GAMES[i];
                        if game_card(ui, game, self.data.get(&game.appid)) {
                            clicked = Some(i);
                        }
                        ui.add_space(6.0);
                    }
                });
        });

        if open_settings {
            self.settings_open = true;
        }
        if let Some(i) = clicked {
            self.selected_game = Some(i);
        }
    }
}

/// One dashboard card. Returns true when clicked.
fn game_card(ui: &mut egui::Ui, game: &games::Game, state: Option<&Loaded>) -> bool {
    let bg_slot = ui.painter().add(egui::Shape::Noop);
    let resp = egui::Frame::group(ui.style())
        .inner_margin(12.0)
        .corner_radius(8.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::new(header_art_url(game.appid))
                        .fit_to_exact_size(egui::vec2(184.0, 86.0))
                        .corner_radius(6.0),
                );
                ui.vertical(|ui| {
                    ui.label(theme::display(game.title, 17.0));
                    ui.add_space(4.0);
                    game_card_status(ui, state);
                });
            });
        })
        .response;

    let resp = resp.interact(egui::Sense::click());
    if resp.hovered() {
        ui.painter().set(
            bg_slot,
            egui::Shape::rect_filled(resp.rect, 8.0, theme::ROW_HOVER),
        );
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}

fn game_card_status(ui: &mut egui::Ui, state: Option<&Loaded>) {
    match state {
        Some(Loaded::Ready(entry)) => {
            let gd = &entry.data;
            let total = gd.achievements.len();
            let unlocked = gd
                .achievements
                .iter()
                .filter(|a| a.unlocked == Some(true))
                .count();
            if gd.player_error.is_none() && total > 0 {
                let f = unlocked as f32 / total as f32;
                ui.horizontal(|ui| {
                    ui.add(
                        egui::ProgressBar::new(f)
                            .desired_width(260.0)
                            .desired_height(12.0),
                    );
                    let pct =
                        egui::RichText::new(format!("{unlocked} / {total}  ({:.0}%)", f * 100.0))
                            .strong();
                    ui.label(if f >= 1.0 { pct.color(GOLD) } else { pct });
                    if f >= 1.0 {
                        ui.label(egui::RichText::new("\u{1F3C6} 100%!").color(GOLD));
                    }
                });
                let last = gd
                    .achievements
                    .iter()
                    .filter_map(|a| a.unlock_time)
                    .max()
                    .map(|t| format!("Last unlock: {} · ", format_unix(t)))
                    .unwrap_or_default();
                ui.label(
                    egui::RichText::new(format!("{last}scanned {}", ago(entry.fetched_at)))
                        .small()
                        .color(DIM),
                );
                if let Some(next) = suggested_next(&gd.achievements).first() {
                    ui.label(
                        egui::RichText::new(format!(
                            "Next easiest: {} ({:.0}%)",
                            next.name,
                            next.percent.unwrap_or(0.0)
                        ))
                        .small()
                        .color(GOLD.gamma_multiply(0.8)),
                    );
                }
            } else {
                ui.label(
                    egui::RichText::new(format!(
                        "\u{26A0} Global stats only — {}",
                        gd.player_error.as_deref().unwrap_or("no player data")
                    ))
                    .small()
                    .color(ORANGE),
                );
            }
        }
        Some(Loaded::Loading) => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(egui::RichText::new("Scanning…").color(DIM));
            });
        }
        Some(Loaded::Failed(e)) => {
            ui.label(egui::RichText::new(e.as_str()).small().color(RED));
        }
        None => {
            ui.label(
                egui::RichText::new("Not scanned — connect Steam in Settings")
                    .small()
                    .color(DIM),
            );
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll_messages();
        self.ensure_loaded(&ctx);
        self.auto_refresh(&ctx);
        self.top_bar(ui);
        self.settings_window(&ctx);
        self.left_panel(ui);
        self.central_panel(ui);
        self.draw_toasts(&ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "settings", &self.settings);
        eframe::set_value(storage, "checklists_v2", &self.checklist_done);
        eframe::set_value(storage, "notes", &self.notes);
        let cache: HashMap<u32, &ScanEntry> = self
            .data
            .iter()
            .filter_map(|(appid, loaded)| match loaded {
                Loaded::Ready(entry) => Some((*appid, entry)),
                _ => None,
            })
            .collect();
        eframe::set_value(storage, "scan_cache", &cache);
    }
}

// ---------------------------------------------------------------------- UI helpers

/// Returns true when the row was clicked.
fn ach_list_row(
    ui: &mut egui::Ui,
    appid: u32,
    row: &AchRow,
    selected: bool,
    checklist_done: &HashMap<String, BTreeSet<String>>,
) -> bool {
    let guide = guides::find(appid, &row.name);
    let unlocked = row.unlocked == Some(true);

    let bg_slot = ui.painter().add(egui::Shape::Noop);
    let fill = if selected {
        ui.visuals().selection.bg_fill.gamma_multiply(0.5)
    } else {
        egui::Color32::TRANSPARENT
    };
    let resp = egui::Frame::new()
        .fill(fill)
        .inner_margin(6.0)
        .corner_radius(6.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let icon = if unlocked { &row.icon } else { &row.icon_gray };
                if !icon.is_empty() {
                    ui.add(
                        egui::Image::new(icon)
                            .fit_to_exact_size(egui::vec2(40.0, 40.0))
                            .corner_radius(4.0),
                    );
                }
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let name = egui::RichText::new(&row.name).strong();
                        ui.label(if unlocked { name.color(GREEN) } else { name });
                        if guide.is_some_and(|g| g.missable) {
                            ui.label(
                                egui::RichText::new("\u{26A0} missable")
                                    .small()
                                    .color(ORANGE),
                            );
                        }
                    });
                    ui.horizontal(|ui| {
                        match row.unlocked {
                            Some(true) => {
                                ui.label(egui::RichText::new("\u{2714}").color(GREEN).small());
                            }
                            Some(false) => {
                                ui.label(egui::RichText::new("\u{1F512}").small());
                            }
                            None => {}
                        }
                        if let Some(p) = row.percent {
                            let (rarity, color) = rarity_label(p);
                            ui.add(
                                egui::ProgressBar::new(p / 100.0)
                                    .desired_width(90.0)
                                    .desired_height(9.0),
                            );
                            ui.label(egui::RichText::new(format!("{p:.1}%")).small().color(DIM));
                            ui.label(
                                egui::RichText::new(format!(" {rarity} "))
                                    .small()
                                    .color(color)
                                    .background_color(color.gamma_multiply(0.15)),
                            );
                        }
                        if let Some(g) = guide
                            && !g.checklist.is_empty()
                            && !unlocked
                        {
                            let key = format!("{}/{}", appid, row.api_name);
                            let (n, total) =
                                checklist_progress(checklist_done.get(&key), g.checklist);
                            if n > 0 {
                                ui.label(
                                    egui::RichText::new(format!("\u{2611} {n}/{total}"))
                                        .small()
                                        .color(BLUE),
                                );
                            }
                        }
                    });
                });
            });
        })
        .response;

    let resp = resp.interact(egui::Sense::click());
    if resp.hovered() && !selected {
        ui.painter().set(
            bg_slot,
            egui::Shape::rect_filled(resp.rect, 6.0, theme::ROW_HOVER),
        );
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}

/// Locked achievements with known rarity, most common first — the
/// "you could probably go get these right now" picks.
fn suggested_next(rows: &[AchRow]) -> Vec<&AchRow> {
    let mut v: Vec<&AchRow> = rows
        .iter()
        .filter(|a| a.unlocked == Some(false) && a.percent.is_some())
        .collect();
    v.sort_by(|a, b| {
        b.percent
            .unwrap_or(0.0)
            .partial_cmp(&a.percent.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    v.truncate(3);
    v
}

/// (ticked, total) for a checklist, counting only non-header items.
fn checklist_progress(done: Option<&BTreeSet<String>>, items: &[&str]) -> (usize, usize) {
    let mut n = 0;
    let mut total = 0;
    for item in items {
        if item.starts_with("## ") {
            continue;
        }
        total += 1;
        if done.is_some_and(|d| d.contains(*item)) {
            n += 1;
        }
    }
    (n, total)
}

fn ach_details(
    ui: &mut egui::Ui,
    game: &games::Game,
    row: &AchRow,
    checklist_done: &mut HashMap<String, BTreeSet<String>>,
    checklist_filter: &mut String,
    notes: &mut HashMap<String, String>,
) {
    let guide = guides::find(game.appid, &row.name);

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let icon = if row.unlocked == Some(true) {
            &row.icon
        } else {
            &row.icon_gray
        };
        if !icon.is_empty() {
            ui.add(
                egui::Image::new(icon)
                    .fit_to_exact_size(egui::vec2(72.0, 72.0))
                    .corner_radius(6.0),
            );
        }
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(theme::display(&row.name, 22.0));
                if guide.is_some_and(|g| g.missable) {
                    ui.label(
                        egui::RichText::new("\u{26A0} MISSABLE")
                            .strong()
                            .color(ORANGE),
                    );
                }
            });
            if row.description.is_empty() && row.hidden {
                ui.label(
                    egui::RichText::new(
                        "Hidden achievement — Steam hides its description. \
                         The links below reveal it (spoilers).",
                    )
                    .italics()
                    .color(DIM),
                );
            } else {
                ui.label(&row.description);
            }
            match row.unlocked {
                Some(true) => {
                    let when = row
                        .unlock_time
                        .map(|t| format!(" on {}", format_unix(t)))
                        .unwrap_or_default();
                    ui.label(egui::RichText::new(format!("\u{2714} Unlocked{when}")).color(GREEN));
                }
                Some(false) => {
                    ui.label(egui::RichText::new("\u{1F512} Locked").color(DIM));
                }
                None => {
                    ui.label(
                        egui::RichText::new("Personal progress unavailable")
                            .color(DIM)
                            .small(),
                    );
                }
            }
        });
    });

    if let Some(p) = row.percent {
        ui.add_space(8.0);
        let (rarity, color) = rarity_label(p);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{p:.1}% of players have this")).strong());
            ui.label(
                egui::RichText::new(format!(" {rarity} "))
                    .color(color)
                    .background_color(color.gamma_multiply(0.15)),
            );
        });
        ui.add(egui::ProgressBar::new(p / 100.0).desired_height(10.0));
    }

    ui.add_space(10.0);
    ui.separator();

    if let Some(g) = guide {
        if !g.rules.is_empty() {
            ui.add_space(6.0);
            let frame_color = if g.missable { ORANGE } else { BLUE };
            egui::Frame::new()
                .stroke(egui::Stroke::new(1.0, frame_color.gamma_multiply(0.6)))
                .inner_margin(10.0)
                .corner_radius(6.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(if g.missable {
                            "\u{26A0} Rules — read before you play on"
                        } else {
                            "\u{2139} Rules & requirements"
                        })
                        .strong()
                        .color(frame_color),
                    );
                    for r in g.rules {
                        bullet(ui, r);
                    }
                });
        }
        if !g.locations.is_empty() {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("\u{1F4CD} Locations").strong());
            for l in g.locations {
                bullet(ui, l);
            }
        }
        if !g.checklist.is_empty() {
            ui.add_space(8.0);
            let key = format!("{}/{}", game.appid, row.api_name);
            let unlocked = row.unlocked == Some(true);
            let (n, total) = checklist_progress(checklist_done.get(&key), g.checklist);
            let header = format!("\u{2611} Checklist — {n}/{total} collected");
            egui::CollapsingHeader::new(egui::RichText::new(header).strong())
                .default_open(!unlocked)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(
                            "Tracked locally on this computer — Steam can't see partial progress.",
                        )
                        .small()
                        .color(DIM),
                    );
                    ui.add(
                        egui::ProgressBar::new(if total > 0 {
                            n as f32 / total as f32
                        } else {
                            0.0
                        })
                        .desired_height(8.0),
                    );
                    ui.horizontal(|ui| {
                        if total > 12 {
                            ui.label("\u{1F50D}");
                            ui.add(
                                egui::TextEdit::singleline(checklist_filter)
                                    .desired_width(180.0)
                                    .hint_text("Filter items…"),
                            );
                        }
                        if ui.small_button("Check all").clicked() {
                            let done = checklist_done.entry(key.clone()).or_default();
                            for item in g.checklist {
                                if !item.starts_with("## ") {
                                    done.insert((*item).to_string());
                                }
                            }
                        }
                        if ui.small_button("Clear all").clicked() {
                            checklist_done.remove(&key);
                        }
                    });
                    let needle = checklist_filter.to_lowercase();
                    let done = checklist_done.entry(key.clone()).or_default();
                    for item in g.checklist {
                        if let Some(heading) = item.strip_prefix("## ") {
                            if needle.is_empty() {
                                ui.add_space(6.0);
                                ui.label(theme::display(heading, 14.0));
                            }
                            continue;
                        }
                        if !needle.is_empty() && !item.to_lowercase().contains(&needle) {
                            continue;
                        }
                        let mut checked = done.contains(*item);
                        let label = if checked {
                            egui::RichText::new(*item).strikethrough().color(DIM)
                        } else {
                            egui::RichText::new(*item)
                        };
                        if ui.checkbox(&mut checked, label).changed() {
                            if checked {
                                done.insert((*item).to_string());
                            } else {
                                done.remove(*item);
                            }
                        }
                    }
                });
        }
        if !g.resources.is_empty() {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("\u{1F4D6} Hand-picked resources").strong());
            for r in g.resources {
                ui.horizontal(|ui| {
                    ui.label("  •");
                    ui.hyperlink_to(r.label, r.url);
                });
            }
        }
    }

    // Personal notes, saved locally.
    ui.add_space(8.0);
    let note_key = format!("{}/{}", game.appid, row.api_name);
    let has_note = notes.get(&note_key).is_some_and(|n| !n.trim().is_empty());
    egui::CollapsingHeader::new(egui::RichText::new("\u{1F4DD} My notes").strong())
        .default_open(has_note)
        .show(ui, |ui| {
            let note = notes.entry(note_key.clone()).or_default();
            ui.add(
                egui::TextEdit::multiline(note)
                    .desired_width(f32::INFINITY)
                    .desired_rows(3)
                    .hint_text("Build reminders, co-op partners, where you left off…"),
            );
        });
    if notes.get(&note_key).is_some_and(|n| n.trim().is_empty()) {
        notes.remove(&note_key);
    }

    // Every achievement gets search links, curated or not.
    ui.add_space(8.0);
    ui.label(egui::RichText::new("\u{1F50E} Find more help").strong());
    let query = urlencode(&format!("{} {} achievement guide", game.title, row.name));
    ui.horizontal(|ui| {
        ui.label("  •");
        ui.hyperlink_to(
            "\u{1F3A5} YouTube — video guides",
            format!("https://www.youtube.com/results?search_query={query}"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("  •");
        ui.hyperlink_to(
            "\u{1F4DC} Google — written guides",
            format!("https://www.google.com/search?q={query}"),
        );
    });

    ui.add_space(12.0);
    ui.separator();
    egui::CollapsingHeader::new(format!("Game-wide notes & guides for {}", game.title))
        .default_open(guide.is_none())
        .show(ui, |ui| {
            game_notes_and_resources(ui, game);
        });
}

/// Returns the api_name of a clicked suggested achievement.
fn game_overview(
    ui: &mut egui::Ui,
    game: &games::Game,
    rows: Option<&[AchRow]>,
    connected: bool,
    settings_open: &mut bool,
) -> Option<String> {
    let mut clicked = None;
    ui.add_space(12.0);
    ui.add(
        egui::Image::new(header_art_url(game.appid))
            .fit_to_exact_size(egui::vec2(460.0, 215.0))
            .corner_radius(8.0),
    );
    ui.add_space(8.0);
    ui.label(theme::display(game.title, 24.0));
    ui.add_space(4.0);
    if connected {
        ui.label("Select an achievement on the left to see rules, locations, and guides.");
    } else {
        ui.label("Connect your Steam account to scan your achievement progress.");
        if ui.button("\u{2699} Open Settings").clicked() {
            *settings_open = true;
        }
    }

    if let Some(rows) = rows {
        let picks = suggested_next(rows);
        if !picks.is_empty() {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("\u{1F3AF} Suggested next — most players have these")
                    .strong()
                    .color(GOLD),
            );
            for a in picks {
                ui.horizontal(|ui| {
                    if !a.icon_gray.is_empty() {
                        ui.add(
                            egui::Image::new(&a.icon_gray)
                                .fit_to_exact_size(egui::vec2(24.0, 24.0))
                                .corner_radius(3.0),
                        );
                    }
                    if ui.link(egui::RichText::new(&a.name).strong()).clicked() {
                        clicked = Some(a.api_name.clone());
                    }
                    if let Some(p) = a.percent {
                        let (rarity, color) = rarity_label(p);
                        ui.label(egui::RichText::new(format!("{p:.1}%")).small().color(DIM));
                        ui.label(
                            egui::RichText::new(format!(" {rarity} "))
                                .small()
                                .color(color)
                                .background_color(color.gamma_multiply(0.15)),
                        );
                    }
                });
            }
        }
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);
    game_notes_and_resources(ui, game);
    clicked
}

fn game_notes_and_resources(ui: &mut egui::Ui, game: &games::Game) {
    if !game.notes.is_empty() {
        ui.label(
            egui::RichText::new("\u{26A0} Know before you 100% this game")
                .strong()
                .color(ORANGE),
        );
        for n in game.notes {
            bullet(ui, n);
        }
        ui.add_space(8.0);
    }
    if !game.mechanics.is_empty() {
        ui.label(
            egui::RichText::new("\u{1F9ED} Mechanics primer")
                .strong()
                .color(BLUE),
        );
        for m in game.mechanics {
            bullet(ui, m);
        }
        ui.add_space(8.0);
    }
    if !game.farming.is_empty() {
        ui.label(
            egui::RichText::new("\u{1F33E} Farming spots")
                .strong()
                .color(GREEN),
        );
        for f in game.farming {
            bullet(ui, f);
        }
        ui.add_space(8.0);
    }
    ui.label(egui::RichText::new("\u{1F4D6} Best overall resources").strong());
    for r in game.resources {
        ui.horizontal(|ui| {
            ui.label("  •");
            ui.hyperlink_to(r.label, r.url);
        });
    }
    ui.horizontal(|ui| {
        ui.label("  •");
        let q = urlencode(&format!("{} 100% achievement guide", game.title));
        ui.hyperlink_to(
            "\u{1F3A5} YouTube — full 100% walkthroughs",
            format!("https://www.youtube.com/results?search_query={q}"),
        );
    });
}

fn bullet(ui: &mut egui::Ui, text: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label("  •");
        ui.label(text);
    });
}

// ---------------------------------------------------------------------- logic helpers

fn filter_sort<'a>(
    achs: &'a [AchRow],
    appid: u32,
    search: &str,
    status: StatusFilter,
    missable_only: bool,
    sort: SortBy,
) -> Vec<&'a AchRow> {
    let needle = search.to_lowercase();
    let mut rows: Vec<&AchRow> = achs
        .iter()
        .filter(|a| {
            (match status {
                StatusFilter::All => true,
                StatusFilter::Locked => a.unlocked != Some(true),
                StatusFilter::Unlocked => a.unlocked == Some(true),
            }) && (!missable_only || guides::find(appid, &a.name).is_some_and(|g| g.missable))
                && (needle.is_empty()
                    || a.name.to_lowercase().contains(&needle)
                    || a.description.to_lowercase().contains(&needle))
        })
        .collect();

    match sort {
        SortBy::MostCommon => rows.sort_by(|a, b| {
            b.percent
                .unwrap_or(-1.0)
                .partial_cmp(&a.percent.unwrap_or(-1.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        SortBy::Rarest => rows.sort_by(|a, b| {
            a.percent
                .unwrap_or(101.0)
                .partial_cmp(&b.percent.unwrap_or(101.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        SortBy::Name => rows.sort_by_key(|a| a.name.to_lowercase()),
        SortBy::RecentUnlocks => {
            rows.sort_by_key(|a| std::cmp::Reverse(a.unlock_time.unwrap_or(0)))
        }
    }
    rows
}

fn rarity_label(p: f32) -> (&'static str, egui::Color32) {
    if p >= 50.0 {
        ("Common", DIM)
    } else if p >= 20.0 {
        ("Uncommon", GREEN)
    } else if p >= 10.0 {
        ("Rare", BLUE)
    } else if p >= 5.0 {
        ("Very rare", PURPLE)
    } else {
        ("Ultra rare", GOLD)
    }
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Unix timestamp -> "YYYY-MM-DD HH:MM UTC" without pulling in a date crate.
fn format_unix(ts: i64) -> String {
    let days = ts.div_euclid(86_400);
    let secs = ts.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02} UTC",
        secs / 3600,
        (secs % 3600) / 60
    )
}

/// Howard Hinnant's civil_from_days algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_formatting() {
        assert_eq!(format_unix(0), "1970-01-01 00:00 UTC");
        // 2022-02-25 00:00:00 UTC — Elden Ring launch day.
        assert_eq!(format_unix(1_645_747_200), "2022-02-25 00:00 UTC");
    }

    #[test]
    fn url_encoding() {
        assert_eq!(urlencode("DARK SOULS III"), "DARK+SOULS+III");
        assert_eq!(urlencode("Knight's Honor"), "Knight%27s+Honor");
    }

    #[test]
    fn steam_input_parsing() {
        use crate::steam::{SteamInput, parse_steam_input};
        assert!(matches!(
            parse_steam_input("76561198000000000"),
            SteamInput::Id(id) if id == "76561198000000000"
        ));
        assert!(matches!(
            parse_steam_input("https://steamcommunity.com/id/gamer/"),
            SteamInput::Vanity(v) if v == "gamer"
        ));
        assert!(matches!(
            parse_steam_input("https://steamcommunity.com/profiles/76561198000000000"),
            SteamInput::Id(_)
        ));
        assert!(matches!(parse_steam_input("  "), SteamInput::Empty));
    }

    #[test]
    fn guides_match_expected_names() {
        // Representative Steam display names must hit their curated guides.
        for (appid, name, missable) in [
            (1245620, "Legendary Armaments", true),
            (1245620, "Age of the Stars", true),
            (374320, "The Usurpation of Fire", true),
            (814380, "Man Without Equal", true),
            (1888160, "Asset Holder", false),
            (570940, "Knight's Honor", true),
            (335300, "Master of Hexes", false),
            (236430, "Master of Hexes", false), // original DS2 shares guides
            (211420, "Knight's Honor", true),   // PTDE shares DS1R guides
            (1245620, "Shardbearer Radahn", false),
            (1245620, "Great Rune", false),
            (570940, "Covenant: Blade of the Darkmoon", true),
            (570940, "Covenant: Gravelord Servant", false),
            (374320, "Covenant: Mound-makers", true),
            (374320, "Covenant: Aldrich Faithful", false),
            (814380, "Peak Physical Strength", false),
        ] {
            let g = crate::guides::find(appid, name)
                .unwrap_or_else(|| panic!("no guide matched {appid} / {name}"));
            assert_eq!(g.missable, missable, "missable flag for {name}");
        }
        // Sekiro is intentionally not fully covered yet — uncurated names miss.
        assert!(crate::guides::find(814380, "Sunset Memory").is_none());
    }

    #[test]
    fn scan_entry_serde_round_trip() {
        let entry = ScanEntry {
            data: crate::steam::GameData {
                achievements: vec![crate::steam::AchRow {
                    api_name: "ACH_1".into(),
                    name: "Elden Lord".into(),
                    description: "Achieved an ending".into(),
                    hidden: true,
                    icon: "https://example.com/i.jpg".into(),
                    icon_gray: "https://example.com/g.jpg".into(),
                    percent: Some(31.4),
                    unlocked: Some(true),
                    unlock_time: Some(1_700_000_000),
                }],
                player_error: None,
            },
            fetched_at: 1_752_000_000,
        };
        let json = serde_json::to_string(&entry).unwrap();
        let back: ScanEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.fetched_at, entry.fetched_at);
        assert_eq!(back.data.achievements.len(), 1);
        assert_eq!(back.data.achievements[0].name, "Elden Lord");
        assert_eq!(back.data.achievements[0].percent, Some(31.4));
    }

    #[test]
    fn checklist_progress_excludes_headers() {
        let items = ["## Group A", "sword", "shield", "## Group B", "spear"];
        let mut done = BTreeSet::new();
        done.insert("sword".to_string());
        done.insert("## Group A".to_string()); // headers never count even if present
        assert_eq!(checklist_progress(Some(&done), &items), (1, 3));
        assert_eq!(checklist_progress(None, &items), (0, 3));
    }

    #[test]
    fn no_duplicate_checklist_items() {
        for g in crate::guides::all_guides() {
            let mut seen = std::collections::HashSet::new();
            for item in g.checklist {
                assert!(
                    seen.insert(*item),
                    "duplicate checklist item {item:?} in guide matching {:?}",
                    g.matches
                );
            }
        }
    }

    #[test]
    fn collectathon_checklists_are_item_level() {
        for (appid, name, floor) in [
            (570940, "Knight's Honor", 40),
            (570940, "Bond of a Pyromancer", 15),
            (570940, "Wisdom of a Sage", 20),
            (570940, "Prayer of a Maiden", 20),
            (374320, "Master of Sorceries", 30),
            (374320, "Master of Miracles", 30),
            (374320, "Master of Pyromancies", 25),
        ] {
            let g = crate::guides::find(appid, name).unwrap();
            let (_, total) = checklist_progress(None, g.checklist);
            assert!(
                total >= floor,
                "{name}: expected at least {floor} checkable items, got {total}"
            );
        }
    }

    #[test]
    fn checklist_state_serde_round_trip() {
        let mut map: HashMap<String, BTreeSet<String>> = HashMap::new();
        map.entry("570940/ACH_KH".into())
            .or_default()
            .insert("Drake Sword (Hellkite Drake's tail, Undead Burg bridge)".into());
        let json = serde_json::to_string(&map).unwrap();
        let back: HashMap<String, BTreeSet<String>> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, map);
    }

    #[test]
    fn exact_match_beats_substring() {
        // "Godfrey, First Elden Lord" contains the substring "elden lord",
        // but must resolve to its own boss entry, not the ending entry.
        let g = crate::guides::find(1245620, "Godfrey, First Elden Lord").unwrap();
        assert!(g.matches.contains(&"godfrey, first elden lord"));
        // "Regal Ancestor Spirit" contains "ancestor spirit".
        let g = crate::guides::find(1245620, "Regal Ancestor Spirit").unwrap();
        assert!(g.matches.contains(&"regal ancestor spirit"));
    }

    #[test]
    fn full_coverage_for_soulsborne_catalog() {
        // Every Steam achievement display name in these four games must
        // resolve to a curated guide (lists sourced from Steam stats pages).
        let er = [
            "Roundtable Hold",
            "Margit, the Fell Omen",
            "Shardbearer Godrick",
            "Great Rune",
            "Red Wolf of Radagon",
            "Rennala, Queen of the Full Moon",
            "Leonine Misbegotten",
            "Royal Knight Loretta",
            "Shardbearer Radahn",
            "Mimic Tear",
            "Godfrey, First Elden Lord",
            "Shardbearer Morgott",
            "Magma Wyrm Makar",
            "God-Slaying Armament",
            "Fire Giant",
            "Erdtree Aflame",
            "Commander Niall",
            "Shardbearer Rykard",
            "Astel, Naturalborn of the Void",
            "Godskin Duo",
            "Godskin Noble",
            "Ancestor Spirit",
            "Maliketh the Black Blade",
            "Shardbearer Mohg",
            "Hoarah Loux, Warrior",
            "Valiant Gargoyles",
            "Loretta, Knight of the Haligtree",
            "Elemer of the Briar",
            "Dragonkin Soldier of Nokstella",
            "Regal Ancestor Spirit",
            "Shardbearer Malenia",
            "Mohg, the Omen",
            "Dragonlord Placidusax",
            "Lichdragon Fortissax",
            "Age of the Stars",
            "Elden Lord",
            "Legendary Armaments",
            "Legendary Talismans",
            "Legendary Ashen Remains",
            "Legendary Sorceries and Incantations",
            "Lord of Frenzied Flame",
            "Elden Ring",
        ];
        let ds1 = [
            "Enkindle",
            "Estus Flask",
            "Reach Lordran",
            "Ring the Bell (Undead Church)",
            "Covenant: Way of White",
            "Ring the Bell (Quelaag's Domain)",
            "Reach Anor Londo",
            "Lordvessel",
            "Rite of Kindling",
            "Covenant: Chaos Servant",
            "Art of Abysswalking",
            "Defeat Seath the Scaleless",
            "Defeat Gravelord Nito",
            "Defeat the Four Kings",
            "Defeat Bed of Chaos",
            "Covenant: Princess's Guard",
            "Covenant: Forest Hunter",
            "Defeat Crossbreed Priscilla",
            "Defeat the Dark Sun Gwyndolin",
            "To Link the Fire",
            "Covenant: Gravelord Servant",
            "Covenant: Path of the Dragon",
            "Strongest Weapon",
            "Covenant: Blade of the Darkmoon",
            "Covenant: Warrior of Sunlight",
            "Dark Lord",
            "Covenant: Darkwraith",
            "Lightning Weapon",
            "Raw Weapon",
            "Divine Weapon",
            "Enchanted Weapon",
            "Crystal Weapon",
            "Wisdom of a Sage",
            "Chaos Weapon",
            "Occult Weapon",
            "Fire Weapon",
            "Bond of a Pyromancer",
            "Magic Weapon",
            "Prayer of a Maiden",
            "Knight's Honor",
            "The Dark Soul",
        ];
        let ds2 = [
            "Self Recollection",
            "This is Dark Souls",
            "Last Giant",
            "Covenant of the Meek",
            "Sinner's Bonfire",
            "Change of Clothes",
            "Clangorous Covenant",
            "Brightstone Bonfire",
            "Iron Keep Bonfire",
            "Gulch Bonfire",
            "Gnawing Covenant",
            "Supreme Weapon",
            "Looking Glass Knight",
            "King's Ring",
            "Ancient Dragon",
            "Covenant of the Fittest",
            "Brilliant Covenant",
            "Reflections on Disembodiment",
            "Protector Covenant",
            "The Heir",
            "Vendrick",
            "Sanguinary Covenant",
            "Abysmal Covenant",
            "Garrulous Miser",
            "Selfless Giver",
            "Holder of the Fort",
            "Gathering of Exiles",
            "Covenant of Ancients",
            "Curious Map",
            "Smith for Life",
            "Lucatiel",
            "Gesture Maestro",
            "Moonlight Greatsword",
            "Master of Hexes",
            "Master of Sorcery",
            "Master of Miracles",
            "Master of Pyromancy",
            "The Dark Soul",
        ];
        let ds3 = [
            "Enkindle",
            "Embrace the Flame",
            "Iudex Gundyr",
            "Vordt of the Boreal Valley",
            "Covenant: Way of Blue",
            "Curse-rotted Greatwood",
            "Covenant: Blue Sentinels",
            "Crystal Sage",
            "Lords of Cinder: Abyss Watchers",
            "Deacons of the Deep",
            "High Lord Wolnir",
            "Covenant: Warrior of Sunlight",
            "Pontiff Sulyvahn",
            "Covenant: Watchdogs of Farron",
            "Dancer of the Boreal Valley",
            "Lord of Cinder: Yhorm the Giant",
            "Lord of Cinder: Aldrich, Devourer of Gods",
            "Old Demon King",
            "Dragonslayer Armour",
            "Oceiros, the Consumed King",
            "Untended Graves",
            "Covenant: Rosaria's Fingers",
            "Lord of Cinder: Lothric, Younger Prince",
            "Champion Gundyr",
            "Supreme Weapon Reinforcement",
            "Archdragon Peak",
            "Ancient Wyvern",
            "The Nameless King",
            "Covenant: Mound-makers",
            "Covenant: Aldrich Faithful",
            "Ultimate Bonfire",
            "Covenant: Blade of the Darkmoon",
            "Ultimate Estus",
            "To Link the First Flame",
            "The End of Fire",
            "The Usurpation of Fire",
            "Master of Infusion",
            "Master of Sorceries",
            "Master of Pyromancies",
            "Master of Miracles",
            "Master of Expression",
            "Master of Rings",
            "The Dark Soul",
        ];
        for (appid, names) in [
            (1245620u32, &er[..]),
            (570940, &ds1[..]),
            (335300, &ds2[..]),
            (374320, &ds3[..]),
        ] {
            for name in names {
                assert!(
                    crate::guides::find(appid, name).is_some(),
                    "no curated guide for {appid} / {name}"
                );
            }
        }
    }

    #[test]
    fn every_missable_has_a_deadline() {
        for g in crate::guides::all_guides() {
            if g.missable {
                assert!(
                    !g.deadline.is_empty(),
                    "missable guide {:?} has no deadline",
                    g.matches
                );
            } else {
                assert!(
                    g.deadline.is_empty(),
                    "non-missable guide {:?} has a deadline",
                    g.matches
                );
            }
        }
    }

    #[test]
    fn every_game_has_primers() {
        for game in crate::games::GAMES {
            assert!(
                !game.mechanics.is_empty(),
                "{} missing mechanics primer",
                game.title
            );
            assert!(
                !game.farming.is_empty(),
                "{} missing farming primer",
                game.title
            );
        }
    }

    #[test]
    fn planner_phases_assigned() {
        for (appid, name, phase) in [
            (1245620, "Age of the Stars", "Multiple runs"),
            (1245620, "Legendary Armaments", "NG"),
            (374320, "Master of Rings", "NG++"),
            (814380, "Shura", "Multiple runs"),
            (1888160, "Alea Iacta Est", "NG++"),
            (570940, "Knight's Honor", "NG++"),
        ] {
            let g = crate::guides::find(appid, name).unwrap();
            assert_eq!(g.phase, phase, "phase for {name}");
        }
    }
}
