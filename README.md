# FromSoft Trophy Helper

A desktop app (Rust + [egui](https://github.com/emilk/egui)) for hunting 100% achievement completion
in FromSoftware's Steam catalog. It scans your live achievement progress through the official
Steam Web API and pairs every achievement with the help you need to finish it:

- **Your progress, scanned from Steam** — unlocked / locked state and unlock dates, per game.
- **Global rarity** — the percentage of players who have each achievement, shown as a bar and a
  rarity tag (Common → Ultra rare). The default sort puts the most-earned achievements at the top.
- **Rules & missable warnings** — every achievement in Elden Ring and Dark Souls 1/2/3 has a
  hand-curated guide (how to reach it, gate requirements, a strategy note), with "read this
  before you play on" rules for anything that can be lost (points of no return, ending
  lockouts, NPC quest deadlines). Questline achievements get ordered, tickable steps.
- **Suggested next** — each game page surfaces the top locked achievements by global unlock
  rate, so you always know what to go do right now.
- **Locations** — where you have to go for the multi-location collectathons.
- **By Area (Dark Souls 1)** — the routing view: every item you still need, grouped by the
  area you go to for it, pooled across all four collectathons at once. Standing in Anor Londo
  shows the Silver Knight weapons, the illusory-wall chests, every boss-soul forge, and the
  Darkmoon spells on one screen — tick them off in place, with a link to that area's map and
  guide on the wiki.
- **Live world map** — Lordran drawn as a connected node map, with each area sized and lit by
  how much you still have to collect there. Click an area to focus the list on it; nodes shrink
  and go green as you tick items off. Drawn by the app itself, so it works offline and ships no
  third-party artwork.
- **Resources** — hand-picked wiki checklists and guides per achievement, plus auto-generated
  YouTube / Google guide searches for every achievement in every game.
- **Filters** — search, locked/unlocked, missable-only, and sorting by rarity, name, or most
  recent unlocks.
- **Overview dashboard** — every game's completion at a glance, sorted by how close you are
  to 100%.
- **Sub-checklists** — tick off every individual item inside collect-them-all achievements:
  all 47 Knight's Honor weapons, every DS1/DS3 spell (with vendor/tome grouping and DLC
  exclusions), legendary armaments, quest steps. Grouped headers, progress bar, quick
  filter, and check-all/clear-all; progress is saved locally.
- **Zero-friction setup** — your SteamID is auto-detected from the local Steam client;
  you only paste an API key.
- **Instant startup** — the last scan is cached locally, so the app opens with your data
  immediately (and works offline); stale games re-scan in the background.
- **Live unlock notifications** — with auto-rescan on, a desktop notification and in-app
  toast fire when a new achievement pops mid-session.
- **Personal notes** — a private notes box on every achievement, saved locally.
- **Playthrough planner** — a per-game roadmap grouping achievements by the earliest
  run they land in (NG → NG+ → NG++ → multiple-run endings).
- **FromSoft-flavored UI** — custom dark/gold theme, serif display font (Cinzel, OFL),
  and Steam artwork on the dashboard and game pages.

## Supported games

| Game | Steam App ID |
|---|---|
| ELDEN RING | 1245620 |
| DARK SOULS III | 374320 |
| Sekiro: Shadows Die Twice | 814380 |
| ARMORED CORE VI: FIRES OF RUBICON | 1888160 |
| DARK SOULS: REMASTERED | 570940 |
| DARK SOULS: Prepare to Die Edition | 211420 |
| DARK SOULS II: Scholar of the First Sin | 335300 |
| DARK SOULS II (2014) | 236430 |

## Setup

1. **Get a free Steam Web API key** at <https://steamcommunity.com/dev/apikey>
   (any domain name works, e.g. `localhost`). The key is stored only on your computer.
2. **Make your game details public**: Steam → Profile → Edit Profile → Privacy Settings →
   set *Game details* to **Public**. Without this, the app still shows the full achievement
   lists and global rarity, but not your personal progress.
3. Run the app and open **Settings** — your Steam account is usually pre-detected from the
   local Steam client, so just paste the API key and hit **Connect & scan**. (You can also
   enter any SteamID64, custom URL name, or full profile URL manually.)

## Build & run

Requires a Rust toolchain (<https://rustup.rs>).

```sh
cargo run --release
```

## Notes

- Achievement names, descriptions, icons, and global percentages come live from Steam, so the
  app stays correct if Valve updates a game's achievement list.
- Curated guide data lives in [src/guides.rs](src/guides.rs) — matched against achievement
  display names, so contributions are just new entries in that table.
- Steam has no public API for *other* launchers; only Steam copies of the games are supported.
