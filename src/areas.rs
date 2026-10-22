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
    /// The area's outline on the world map, normalised 0..1, as a convex
    /// polygon. Drawn as a filled region so the map reads as a map rather
    /// than a bubble chart; the layout is ours, not traced from anyone's.
    pub shape: &'static [(f32, f32)],
    /// Muted fill colour for that region.
    pub tint: (u8, u8, u8),
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
// These are generated map coordinates; clippy reads the odd one as a
// near-miss of a math constant (0.3927 vs FRAC_PI_8), which it isn't.
#[allow(clippy::approx_constant)]
static DS1_AREAS: &[Area] = &[
    Area {
        name: "Northern Undead Asylum",
        strong: &["undead asylum"],
        weak: &["oscar"],
        map_url: "https://darksouls.wiki.fextralife.com/Northern+Undead+Asylum",
        pos: (0.0950, 0.0700),
        shape: &[
            (0.1728, 0.0725),
            (0.1489, 0.1053),
            (0.0907, 0.1166),
            (0.0358, 0.1013),
            (0.018, 0.0676),
            (0.0465, 0.0382),
            (0.0991, 0.0253),
            (0.1526, 0.0396),
        ],
        tint: (150, 160, 172),
        spots: SPOTS_ASYLUM,
    },
    Area {
        name: "Firelink Shrine",
        strong: &["firelink"],
        weak: &["griggs", "laurentius", "petrus", "siegmeyer", "logan"],
        map_url: "https://darksouls.wiki.fextralife.com/Firelink+Shrine",
        pos: (0.5875, 0.5175),
        shape: &[
            (0.6696, 0.5423),
            (0.6116, 0.5798),
            (0.5346, 0.5671),
            (0.5047, 0.5196),
            (0.526, 0.468),
            (0.6046, 0.4551),
            (0.6728, 0.4863),
        ],
        tint: (216, 200, 158),
        spots: SPOTS_FIRELINK,
    },
    Area {
        name: "Undead Burg",
        strong: &["undead burg"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Undead+Burg",
        pos: (0.3700, 0.5175),
        shape: &[
            (0.4507, 0.5268),
            (0.4089, 0.5652),
            (0.3377, 0.5702),
            (0.2958, 0.5319),
            (0.3079, 0.4877),
            (0.3657, 0.4655),
            (0.4309, 0.4807),
        ],
        tint: (205, 185, 155),
        spots: SPOTS_UNDEAD_BURG,
    },
    Area {
        name: "Undead Parish",
        strong: &["undead parish", "sunlight altar"],
        weak: &["reah", "oswald", "warrior of sunlight"],
        map_url: "https://darksouls.wiki.fextralife.com/Undead+Parish",
        pos: (0.3250, 0.3825),
        shape: &[
            (0.3962, 0.3935),
            (0.3615, 0.4338),
            (0.2926, 0.4279),
            (0.248, 0.3943),
            (0.2649, 0.35),
            (0.3251, 0.3279),
            (0.391, 0.347),
        ],
        tint: (200, 180, 150),
        spots: SPOTS_UNDEAD_PARISH,
    },
    Area {
        name: "Darkroot Garden",
        // "defeat sif" is safe where a bare "sif" is not — the boss-soul
        // weapons say "Sif soul" and must route to the Giant Blacksmith.
        strong: &["darkroot garden", "defeat sif"],
        weak: &["giant stone knights", "shiva"],
        map_url: "https://darksouls.wiki.fextralife.com/Darkroot+Garden",
        pos: (0.1350, 0.2300),
        shape: &[
            (0.2345, 0.2457),
            (0.1928, 0.2818),
            (0.1295, 0.2929),
            (0.0638, 0.2803),
            (0.0439, 0.2371),
            (0.0506, 0.193),
            (0.1079, 0.1698),
            (0.1727, 0.1717),
            (0.227, 0.1992),
        ],
        tint: (150, 180, 150),
        spots: SPOTS_DARKROOT_GARDEN,
    },
    Area {
        name: "Darkroot Basin",
        strong: &["darkroot basin"],
        weak: &["dusk"],
        map_url: "https://darksouls.wiki.fextralife.com/Darkroot+Basin",
        pos: (0.1150, 0.3850),
        shape: &[
            (0.1954, 0.3959),
            (0.1623, 0.4292),
            (0.0955, 0.4329),
            (0.0461, 0.4097),
            (0.0313, 0.3736),
            (0.0667, 0.3399),
            (0.1339, 0.3386),
            (0.1905, 0.3579),
        ],
        tint: (140, 175, 175),
        spots: SPOTS_DARKROOT_BASIN,
    },
    Area {
        name: "The Depths",
        strong: &["depths", "gaping dragon"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Depths",
        pos: (0.3500, 0.6575),
        shape: &[
            (0.4324, 0.6752),
            (0.3888, 0.7068),
            (0.322, 0.7054),
            (0.2733, 0.6796),
            (0.262, 0.6386),
            (0.3152, 0.6134),
            (0.377, 0.6114),
            (0.4267, 0.6354),
        ],
        tint: (150, 165, 135),
        spots: SPOTS_DEPTHS,
    },
    Area {
        name: "Blighttown",
        strong: &["blighttown"],
        weak: &["quelana"],
        map_url: "https://darksouls.wiki.fextralife.com/Blighttown",
        pos: (0.2875, 0.8050),
        shape: &[
            (0.3754, 0.8174),
            (0.3276, 0.8564),
            (0.2514, 0.851),
            (0.2041, 0.8167),
            (0.2137, 0.7687),
            (0.2876, 0.7501),
            (0.3556, 0.7716),
        ],
        tint: (160, 158, 118),
        spots: SPOTS_BLIGHTTOWN,
    },
    Area {
        name: "Quelaag's Domain",
        strong: &["quelaag's domain"],
        weak: &["eingyi", "chaos servant"],
        map_url: "https://darksouls.wiki.fextralife.com/Quelaag's+Domain",
        pos: (0.5225, 0.8075),
        shape: &[
            (0.6056, 0.8312),
            (0.5276, 0.8607),
            (0.4404, 0.8375),
            (0.4314, 0.7815),
            (0.5177, 0.7575),
            (0.6045, 0.7776),
        ],
        tint: (205, 150, 120),
        spots: SPOTS_QUELAAGS,
    },
    Area {
        name: "Sen's Fortress",
        strong: &["sen's fortress"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Sen's+Fortress",
        pos: (0.5350, 0.3650),
        shape: &[
            (0.6212, 0.3711),
            (0.6013, 0.4008),
            (0.5506, 0.4114),
            (0.4949, 0.4148),
            (0.4533, 0.3912),
            (0.4503, 0.359),
            (0.4763, 0.3333),
            (0.5178, 0.3139),
            (0.5749, 0.3154),
            (0.6148, 0.3394),
        ],
        tint: (172, 166, 156),
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
        pos: (0.5000, 0.2150),
        shape: &[
            (0.631, 0.2215),
            (0.5759, 0.2706),
            (0.4526, 0.2796),
            (0.3634, 0.2378),
            (0.3887, 0.1828),
            (0.4849, 0.156),
            (0.5941, 0.1705),
        ],
        tint: (214, 196, 140),
        spots: SPOTS_ANOR_LONDO,
    },
    Area {
        name: "Painted World of Ariamis",
        strong: &["painted world"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Painted+World+of+Ariamis",
        pos: (0.3150, 0.0650),
        shape: &[
            (0.4046, 0.0694),
            (0.3768, 0.0959),
            (0.3342, 0.1123),
            (0.2791, 0.1144),
            (0.2403, 0.0912),
            (0.2318, 0.061),
            (0.2449, 0.03),
            (0.2941, 0.0133),
            (0.3512, 0.0151),
            (0.3963, 0.0365),
        ],
        tint: (168, 186, 198),
        spots: SPOTS_PAINTED_WORLD,
    },
    Area {
        name: "The Catacombs",
        strong: &["catacombs"],
        weak: &["gravelord"],
        map_url: "https://darksouls.wiki.fextralife.com/The+Catacombs",
        pos: (0.8150, 0.5050),
        shape: &[
            (0.9049, 0.5064),
            (0.8563, 0.549),
            (0.7663, 0.551),
            (0.7215, 0.5036),
            (0.7722, 0.4593),
            (0.8647, 0.4581),
        ],
        tint: (156, 150, 150),
        spots: SPOTS_CATACOMBS,
    },
    Area {
        name: "Tomb of the Giants",
        strong: &["tomb of the giants"],
        weak: &["patches"],
        map_url: "https://darksouls.wiki.fextralife.com/Tomb+of+Giants",
        pos: (0.8650, 0.6600),
        shape: &[
            (0.9731, 0.6708),
            (0.9352, 0.7002),
            (0.8792, 0.7179),
            (0.8152, 0.7103),
            (0.7708, 0.6846),
            (0.7573, 0.6492),
            (0.7994, 0.6225),
            (0.8517, 0.6059),
            (0.9138, 0.6108),
            (0.949, 0.638),
        ],
        tint: (138, 136, 142),
        spots: SPOTS_TOMB,
    },
    Area {
        name: "New Londo Ruins",
        strong: &["new londo"],
        weak: &["ingward", "darkwraith", "four kings", "kaathe"],
        map_url: "https://darksouls.wiki.fextralife.com/New+Londo+Ruins",
        pos: (0.6050, 0.6675),
        shape: &[
            (0.6939, 0.6827),
            (0.6378, 0.7146),
            (0.5652, 0.7117),
            (0.514, 0.6772),
            (0.5402, 0.6329),
            (0.6094, 0.6143),
            (0.6791, 0.6353),
        ],
        tint: (160, 180, 172),
        spots: SPOTS_NEW_LONDO,
    },
    Area {
        name: "Valley of Drakes",
        strong: &["valley of drakes"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Valley+of+Drakes",
        pos: (0.1325, 0.5250),
        shape: &[
            (0.2296, 0.5324),
            (0.1792, 0.5675),
            (0.101, 0.5692),
            (0.0439, 0.5401),
            (0.0587, 0.4985),
            (0.1248, 0.4758),
            (0.1972, 0.4922),
        ],
        tint: (172, 180, 140),
        spots: SPOTS_VALLEY_OF_DRAKES,
    },
    Area {
        name: "The Great Hollow",
        strong: &["great hollow"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/The+Great+Hollow",
        pos: (0.0900, 0.7975),
        shape: &[
            (0.1622, 0.8009),
            (0.1434, 0.8343),
            (0.098, 0.8503),
            (0.0493, 0.8412),
            (0.0169, 0.8128),
            (0.0181, 0.7748),
            (0.055, 0.7464),
            (0.109, 0.7419),
            (0.1468, 0.7676),
        ],
        tint: (160, 140, 115),
        spots: &[],
    },
    Area {
        name: "Ash Lake",
        strong: &["ash lake"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Ash+Lake",
        pos: (0.1400, 0.9350),
        shape: &[
            (0.2423, 0.9392),
            (0.1845, 0.9788),
            (0.082, 0.9724),
            (0.0444, 0.9311),
            (0.0952, 0.891),
            (0.195, 0.8995),
        ],
        tint: (175, 195, 205),
        spots: SPOTS_ASH_LAKE,
    },
    Area {
        name: "The Duke's Archives",
        strong: &["archives", "seath"],
        weak: &["pisaca"],
        map_url: "https://darksouls.wiki.fextralife.com/The+Duke's+Archives",
        pos: (0.7950, 0.2100),
        shape: &[
            (0.885, 0.2167),
            (0.8705, 0.2508),
            (0.8135, 0.2666),
            (0.7512, 0.2627),
            (0.7118, 0.2359),
            (0.7041, 0.2032),
            (0.7227, 0.1709),
            (0.7768, 0.1542),
            (0.835, 0.1619),
            (0.874, 0.1854),
        ],
        tint: (178, 170, 200),
        spots: SPOTS_DUKES_ARCHIVES,
    },
    Area {
        name: "Crystal Cave",
        strong: &["crystal cave"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Crystal+Cave",
        pos: (0.9100, 0.0700),
        shape: &[
            (0.9886, 0.0736),
            (0.9527, 0.1089),
            (0.8872, 0.1166),
            (0.8392, 0.0875),
            (0.8414, 0.0454),
            (0.898, 0.0213),
            (0.9607, 0.0357),
        ],
        tint: (170, 200, 205),
        spots: &[],
    },
    Area {
        name: "Demon Ruins",
        strong: &["demon ruins"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Demon+Ruins",
        pos: (0.7575, 0.8115),
        shape: &[
            (0.8498, 0.8219),
            (0.8161, 0.8534),
            (0.7561, 0.8592),
            (0.6984, 0.8509),
            (0.6684, 0.8197),
            (0.674, 0.7825),
            (0.7303, 0.7658),
            (0.7873, 0.7665),
            (0.838, 0.7856),
        ],
        tint: (214, 160, 110),
        spots: &[],
    },
    Area {
        name: "Lost Izalith",
        strong: &["lost izalith"],
        weak: &[],
        map_url: "https://darksouls.wiki.fextralife.com/Lost+Izalith",
        pos: (0.7700, 0.9375),
        shape: &[
            (0.8726, 0.95),
            (0.8286, 0.9756),
            (0.7615, 0.9828),
            (0.6914, 0.9735),
            (0.666, 0.9417),
            (0.6844, 0.9102),
            (0.7426, 0.8912),
            (0.815, 0.8933),
            (0.86, 0.9183),
        ],
        tint: (210, 145, 100),
        spots: SPOTS_LOST_IZALITH,
    },
    Area {
        name: "Kiln of the First Flame",
        strong: &["kiln"],
        weak: &["black knight"],
        map_url: "https://darksouls.wiki.fextralife.com/Kiln+of+the+First+Flame",
        pos: (0.5550, 0.0565),
        shape: &[
            (0.6649, 0.0655),
            (0.626, 0.0899),
            (0.5683, 0.1009),
            (0.5041, 0.0987),
            (0.4573, 0.0775),
            (0.4493, 0.0478),
            (0.4808, 0.0217),
            (0.5402, 0.0071),
            (0.6032, 0.0165),
            (0.6481, 0.0365),
        ],
        tint: (190, 178, 158),
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

    fn cross(o: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }

    /// Regions are drawn with `Shape::convex_polygon`, which only renders
    /// correctly for convex outlines.
    #[test]
    fn region_shapes_are_convex_and_in_bounds() {
        for area in areas_for(DS1) {
            let p = area.shape;
            assert!(p.len() >= 3, "{} has no outline", area.name);
            for &(x, y) in p {
                assert!(
                    (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y),
                    "{} has a vertex outside the map at {x},{y}",
                    area.name
                );
            }
            let mut sign = 0.0f32;
            for i in 0..p.len() {
                let c = cross(p[i], p[(i + 1) % p.len()], p[(i + 2) % p.len()]);
                if c.abs() < 1e-9 {
                    continue;
                }
                if sign == 0.0 {
                    sign = c.signum();
                } else {
                    assert_eq!(c.signum(), sign, "{} is not convex", area.name);
                }
            }
        }
    }

    /// The name is painted at `pos`, so that point has to sit inside the
    /// region it names.
    #[test]
    fn region_labels_sit_inside_their_region() {
        for area in areas_for(DS1) {
            let p = area.shape;
            let (px, py) = area.pos;
            let mut hit = false;
            let mut j = p.len() - 1;
            for i in 0..p.len() {
                let (a, b) = (p[i], p[j]);
                if (a.1 > py) != (b.1 > py) && px < (b.0 - a.0) * (py - a.1) / (b.1 - a.1) + a.0 {
                    hit = !hit;
                }
                j = i;
            }
            assert!(
                hit,
                "{}'s label anchor is outside its own region",
                area.name
            );
        }
    }

    /// Two regions sharing screen space would render as one muddled blob, and
    /// a click could never pick the one underneath. Separating-axis test.
    #[test]
    fn regions_do_not_overlap() {
        fn separated(a: &[(f32, f32)], b: &[(f32, f32)]) -> bool {
            for poly in [a, b] {
                for i in 0..poly.len() {
                    let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
                    // outward normal of this edge
                    let (nx, ny) = (-(q.1 - p.1), q.0 - p.0);
                    let proj = |s: &[(f32, f32)]| {
                        s.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &(x, y)| {
                            let d = x * nx + y * ny;
                            (lo.min(d), hi.max(d))
                        })
                    };
                    let (alo, ahi) = proj(a);
                    let (blo, bhi) = proj(b);
                    if ahi < blo || bhi < alo {
                        return true;
                    }
                }
            }
            false
        }
        let areas = areas_for(DS1);
        for (i, a) in areas.iter().enumerate() {
            for b in &areas[i + 1..] {
                assert!(
                    separated(a.shape, b.shape),
                    "regions '{}' and '{}' overlap on the map",
                    a.name,
                    b.name
                );
            }
        }
    }
}
