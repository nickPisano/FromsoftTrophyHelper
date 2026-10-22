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
    /// Notable spots inside the area, for the drilled-in area map. Ordered
    /// most-specific first: the first spot whose keyword matches wins, so a
    /// broad catch-all ("painted world") must come after the precise ones
    /// ("annex", "undead dragon"). Items matching nothing are listed as
    /// "Elsewhere in <area>" rather than guessed at.
    pub spots: &'static [Spot],
}

pub struct Spot {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    /// Position on the area map, normalised 0..1.
    pub pos: (f32, f32),
}

/// Which spot inside `area` an item belongs to, by the same item-then-section
/// keyword scheme [`resolve`] uses for areas.
pub fn resolve_spot(area: &Area, item: &str, section: &str) -> Option<usize> {
    let item = item.to_lowercase();
    let section = section.to_lowercase();
    for hay in [&item, &section] {
        for (i, spot) in area.spots.iter().enumerate() {
            if spot.keywords.iter().any(|k| hay.contains(*k)) {
                return Some(i);
            }
        }
    }
    None
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

// Spots inside each area. Positions are laid out to keep the cards clear of
// each other and of the panel edges; they read as a diagram of the area, not
// a surveyed floor plan.

static SPOTS_ASYLUM: &[Spot] = &[Spot {
    name: "Oscar's cell",
    keywords: &["oscar"],
    pos: (0.50, 0.50),
}];

static SPOTS_FIRELINK: &[Spot] = &[
    Spot {
        name: "Petrus of Thorolund",
        keywords: &["petrus"],
        pos: (0.22, 0.20),
    },
    Spot {
        name: "Griggs of Vinheim",
        keywords: &["griggs"],
        pos: (0.74, 0.20),
    },
    Spot {
        name: "Laurentius",
        keywords: &["laurentius"],
        pos: (0.24, 0.55),
    },
    Spot {
        name: "Siegmeyer (stairs)",
        keywords: &["siegmeyer"],
        pos: (0.50, 0.85),
    },
];

static SPOTS_UNDEAD_BURG: &[Spot] = &[Spot {
    name: "Hellkite bridge",
    keywords: &["hellkite", "undead burg bridge"],
    pos: (0.50, 0.50),
}];

static SPOTS_UNDEAD_PARISH: &[Spot] = &[
    Spot {
        name: "Reah (church)",
        keywords: &["reah"],
        pos: (0.24, 0.26),
    },
    Spot {
        name: "Sunlight Altar",
        keywords: &["sunlight altar", "warrior of sunlight"],
        pos: (0.74, 0.30),
    },
    Spot {
        name: "Oswald (belfry)",
        keywords: &["oswald"],
        pos: (0.46, 0.74),
    },
];

static SPOTS_DARKROOT_GARDEN: &[Spot] = &[
    Spot {
        name: "Giant Stone Knights",
        keywords: &["giant stone knights"],
        pos: (0.30, 0.34),
    },
    Spot {
        name: "Sif's arena",
        keywords: &["defeat sif"],
        pos: (0.68, 0.68),
    },
];

static SPOTS_DARKROOT_BASIN: &[Spot] = &[Spot {
    name: "Dusk (Crystal Golem)",
    keywords: &["dusk"],
    pos: (0.50, 0.50),
}];

static SPOTS_DEPTHS: &[Spot] = &[
    Spot {
        name: "Channelers",
        keywords: &["channeler"],
        pos: (0.30, 0.32),
    },
    Spot {
        name: "Gaping Dragon arena",
        keywords: &["gaping dragon"],
        pos: (0.68, 0.68),
    },
];

static SPOTS_BLIGHTTOWN: &[Spot] = &[
    Spot {
        name: "Quelana (swamp)",
        keywords: &["quelana"],
        pos: (0.30, 0.66),
    },
    Spot {
        name: "Swamp floor",
        keywords: &["blighttown"],
        pos: (0.68, 0.32),
    },
];

static SPOTS_QUELAAGS: &[Spot] = &[
    Spot {
        name: "Eingyi (illusory wall)",
        keywords: &["eingyi"],
        pos: (0.30, 0.34),
    },
    Spot {
        name: "Chaos Servant covenant",
        keywords: &["chaos servant"],
        pos: (0.68, 0.66),
    },
];

static SPOTS_SENS: &[Spot] = &[Spot {
    name: "Upper-level chest",
    keywords: &["sen's fortress chest", "chest"],
    pos: (0.50, 0.50),
}];

static SPOTS_ANOR_LONDO: &[Spot] = &[
    Spot {
        name: "Rafters & buttresses",
        keywords: &["silver knight"],
        pos: (0.18, 0.30),
    },
    Spot {
        name: "Sentinel hall",
        keywords: &["sentinels"],
        pos: (0.50, 0.12),
    },
    Spot {
        name: "Giant Blacksmith",
        keywords: &["giant blacksmith"],
        pos: (0.82, 0.30),
    },
    Spot {
        name: "Broken window ledge",
        keywords: &["broken window"],
        pos: (0.50, 0.48),
    },
    Spot {
        name: "By the painting",
        keywords: &["next to the painting"],
        pos: (0.18, 0.68),
    },
    Spot {
        name: "Illusory-wall chest room",
        keywords: &["illusory-wall"],
        pos: (0.82, 0.68),
    },
    Spot {
        name: "Darkmoon Tomb",
        keywords: &["darkmoon tomb", "blade of the darkmoon"],
        pos: (0.50, 0.88),
    },
];

static SPOTS_PAINTED_WORLD: &[Spot] = &[
    Spot {
        name: "Priscilla's tower",
        keywords: &["priscilla"],
        pos: (0.50, 0.16),
    },
    Spot {
        name: "Annex (Annex Key door)",
        keywords: &["annex"],
        pos: (0.20, 0.46),
    },
    Spot {
        name: "By the Undead Dragon",
        keywords: &["undead dragon"],
        pos: (0.80, 0.46),
    },
    Spot {
        name: "Township grounds",
        keywords: &["painted world"],
        pos: (0.50, 0.80),
    },
];

static SPOTS_CATACOMBS: &[Spot] = &[
    Spot {
        name: "Gravelord coffin",
        keywords: &["gravelord"],
        pos: (0.32, 0.34),
    },
    Spot {
        name: "Catacombs treasure",
        keywords: &["catacombs"],
        pos: (0.68, 0.66),
    },
];

static SPOTS_TOMB: &[Spot] = &[
    Spot {
        name: "Patches",
        keywords: &["patches"],
        pos: (0.24, 0.26),
    },
    Spot {
        name: "Paladin Leeroy",
        keywords: &["leeroy"],
        pos: (0.74, 0.28),
    },
    Spot {
        name: "Reah's rescue",
        keywords: &["reah"],
        pos: (0.24, 0.70),
    },
    Spot {
        name: "Tomb floor",
        keywords: &["tomb of the giants"],
        pos: (0.74, 0.72),
    },
];

static SPOTS_NEW_LONDO: &[Spot] = &[
    Spot {
        name: "Ingward's tower",
        keywords: &["ingward", "key to the seal"],
        pos: (0.24, 0.28),
    },
    Spot {
        name: "Kaathe & the Darkwraiths",
        keywords: &["kaathe", "darkwraith"],
        pos: (0.76, 0.34),
    },
    Spot {
        name: "The Abyss (Four Kings)",
        keywords: &["abyss", "four kings"],
        pos: (0.50, 0.76),
    },
];

static SPOTS_VALLEY_OF_DRAKES: &[Spot] = &[Spot {
    name: "By the Undead Dragon",
    keywords: &["undead dragon"],
    pos: (0.50, 0.50),
}];

static SPOTS_ASH_LAKE: &[Spot] = &[
    Spot {
        name: "The Stone Dragon",
        keywords: &["stone dragon"],
        pos: (0.32, 0.36),
    },
    Spot {
        name: "Ash Lake shore",
        keywords: &["ash lake"],
        pos: (0.68, 0.66),
    },
];

static SPOTS_DUKES_ARCHIVES: &[Spot] = &[
    Spot {
        name: "Seath's arena",
        keywords: &["seath"],
        pos: (0.74, 0.26),
    },
    Spot {
        name: "Logan's cell",
        keywords: &["logan"],
        pos: (0.26, 0.28),
    },
    Spot {
        name: "Pisaca cages",
        keywords: &["pisaca"],
        pos: (0.26, 0.70),
    },
    Spot {
        name: "Archives interior",
        keywords: &["archives"],
        pos: (0.74, 0.72),
    },
];

static SPOTS_LOST_IZALITH: &[Spot] = &[Spot {
    name: "Izalith ruins",
    keywords: &["lost izalith"],
    pos: (0.50, 0.50),
}];

static SPOTS_KILN: &[Spot] = &[Spot {
    name: "Respawning Black Knights",
    keywords: &["black knight"],
    pos: (0.50, 0.50),
}];

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
        spots: SPOTS_ASYLUM,
    },
    Area {
        name: "Firelink Shrine",
        strong: &["firelink"],
        weak: &["griggs", "laurentius", "petrus", "siegmeyer", "logan"],
        map_url: "https://darksouls.wiki.fextralife.com/Firelink+Shrine",
        pos: (0.53, 0.52),
        spots: SPOTS_FIRELINK,
    },
    Area {
        name: "Undead Burg",
        strong: &["undead burg"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Undead+Burg",
        pos: (0.42, 0.48),
        spots: SPOTS_UNDEAD_BURG,
    },
    Area {
        name: "Undead Parish",
        strong: &["undead parish", "sunlight altar"],
        weak: &["reah", "oswald", "warrior of sunlight"],
        map_url: "https://darksouls.wiki.fextralife.com/Undead+Parish",
        pos: (0.32, 0.34),
        spots: SPOTS_UNDEAD_PARISH,
    },
    Area {
        name: "Darkroot Garden",
        // "defeat sif" is safe where a bare "sif" is not — the boss-soul
        // weapons say "Sif soul" and must route to the Giant Blacksmith.
        strong: &["darkroot garden", "defeat sif"],
        weak: &["giant stone knights", "shiva"],
        map_url: "https://darksouls.wiki.fextralife.com/Darkroot+Garden",
        pos: (0.17, 0.30),
        spots: SPOTS_DARKROOT_GARDEN,
    },
    Area {
        name: "Darkroot Basin",
        strong: &["darkroot basin"],
        weak: &["dusk"],
        map_url: "https://darksouls.wiki.fextralife.com/Darkroot+Basin",
        pos: (0.07, 0.44),
        spots: SPOTS_DARKROOT_BASIN,
    },
    Area {
        name: "The Depths",
        strong: &["depths", "gaping dragon"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Depths",
        pos: (0.34, 0.61),
        spots: SPOTS_DEPTHS,
    },
    Area {
        name: "Blighttown",
        strong: &["blighttown"],
        weak: &["quelana"],
        map_url: "https://darksouls.wiki.fextralife.com/Blighttown",
        pos: (0.28, 0.77),
        spots: SPOTS_BLIGHTTOWN,
    },
    Area {
        name: "Quelaag's Domain",
        strong: &["quelaag's domain"],
        weak: &["eingyi", "chaos servant"],
        map_url: "https://darksouls.wiki.fextralife.com/Quelaag's+Domain",
        pos: (0.47, 0.80),
        spots: SPOTS_QUELAAGS,
    },
    Area {
        name: "Sen's Fortress",
        strong: &["sen's fortress"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Sen's+Fortress",
        pos: (0.50, 0.34),
        spots: SPOTS_SENS,
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
        spots: SPOTS_ANOR_LONDO,
    },
    Area {
        name: "Painted World of Ariamis",
        strong: &["painted world"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Painted+World+of+Ariamis",
        pos: (0.30, 0.11),
        spots: SPOTS_PAINTED_WORLD,
    },
    Area {
        name: "The Catacombs",
        strong: &["catacombs"],
        weak: &["gravelord"],
        map_url: "https://darksouls.wiki.fextralife.com/The+Catacombs",
        pos: (0.74, 0.55),
        spots: SPOTS_CATACOMBS,
    },
    Area {
        name: "Tomb of the Giants",
        strong: &["tomb of the giants"],
        weak: &["patches"],
        map_url: "https://darksouls.wiki.fextralife.com/Tomb+of+Giants",
        pos: (0.86, 0.67),
        spots: SPOTS_TOMB,
    },
    Area {
        name: "New Londo Ruins",
        strong: &["new londo"],
        weak: &["ingward", "darkwraith", "four kings", "kaathe"],
        map_url: "https://darksouls.wiki.fextralife.com/New+Londo+Ruins",
        pos: (0.63, 0.65),
        spots: SPOTS_NEW_LONDO,
    },
    Area {
        name: "Valley of Drakes",
        strong: &["valley of drakes"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Valley+of+Drakes",
        pos: (0.16, 0.62),
        spots: SPOTS_VALLEY_OF_DRAKES,
    },
    Area {
        name: "The Great Hollow",
        strong: &["great hollow"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/The+Great+Hollow",
        pos: (0.17, 0.84),
        spots: &[],
    },
    Area {
        name: "Ash Lake",
        strong: &["ash lake"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Ash+Lake",
        pos: (0.09, 0.94),
        spots: SPOTS_ASH_LAKE,
    },
    Area {
        name: "The Duke's Archives",
        strong: &["archives", "seath"],
        weak: &["pisaca"],
        map_url: "https://darksouls.wiki.fextralife.com/The+Duke's+Archives",
        pos: (0.74, 0.14),
        spots: SPOTS_DUKES_ARCHIVES,
    },
    Area {
        name: "Crystal Cave",
        strong: &["crystal cave"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Crystal+Cave",
        pos: (0.88, 0.07),
        spots: &[],
    },
    Area {
        name: "Demon Ruins",
        strong: &["demon ruins"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Demon+Ruins",
        pos: (0.58, 0.88),
        spots: &[],
    },
    Area {
        name: "Lost Izalith",
        strong: &["lost izalith"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Lost+Izalith",
        pos: (0.70, 0.94),
        spots: SPOTS_LOST_IZALITH,
    },
    Area {
        name: "Kiln of the First Flame",
        strong: &["kiln"],
        weak: &["black knight"],
        map_url: "https://darksouls.wiki.fextralife.com/Kiln+of+the+First+Flame",
        pos: (0.50, 0.02),
        spots: SPOTS_KILN,
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

    fn spot_of(area_name: &str, item: &str, section: &str) -> &'static str {
        let area = &areas_for(DS1)[super::index_of(DS1, area_name).unwrap()];
        super::resolve_spot(area, item, section)
            .map(|s| area.spots[s].name)
            .unwrap_or("<elsewhere>")
    }

    #[test]
    fn spots_resolve_specific_before_general() {
        // The precise spot must beat the area-wide catch-all in the same text.
        assert_eq!(
            spot_of(
                "Anor Londo",
                "Havel's Greatshield (Anor Londo, illusory-wall chest)",
                ""
            ),
            "Illusory-wall chest room"
        );
        assert_eq!(
            spot_of(
                "Painted World of Ariamis",
                "Velka's Rapier (Painted World, behind the Annex Key door)",
                ""
            ),
            "Annex (Annex Key door)"
        );
        assert_eq!(
            spot_of(
                "Tomb of the Giants",
                "Replenishment (Reah, at the Tomb of the Giants)",
                ""
            ),
            "Reah's rescue"
        );
        // Hollow Logan is fought in Seath's arena, not his cell.
        assert_eq!(
            spot_of(
                "The Duke's Archives",
                "White Dragon Breath (defeat hollow Logan in Seath's arena)",
                ""
            ),
            "Seath's arena"
        );
        assert_eq!(
            spot_of(
                "The Duke's Archives",
                "Homing Crystal Soulmass (Logan, Archives only)",
                ""
            ),
            "Logan's cell"
        );
        // Boss souls carry no place of their own; the section header puts
        // them at the smith who forges them.
        assert_eq!(
            spot_of(
                "Anor Londo",
                "Chaos Blade (Quelaag soul + katana)",
                "Boss-soul weapons (Giant Blacksmith)"
            ),
            "Giant Blacksmith"
        );
    }

    #[test]
    fn every_spot_claims_at_least_one_item() {
        for area in areas_for(DS1) {
            let mut claimed = vec![false; area.spots.len()];
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
                    if resolve(DS1, item, section) != super::index_of(DS1, area.name) {
                        continue;
                    }
                    if let Some(s) = super::resolve_spot(area, item, section) {
                        claimed[s] = true;
                    }
                }
            }
            for (s, got) in claimed.iter().enumerate() {
                assert!(
                    *got,
                    "{} / {} is a dead spot — no item resolves to it",
                    area.name, area.spots[s].name
                );
            }
        }
    }

    #[test]
    fn spot_cards_cannot_collide() {
        for area in areas_for(DS1) {
            for (i, a) in area.spots.iter().enumerate() {
                assert!(
                    (0.0..=1.0).contains(&a.pos.0) && (0.0..=1.0).contains(&a.pos.1),
                    "{} / {} sits outside the panel",
                    area.name,
                    a.name
                );
                for b in &area.spots[i + 1..] {
                    let (dx, dy) = ((a.pos.0 - b.pos.0).abs(), (a.pos.1 - b.pos.1).abs());
                    // Cards are wide and short: same row needs real horizontal gap.
                    assert!(
                        dy >= 0.14 || dx > 0.25,
                        "{}: '{}' and '{}' would overlap",
                        area.name,
                        a.name,
                        b.name
                    );
                }
            }
        }
    }

    /// The world map prints each area's name just beneath its node, so a
    /// node sitting close below a neighbour can land on top of that label —
    /// which is exactly what Sen's Fortress did to Anor Londo's.
    ///
    /// Modelled against the real geometry: a ~764x368 drawing area, a 10px
    /// proportional label centred under its node, and nodes up to radius 20.
    #[test]
    fn world_map_labels_are_not_covered() {
        const W: f32 = 764.0;
        const H: f32 = 368.0;
        const CHAR_W: f32 = 5.0;
        const MAX_R: f32 = 20.0;
        const LABEL_DROP: f32 = 36.0; // node centre -> bottom of its label

        let areas = areas_for(DS1);
        for (i, a) in areas.iter().enumerate() {
            for b in &areas[i + 1..] {
                // The higher node owns the label that could get covered.
                let (high, low) = if a.pos.1 < b.pos.1 { (a, b) } else { (b, a) };
                let label_half = high.name.len() as f32 * CHAR_W / 2.0;
                let dx = (high.pos.0 - low.pos.0).abs() * W;
                if dx >= label_half + MAX_R {
                    continue; // clear of each other horizontally
                }
                let gap = (low.pos.1 - high.pos.1) * H - MAX_R;
                assert!(
                    gap >= LABEL_DROP,
                    "'{}' label would be covered by '{}' (horizontal overlap, only \
                     {gap:.1}px of vertical room, needs {LABEL_DROP})",
                    high.name,
                    low.name
                );
            }
        }
    }
}
