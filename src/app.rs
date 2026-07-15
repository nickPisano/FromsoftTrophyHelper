use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

use crate::games::{self, GAMES};
use crate::guides;
use crate::steam::{self, AchRow, GameData, Msg};

const GOLD: egui::Color32 = egui::Color32::from_rgb(214, 181, 106);
const GREEN: egui::Color32 = egui::Color32::from_rgb(120, 200, 130);
const RED: egui::Color32 = egui::Color32::from_rgb(235, 110, 90);
const ORANGE: egui::Color32 = egui::Color32::from_rgb(240, 170, 80);
const BLUE: egui::Color32 = egui::Color32::from_rgb(120, 170, 240);
const PURPLE: egui::Color32 = egui::Color32::from_rgb(190, 140, 240);
const DIM: egui::Color32 = egui::Color32::from_rgb(150, 150, 150);

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
struct Settings {
    api_key: String,
    steam_input: String,
    steam_id: String,
}

enum Loaded {
    Loading,
    Ready(GameData),
    Failed(String),
}

#[derive(PartialEq, Clone, Copy)]
enum StatusFilter {
    All,
    Locked,
    Unlocked,
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

    tx: Sender<Msg>,
    rx: Receiver<Msg>,

    data: HashMap<u32, Loaded>,
    selected_game: usize,
    selected_ach: HashMap<u32, String>,

    search: String,
    status_filter: StatusFilter,
    missable_only: bool,
    sort_by: SortBy,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut visuals = egui::Visuals::dark();
        visuals.selection.bg_fill = egui::Color32::from_rgb(94, 74, 34);
        visuals.hyperlink_color = GOLD;
        cc.egui_ctx.set_visuals(visuals);

        let settings: Settings = cc
            .storage
            .and_then(|s| eframe::get_value(s, "settings"))
            .unwrap_or_default();
        let settings_open = settings.api_key.is_empty() || settings.steam_id.is_empty();

        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            settings,
            settings_open,
            connect_status: None,
            resolving: false,
            tx,
            rx,
            data: HashMap::new(),
            selected_game: 0,
            selected_ach: HashMap::new(),
            search: String::new(),
            status_filter: StatusFilter::All,
            missable_only: false,
            sort_by: SortBy::MostCommon,
        }
    }

    fn connected(&self) -> bool {
        !self.settings.api_key.is_empty() && !self.settings.steam_id.is_empty()
    }

    fn current_appid(&self) -> u32 {
        GAMES[self.selected_game].appid
    }

    fn poll_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Msg::Resolved { result } => {
                    self.resolving = false;
                    match result {
                        Ok(id) => {
                            self.settings.steam_id = id.clone();
                            self.connect_status =
                                Some(Ok(format!("Connected as SteamID {id}")));
                        }
                        Err(e) => self.connect_status = Some(Err(e)),
                    }
                }
                Msg::Game { appid, result } => {
                    let loaded = match result {
                        Ok(gd) => Loaded::Ready(gd),
                        Err(e) => Loaded::Failed(e),
                    };
                    self.data.insert(appid, loaded);
                }
            }
        }
    }

    fn ensure_loaded(&mut self, ctx: &egui::Context) {
        let appid = self.current_appid();
        if self.connected() && !self.data.contains_key(&appid) {
            self.data.insert(appid, Loaded::Loading);
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
                self.connect_status =
                    Some(Err("Enter your SteamID64, custom URL name, or profile URL.".into()));
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
        let appid = self.current_appid();
        self.data.remove(&appid);
        self.ensure_loaded(ctx);
    }

    // ------------------------------------------------------------------ UI

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("top_bar").show(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("\u{2694} FromSoft Trophy Helper")
                        .heading()
                        .color(GOLD),
                );
                ui.separator();

                let prev = self.selected_game;
                egui::ComboBox::from_id_salt("game_select")
                    .selected_text(GAMES[self.selected_game].title)
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        for (i, g) in GAMES.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_game, i, g.title);
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
                            egui::RichText::new(format!(
                                "\u{1F7E2} {}",
                                self.settings.steam_id
                            ))
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
                ui.label(
                    egui::RichText::new(
                        "Your profile's \"Game details\" must be public for progress to show \
                         (Steam > Profile > Edit Profile > Privacy Settings).",
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
        let appid = self.current_appid();
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
                    ui.selectable_value(&mut self.status_filter, StatusFilter::Locked, "\u{1F512} Locked");
                    ui.selectable_value(&mut self.status_filter, StatusFilter::Unlocked, "\u{2714} Unlocked");
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
                    Some(Loaded::Ready(gd)) => {
                        // Progress header
                        let total = gd.achievements.len();
                        let unlocked = gd
                            .achievements
                            .iter()
                            .filter(|a| a.unlocked == Some(true))
                            .count();
                        if gd.player_error.is_none() && total > 0 {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "{unlocked} / {total} unlocked ({:.0}%)",
                                    unlocked as f32 / total as f32 * 100.0
                                ))
                                .strong(),
                            );
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
                                    if ach_list_row(ui, appid, row, selected) {
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
        let appid = self.current_appid();
        let game = &GAMES[self.selected_game];
        let selected: Option<AchRow> = match (self.data.get(&appid), self.selected_ach.get(&appid))
        {
            (Some(Loaded::Ready(gd)), Some(sel)) => gd
                .achievements
                .iter()
                .find(|a| &a.api_name == sel)
                .cloned(),
            _ => None,
        };

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                match selected {
                    Some(row) => ach_details(ui, game, &row),
                    None => game_overview(ui, game, self.connected(), &mut self.settings_open),
                }
            });
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll_messages();
        self.ensure_loaded(&ctx);
        self.top_bar(ui);
        self.settings_window(&ctx);
        self.left_panel(ui);
        self.central_panel(ui);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "settings", &self.settings);
    }
}

// ---------------------------------------------------------------------- UI helpers

/// Returns true when the row was clicked.
fn ach_list_row(ui: &mut egui::Ui, appid: u32, row: &AchRow, selected: bool) -> bool {
    let guide = guides::find(appid, &row.name);
    let unlocked = row.unlocked == Some(true);

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
                            ui.label(
                                egui::RichText::new(format!("{p:.1}%"))
                                    .small()
                                    .color(DIM),
                            );
                            ui.label(egui::RichText::new(rarity).small().color(color));
                        }
                    });
                });
            });
        })
        .response;

    let resp = resp.interact(egui::Sense::click());
    resp.clicked()
}

fn ach_details(ui: &mut egui::Ui, game: &games::Game, row: &AchRow) {
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
                ui.label(egui::RichText::new(&row.name).heading().color(GOLD));
                if guide.is_some_and(|g| g.missable) {
                    ui.label(egui::RichText::new("\u{26A0} MISSABLE").strong().color(ORANGE));
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
                    ui.label(
                        egui::RichText::new(format!("\u{2714} Unlocked{when}")).color(GREEN),
                    );
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
            ui.label(egui::RichText::new(rarity).color(color));
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

fn game_overview(
    ui: &mut egui::Ui,
    game: &games::Game,
    connected: bool,
    settings_open: &mut bool,
) {
    ui.add_space(12.0);
    ui.label(egui::RichText::new(game.title).heading().color(GOLD));
    ui.add_space(4.0);
    if connected {
        ui.label("Select an achievement on the left to see rules, locations, and guides.");
    } else {
        ui.label("Connect your Steam account to scan your achievement progress.");
        if ui.button("\u{2699} Open Settings").clicked() {
            *settings_open = true;
        }
    }
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);
    game_notes_and_resources(ui, game);
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
            }) && (!missable_only
                || guides::find(appid, &a.name).is_some_and(|g| g.missable))
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
        SortBy::Name => rows.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortBy::RecentUnlocks => rows.sort_by(|a, b| {
            b.unlock_time
                .unwrap_or(0)
                .cmp(&a.unlock_time.unwrap_or(0))
        }),
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
        use crate::steam::{parse_steam_input, SteamInput};
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
        ] {
            let g = crate::guides::find(appid, name)
                .unwrap_or_else(|| panic!("no guide matched {appid} / {name}"));
            assert_eq!(g.missable, missable, "missable flag for {name}");
        }
        assert!(crate::guides::find(1245620, "Roundtable Hold").is_none());
    }
}
