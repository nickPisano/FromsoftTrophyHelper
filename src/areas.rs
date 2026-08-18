//! Maps checklist items to the in-game area you actually travel to, so the
//! app can answer "I'm in Anor Londo — what do I still need here?" across
//! every collect-them-all achievement at once.
//!
//! Resolution is keyword based against the item text (and its `## ` section
//! header as a fallback), so the curated checklists need no extra authoring.

pub struct Area {
    pub name: &'static str,
    /// Definitive location signals: place names, and bosses/NPCs that can
    /// only mean one place. Checked before `weak`.
    pub strong: &'static [&'static str],
    /// Softer signals — vendors and enemies that move or appear in several
    /// places. Only consulted when nothing strong matched.
    pub weak: &'static [&'static str],
    /// Wiki page for the area, which carries its map.
    pub map_url: &'static str,
    /// Position on the drawn world map, normalised 0..1 with y increasing
    /// downward. Our own layout — it echoes how the world actually connects
    /// without reproducing anyone's artwork.
    pub pos: (f32, f32),
}

/// How the areas actually connect, by name so that reordering the table
/// above can't silently rewire the graph.
pub fn links_for(appid: u32) -> &'static [(&'static str, &'static str)] {
    match appid {
        570940 | 211420 => DS1_LINKS,
        _ => &[],
    }
}

pub fn index_of(appid: u32, name: &str) -> Option<usize> {
    areas_for(appid).iter().position(|a| a.name == name)
}

static DS1_LINKS: &[(&str, &str)] = &[
    ("Northern Undead Asylum", "Firelink Shrine"),
    ("Firelink Shrine", "Undead Burg"),
    ("Firelink Shrine", "New Londo Ruins"),
    ("Firelink Shrine", "The Catacombs"),
    ("Firelink Shrine", "Kiln of the First Flame"),
    ("Undead Burg", "Undead Parish"),
    ("Undead Burg", "The Depths"),
    ("Undead Parish", "Darkroot Garden"),
    ("Undead Parish", "Sen's Fortress"),
    ("Darkroot Garden", "Darkroot Basin"),
    ("Darkroot Basin", "Valley of Drakes"),
    ("Sen's Fortress", "Anor Londo"),
    ("Anor Londo", "Painted World of Ariamis"),
    ("Anor Londo", "The Duke's Archives"),
    ("The Duke's Archives", "Crystal Cave"),
    ("The Depths", "Blighttown"),
    ("Blighttown", "Quelaag's Domain"),
    ("Blighttown", "The Great Hollow"),
    ("Blighttown", "Valley of Drakes"),
    ("The Great Hollow", "Ash Lake"),
    ("Quelaag's Domain", "Demon Ruins"),
    ("Demon Ruins", "Lost Izalith"),
    ("New Londo Ruins", "Valley of Drakes"),
    ("The Catacombs", "Tomb of the Giants"),
];

/// Areas for a game, in rough progression order. Empty when the game has no
/// area data yet (only Dark Souls 1 is mapped so far).
pub fn areas_for(appid: u32) -> &'static [Area] {
    match appid {
        // Remastered and Prepare to Die Edition share a world.
        570940 | 211420 => DS1_AREAS,
        _ => &[],
    }
}

/// Index into [`areas_for`] of the area an item belongs to. `section` is the
/// item's `## ` header text (without the marker), used when the item itself
/// names no location — e.g. boss-soul weapons, which you actually collect at
/// the Giant Blacksmith.
pub fn resolve(appid: u32, item: &str, section: &str) -> Option<usize> {
    let areas = areas_for(appid);
    if areas.is_empty() {
        return None;
    }
    let item = item.to_lowercase();
    let section = section.to_lowercase();
    for hay in [&item, &section] {
        for strong_pass in [true, false] {
            for (i, area) in areas.iter().enumerate() {
                let keys = if strong_pass { area.strong } else { area.weak };
                if keys.iter().any(|k| hay.contains(*k)) {
                    return Some(i);
                }
            }
        }
    }
    None
}

/// Keyword notes:
/// - Keep place names full ("darkroot garden", never "darkroot") so sibling
///   areas can't shadow each other.
/// - Never use bare "gwyn" — it is a substring of Gwyndolin and Gwynevere.
/// - Boss names are deliberately absent where a boss soul is *forged*
///   somewhere else; those items fall through to the section header and land
///   on the Giant Blacksmith in Anor Londo, which is where you actually go.
static DS1_AREAS: &[Area] = &[
    Area {
        name: "Northern Undead Asylum",
        strong: &["undead asylum"],
        weak: &["oscar"],
        map_url: "https://darksouls.wiki.fextralife.com/Northern+Undead+Asylum",
        pos: (0.07, 0.09),
    },
    Area {
        name: "Firelink Shrine",
        strong: &["firelink"],
        weak: &["griggs", "laurentius", "petrus", "siegmeyer", "logan"],
        map_url: "https://darksouls.wiki.fextralife.com/Firelink+Shrine",
        pos: (0.53, 0.52),
    },
    Area {
        name: "Undead Burg",
        strong: &["undead burg"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Undead+Burg",
        pos: (0.40, 0.45),
    },
    Area {
        name: "Undead Parish",
        strong: &["undead parish", "sunlight altar"],
        weak: &["reah", "oswald", "warrior of sunlight"],
        map_url: "https://darksouls.wiki.fextralife.com/Undead+Parish",
        pos: (0.34, 0.35),
    },
    Area {
        name: "Darkroot Garden",
        // "defeat sif" is safe where a bare "sif" is not — the boss-soul
        // weapons say "Sif soul" and must route to the Giant Blacksmith.
        strong: &["darkroot garden", "defeat sif"],
        weak: &["giant stone knights", "shiva"],
        map_url: "https://darksouls.wiki.fextralife.com/Darkroot+Garden",
        pos: (0.17, 0.30),
    },
    Area {
        name: "Darkroot Basin",
        strong: &["darkroot basin"],
        weak: &["dusk"],
        map_url: "https://darksouls.wiki.fextralife.com/Darkroot+Basin",
        pos: (0.09, 0.41),
    },
    Area {
        name: "The Depths",
        strong: &["depths", "gaping dragon"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Depths",
        pos: (0.39, 0.59),
    },
    Area {
        name: "Blighttown",
        strong: &["blighttown"],
        weak: &["quelana"],
        map_url: "https://darksouls.wiki.fextralife.com/Blighttown",
        pos: (0.33, 0.72),
    },
    Area {
        name: "Quelaag's Domain",
        strong: &["quelaag's domain"],
        weak: &["eingyi", "chaos servant"],
        map_url: "https://darksouls.wiki.fextralife.com/Quelaag's+Domain",
        pos: (0.47, 0.80),
    },
    Area {
        name: "Sen's Fortress",
        strong: &["sen's fortress"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Sen's+Fortress",
        pos: (0.50, 0.30),
    },
    Area {
        name: "Anor Londo",
        strong: &["anor londo", "darkmoon tomb"],
        weak: &[
            "giant blacksmith",
            "silver knight",
            "sentinels",
            "gwyndolin",
            "gwynevere",
            "blade of the darkmoon",
        ],
        map_url: "https://darksouls.wiki.fextralife.com/Anor+Londo",
        pos: (0.50, 0.18),
    },
    Area {
        name: "Painted World of Ariamis",
        strong: &["painted world"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Painted+World+of+Ariamis",
        pos: (0.30, 0.11),
    },
    Area {
        name: "The Catacombs",
        strong: &["catacombs"],
        weak: &["gravelord"],
        map_url: "https://darksouls.wiki.fextralife.com/The+Catacombs",
        pos: (0.74, 0.55),
    },
    Area {
        name: "Tomb of the Giants",
        strong: &["tomb of the giants"],
        weak: &["patches"],
        map_url: "https://darksouls.wiki.fextralife.com/Tomb+of+Giants",
        pos: (0.86, 0.67),
    },
    Area {
        name: "New Londo Ruins",
        strong: &["new londo"],
        weak: &["ingward", "darkwraith", "four kings", "kaathe"],
        map_url: "https://darksouls.wiki.fextralife.com/New+Londo+Ruins",
        pos: (0.63, 0.65),
    },
    Area {
        name: "Valley of Drakes",
        strong: &["valley of drakes"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Valley+of+Drakes",
        pos: (0.25, 0.60),
    },
    Area {
        name: "The Great Hollow",
        strong: &["great hollow"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/The+Great+Hollow",
        pos: (0.17, 0.84),
    },
    Area {
        name: "Ash Lake",
        strong: &["ash lake"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Ash+Lake",
        pos: (0.09, 0.94),
    },
    Area {
        name: "The Duke's Archives",
        strong: &["archives", "seath"],
        weak: &["pisaca"],
        map_url: "https://darksouls.wiki.fextralife.com/The+Duke's+Archives",
        pos: (0.74, 0.14),
    },
    Area {
        name: "Crystal Cave",
        strong: &["crystal cave"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Crystal+Cave",
        pos: (0.88, 0.07),
    },
    Area {
        name: "Demon Ruins",
        strong: &["demon ruins"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Demon+Ruins",
        pos: (0.58, 0.88),
    },
    Area {
        name: "Lost Izalith",
        strong: &["lost izalith"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Lost+Izalith",
        pos: (0.70, 0.94),
    },
    Area {
        name: "Kiln of the First Flame",
        strong: &["kiln"],
        weak: &["black knight"],
        map_url: "https://darksouls.wiki.fextralife.com/Kiln+of+the+First+Flame",
        pos: (0.50, 0.03),
    },
];

#[cfg(test)]
mod tests {
    use super::{areas_for, resolve};

    const DS1: u32 = 570940;

    fn area_of(item: &str, section: &str) -> &'static str {
        resolve(DS1, item, section)
            .map(|i| areas_for(DS1)[i].name)
            .unwrap_or("<unrouted>")
    }

    #[test]
    fn routes_to_where_you_actually_go() {
        // Boss souls are forged at the Giant Blacksmith, not where the boss died.
        assert_eq!(
            area_of(
                "Quelaag's Furysword (Quelaag soul + curved sword)",
                "Boss-soul weapons (Giant Blacksmith)"
            ),
            "Anor Londo"
        );
        // An explicit place beats the NPC named alongside it.
        assert_eq!(
            area_of(
                "Replenishment (Reah, at the Tomb of the Giants)",
                "Petrus & Reah"
            ),
            "Tomb of the Giants"
        );
        // A vendor with no place named falls back to where they sell.
        assert_eq!(
            area_of("Great Heal (Reah)", "Petrus & Reah"),
            "Undead Parish"
        );
        assert_eq!(area_of("Aural Decoy (Griggs)", ""), "Firelink Shrine");
        // Black Knights are farmed reliably at the Kiln.
        assert_eq!(
            area_of("Black Knight Halberd (halberd Black Knights)", ""),
            "Kiln of the First Flame"
        );
        // Sibling areas must not shadow each other.
        assert_eq!(
            area_of(
                "Stone Greatshield (Giant Stone Knights, Darkroot Garden)",
                ""
            ),
            "Darkroot Garden"
        );
        assert_eq!(
            area_of("Hidden Body (Dusk)", "Dusk of Oolacile"),
            "Darkroot Basin"
        );
        // "Sif soul" must NOT be dragged to Darkroot Garden by the quest-step keyword.
        assert_eq!(
            area_of(
                "Greatshield of Artorias (Sif soul + shield)",
                "Boss-soul weapons (Giant Blacksmith)"
            ),
            "Anor Londo"
        );
        assert_eq!(
            area_of("Defeat Sif for the Covenant of Artorias ring", "Steps"),
            "Darkroot Garden"
        );
    }

    #[test]
    fn ds1_checklists_are_almost_fully_routed() {
        let mut total = 0;
        let mut unrouted = 0;
        for g in crate::guides::all_guides() {
            if g.appid != DS1 {
                continue;
            }
            let mut section = "";
            for item in g.checklist {
                if let Some(h) = item.strip_prefix("## ") {
                    section = h;
                    continue;
                }
                total += 1;
                if resolve(DS1, item, section).is_none() {
                    unrouted += 1;
                }
            }
        }
        assert!(
            total > 100,
            "expected the DS1 checklists to be substantial, got {total}"
        );
        assert!(
            unrouted <= 2,
            "{unrouted}/{total} DS1 checklist items could not be routed to an area"
        );
    }

    #[test]
    fn ptde_shares_the_ds1_world() {
        assert_eq!(areas_for(211420).len(), areas_for(DS1).len());
        assert_eq!(
            area_of_in(211420, "Silver Knight Spear (Anor Londo)"),
            "Anor Londo"
        );
    }

    #[test]
    fn unmapped_games_resolve_to_nothing() {
        // Elden Ring has no area table yet; resolve must stay quiet, not guess.
        assert!(areas_for(1245620).is_empty());
        assert_eq!(resolve(1245620, "Bolt of Gransax (Leyndell)", ""), None);
    }

    fn area_of_in(appid: u32, item: &str) -> &'static str {
        resolve(appid, item, "")
            .map(|i| areas_for(appid)[i].name)
            .unwrap_or("<unrouted>")
    }

    #[test]
    fn world_map_layout_is_sane() {
        let areas = areas_for(DS1);
        for a in areas {
            let (x, y) = a.pos;
            assert!(
                (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y),
                "{} sits outside the map at {:?}",
                a.name,
                a.pos
            );
        }
        // Two areas on the same spot means a copy-pasted coordinate.
        for (i, a) in areas.iter().enumerate() {
            for b in &areas[i + 1..] {
                let d = (a.pos.0 - b.pos.0).abs() + (a.pos.1 - b.pos.1).abs();
                assert!(d > 0.02, "{} and {} overlap on the map", a.name, b.name);
            }
        }
    }

    #[test]
    fn every_link_endpoint_exists() {
        let links = super::links_for(DS1);
        assert!(!links.is_empty());
        for (a, b) in links {
            assert!(
                super::index_of(DS1, a).is_some(),
                "unknown area in link: {a}"
            );
            assert!(
                super::index_of(DS1, b).is_some(),
                "unknown area in link: {b}"
            );
            assert_ne!(a, b, "self-link on {a}");
        }
        assert!(super::links_for(1245620).is_empty());
    }

    #[test]
    fn world_graph_is_connected() {
        // Every area should be reachable from Firelink, or the map will show
        // orphan nodes floating with no route to them.
        let areas = areas_for(DS1);
        let start = super::index_of(DS1, "Firelink Shrine").unwrap();
        let mut seen = vec![false; areas.len()];
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(i) = stack.pop() {
            for (a, b) in super::links_for(DS1) {
                let (ia, ib) = (
                    super::index_of(DS1, a).unwrap(),
                    super::index_of(DS1, b).unwrap(),
                );
                for (from, to) in [(ia, ib), (ib, ia)] {
                    if from == i && !seen[to] {
                        seen[to] = true;
                        stack.push(to);
                    }
                }
            }
        }
        for (i, ok) in seen.iter().enumerate() {
            assert!(*ok, "{} is unreachable on the world map", areas[i].name);
        }
    }

    #[test]
    fn every_area_has_a_map_link() {
        for a in areas_for(DS1) {
            assert!(
                a.map_url.starts_with("https://"),
                "{} has a non-https map url",
                a.name
            );
            assert!(!a.strong.is_empty(), "{} has no strong keywords", a.name);
        }
    }
}
