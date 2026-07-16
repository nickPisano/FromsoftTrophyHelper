//! Hand-curated guide data for the achievements that actually need a plan:
//! missable warnings ("rules"), where you have to go ("locations"), and
//! good things to read or watch.
//!
//! Entries are matched against the Steam display name (case-insensitive
//! substring), so a slightly different schema name still finds its guide.
//! Achievements without a curated entry still get auto-generated search
//! resources in the UI, so every achievement has somewhere to go for help.

pub struct Resource {
    pub label: &'static str,
    pub url: &'static str,
}

pub struct Guide {
    pub appid: u32,
    /// Lowercase substrings; if any appears in the achievement's display
    /// name, this guide applies.
    pub matches: &'static [&'static str],
    /// True when the achievement can be lost for the current playthrough.
    pub missable: bool,
    /// Hard rules: points of no return, lockouts, prerequisites.
    pub rules: &'static [&'static str],
    /// Places you'll need to be.
    pub locations: &'static [&'static str],
    /// Earliest playthrough phase this realistically completes in:
    /// "" (anytime), "NG", "NG+", "NG++", or "Multiple runs".
    pub phase: &'static str,
    /// Discrete tickable sub-items for collect-them-all achievements.
    /// Checked state is persisted locally by the app.
    pub checklist: &'static [&'static str],
    pub resources: &'static [Resource],
}

/// Look up curated guide data for an achievement. The original DS2 shares
/// guides with Scholar of the First Sin, and Prepare to Die Edition shares
/// guides with Dark Souls: Remastered (identical achievement lists).
pub fn find(appid: u32, display_name: &str) -> Option<&'static Guide> {
    let appid = match appid {
        236430 => 335300,
        211420 => 570940,
        other => other,
    };
    let name = display_name.to_lowercase();
    GUIDES
        .iter()
        .find(|g| g.appid == appid && g.matches.iter().any(|m| name.contains(m)))
}

#[cfg(test)]
pub(crate) fn all_guides() -> &'static [Guide] {
    GUIDES
}

const ER: u32 = 1245620;
const DS3: u32 = 374320;
const SEKIRO: u32 = 814380;
const AC6: u32 = 1888160;
const DS1: u32 = 570940;
const DS2: u32 = 335300;

static GUIDES: &[Guide] = &[
    // ------------------------------------------------------------------
    // ELDEN RING
    // ------------------------------------------------------------------
    Guide {
        appid: ER,
        matches: &["age of the stars"],
        missable: true,
        rules: &[
            "Requires completing Ranni's entire questline before the final boss — start it early at Ranni's Rise in the Three Sisters (northwest Liurnia).",
            "Do NOT kill Ranni, and do not accept the Frenzied Flame (or cure it with Miquella's Needle) before choosing this ending.",
            "After beating the final boss, use the blue summon sign that appears near the Fractured Marika instead of touching her.",
            "Tip: back up your save folder before the final boss to collect all three ending achievements in one playthrough.",
        ],
        locations: &[
            "Ranni's Rise — Three Sisters, northwest Liurnia (quest start)",
            "Nokron, Eternal City — Fingerslayer Blade for Ranni",
            "Renna's Rise portal → Ainsel River Main → Lake of Rot",
            "Grand Cloister → Astel, Naturalborn of the Void",
            "Moonlight Altar — Cathedral of Manus Celes (give Ranni the Dark Moon Ring)",
        ],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — Ranni the Witch full questline",
                url: "https://eldenring.wiki.fextralife.com/Ranni+the+Witch",
            },
            Resource {
                label: "Fextralife — Endings overview",
                url: "https://eldenring.wiki.fextralife.com/Endings",
            },
        ],
    },
    Guide {
        appid: ER,
        matches: &["frenzied flame"],
        missable: true,
        rules: &[
            "WARNING: receiving the Frenzied Flame from the Three Fingers locks you into this ending and voids every other ending for the run — unless you cure it.",
            "The only cure is Miquella's Needle: complete Millicent's questline, then use the needle inside Dragonlord Placidusax's arena in Crumbling Farum Azula.",
            "You must remove ALL armor (fully naked) to open the door to the Three Fingers.",
            "Safest plan: back up your save before the final boss, take this ending from the backup, then restore.",
        ],
        locations: &[
            "Subterranean Shunning-Grounds beneath Leyndell → Cathedral of the Forsaken (behind Mohg, the Omen)",
            "Hidden path: hit the altar behind Mohg's arena, then descend the platforming section to the Frenzied Flame Proscription",
            "Crumbling Farum Azula — Dragonlord Placidusax arena (only place Miquella's Needle can be used)",
        ],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — Lord of Frenzied Flame ending",
                url: "https://eldenring.wiki.fextralife.com/Endings",
            },
            Resource {
                label: "Fextralife — Millicent questline (for Miquella's Needle)",
                url: "https://eldenring.wiki.fextralife.com/Millicent",
            },
        ],
    },
    Guide {
        appid: ER,
        matches: &["elden lord"],
        missable: false,
        rules: &[
            "The 'default' ending family: after the final boss, touch Fractured Marika (any Mending Rune variant also counts as Elden Lord).",
            "Only blocked if you're locked into the Frenzied Flame — cure it with Miquella's Needle or avoid the Three Fingers entirely.",
        ],
        locations: &["Leyndell, Ashen Capital — Elden Throne, after Radagon / Elden Beast"],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Endings overview",
            url: "https://eldenring.wiki.fextralife.com/Endings",
        }],
    },
    Guide {
        appid: ER,
        matches: &["legendary armaments"],
        missable: true,
        rules: &[
            "Nine weapons total. Only one is truly missable in a playthrough: Bolt of Gransax — pick it up in Leyndell, Royal Capital BEFORE defeating Maliketh. Once the city becomes the Ashen Capital it's gone for the cycle.",
            "Dark Moon Greatsword requires progressing Ranni's questline to its end.",
            "Devourer's Scepter comes from defeating Knight Bernahl (invades in Crumbling Farum Azula; also at Warmaster's Shack).",
            "You can spread the nine across NG+ cycles — the achievement counts what your character has ever obtained.",
        ],
        locations: &[
            "Grafted Blade Greatsword — Castle Morne (Leonine Misbegotten), Weeping Peninsula",
            "Sword of Night and Flame — Caria Manor chest, Liurnia",
            "Ruins Greatsword — Redmane Castle duo fight, Caelid",
            "Marais Executioner's Sword — Elemer of the Briar, The Shaded Castle",
            "Eclipse Shotel — Castle Sol, Mountaintops of the Giants",
            "Golden Order Greatsword — Misbegotten Crusader, Cave of the Forlorn",
            "Bolt of Gransax — Leyndell, on the giant spear (MISSABLE after Maliketh)",
            "Dark Moon Greatsword — Ranni's questline finale",
            "Devourer's Scepter — Knight Bernahl",
        ],
        phase: "NG",
        checklist: &[
            "Grafted Blade Greatsword (Castle Morne)",
            "Sword of Night and Flame (Caria Manor)",
            "Ruins Greatsword (Redmane Castle)",
            "Marais Executioner's Sword (Shaded Castle)",
            "Eclipse Shotel (Castle Sol)",
            "Golden Order Greatsword (Cave of the Forlorn)",
            "Bolt of Gransax (Leyndell — before Maliketh!)",
            "Dark Moon Greatsword (Ranni's questline)",
            "Devourer's Scepter (Knight Bernahl)",
        ],
        resources: &[Resource {
            label: "Fextralife — Legendary Armaments checklist",
            url: "https://eldenring.wiki.fextralife.com/Legendary+Armaments",
        }],
    },
    Guide {
        appid: ER,
        matches: &[
            "legendary sorceries",
            "legendary incantations",
            "legendary sorceries and incantations",
        ],
        missable: true,
        rules: &[
            "Seven spells. Two hang off questlines: Ranni's Dark Moon (complete puzzle tower Chelona's Rise, reachable via Ranni's questline area) and Stars of Ruin (from Master Lusat — requires Sellen's quest item, the Sellian Sealbreaker).",
            "Don't kill Sorceress Sellen before getting Stars of Ruin.",
            "Greyoll's Roar needs 3 Dragon Hearts and the Elder Dragon Greyoll's heart at the Cathedral of Dragon Communion.",
        ],
        locations: &[
            "Comet Azur — Primeval Sorcerer Azur, Hermit Village, Mt. Gelmir",
            "Stars of Ruin — Master Lusat, Sellia Hideaway (Sellen's quest)",
            "Ranni's Dark Moon — Chelona's Rise, Moonlight Altar",
            "Founding Rain of Stars — Heretical Rise, Mountaintops of the Giants",
            "Flame of the Fell God — Adan, Thief of Fire (Malefactor's Evergaol), Liurnia",
            "Greyoll's Roar — Cathedral of Dragon Communion, Caelid",
            "Elden Stars — Deeproot Depths",
        ],
        phase: "NG",
        checklist: &[
            "Comet Azur (Hermit Village, Mt. Gelmir)",
            "Stars of Ruin (Master Lusat, Sellia Hideaway)",
            "Ranni's Dark Moon (Chelona's Rise)",
            "Founding Rain of Stars (Heretical Rise)",
            "Flame of the Fell God (Malefactor's Evergaol)",
            "Greyoll's Roar (Cathedral of Dragon Communion)",
            "Elden Stars (Deeproot Depths)",
        ],
        resources: &[
            Resource {
                label: "Fextralife — Sorceries (all locations)",
                url: "https://eldenring.wiki.fextralife.com/Sorceries",
            },
            Resource {
                label: "Fextralife — Incantations (all locations)",
                url: "https://eldenring.wiki.fextralife.com/Incantations",
            },
        ],
    },
    Guide {
        appid: ER,
        matches: &["legendary talismans"],
        missable: false,
        rules: &[
            "Eight talismans, none missable, but two sit deep inside Miquella's Haligtree — you'll need both halves of the Haligtree Secret Medallion (Village of the Albinaurics + Castle Sol) to get there.",
            "Erdtree's Favor +2 only exists in Leyndell, Ashen Capital, i.e. after Maliketh — it's late, not missable.",
        ],
        locations: &[
            "Radagon Icon — Raya Lucaria Academy (chest, Debate Parlor area)",
            "Radagon's Soreseal — Fort Faroth, Caelid",
            "Godfrey Icon — Golden Lineage Evergaol, Liurnia",
            "Moon of Nokstella — Nokstella, Eternal City",
            "Dragoncrest Greatshield Talisman — Elphael, Brace of the Haligtree",
            "Marika's Soreseal — Elphael, Brace of the Haligtree",
            "Old Lord's Talisman — Crumbling Farum Azula",
            "Erdtree's Favor +2 — Leyndell, Ashen Capital",
        ],
        phase: "NG",
        checklist: &[
            "Radagon Icon (Raya Lucaria Academy)",
            "Radagon's Soreseal (Fort Faroth)",
            "Godfrey Icon (Golden Lineage Evergaol)",
            "Moon of Nokstella (Nokstella)",
            "Dragoncrest Greatshield Talisman (Elphael)",
            "Marika's Soreseal (Elphael)",
            "Old Lord's Talisman (Crumbling Farum Azula)",
            "Erdtree's Favor +2 (Leyndell, Ashen Capital)",
        ],
        resources: &[Resource {
            label: "Fextralife — Legendary Talismans checklist",
            url: "https://eldenring.wiki.fextralife.com/Legendary+Talismans",
        }],
    },
    Guide {
        appid: ER,
        matches: &["legendary ashen remains"],
        missable: false,
        rules: &[
            "Six legendary spirit ashes. Black Knife Tiche requires progressing Ranni's questline far enough to reach the Moonlight Altar and beating Alecto in the Ringleader's Evergaol.",
            "Mimic Tear is in Nokron — reachable only after Starscourge Radahn falls.",
        ],
        locations: &[
            "Mimic Tear Ashes — Night's Sacred Ground, Nokron",
            "Black Knife Tiche — Ringleader's Evergaol, Moonlight Altar",
            "Lhutel the Headless — Tombsward Catacombs, Weeping Peninsula",
            "Redmane Knight Ogha — War-Dead Catacombs, Caelid",
            "Ancient Dragon Knight Kristoff — Sainted Hero's Grave, Altus Plateau",
            "Cleanrot Knight Finlay — Elphael, Brace of the Haligtree",
        ],
        phase: "NG",
        checklist: &[
            "Mimic Tear Ashes (Night's Sacred Ground, Nokron)",
            "Black Knife Tiche (Ringleader's Evergaol)",
            "Lhutel the Headless (Tombsward Catacombs)",
            "Redmane Knight Ogha (War-Dead Catacombs)",
            "Ancient Dragon Knight Kristoff (Sainted Hero's Grave)",
            "Cleanrot Knight Finlay (Elphael)",
        ],
        resources: &[Resource {
            label: "Fextralife — Legendary Ashen Remains checklist",
            url: "https://eldenring.wiki.fextralife.com/Legendary+Spirit+Ashes",
        }],
    },
    Guide {
        appid: ER,
        matches: &["shardbearer mohg", "mohg, lord of blood"],
        missable: false,
        rules: &[
            "Reach Mohgwyn Palace either by completing White Mask Varre's questline (his invasion steps can be done offline against an NPC) or via the waygate portal in the far west of the Consecrated Snowfield.",
        ],
        locations: &[
            "Mohgwyn Palace — beneath Caelid / reached from Consecrated Snowfield portal",
            "Varre quest start — Rose Church, Liurnia",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — Mohg, Lord of Blood",
                url: "https://eldenring.wiki.fextralife.com/Mohg,+Lord+of+Blood",
            },
            Resource {
                label: "Fextralife — White Mask Varre questline",
                url: "https://eldenring.wiki.fextralife.com/White-Faced+Varre",
            },
        ],
    },
    Guide {
        appid: ER,
        matches: &["shardbearer malenia", "malenia"],
        missable: false,
        rules: &[
            "Miquella's Haligtree is a secret area: collect both halves of the Haligtree Secret Medallion, then solve the Ordina, Liturgical Town evergaol seal in the Consecrated Snowfield.",
            "Malenia heals on every hit that connects with you — bring bleed/frost and consider Mimic Tear or co-op.",
        ],
        locations: &[
            "Medallion (right) — Village of the Albinaurics, Liurnia (talk to the pot, Albus)",
            "Medallion (left) — Castle Sol, Mountaintops of the Giants",
            "Ordina, Liturgical Town — Consecrated Snowfield (evergaol puzzle)",
            "Malenia — Elphael, Brace of the Haligtree (bottom)",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Malenia, Blade of Miquella",
            url: "https://eldenring.wiki.fextralife.com/Malenia+Blade+of+Miquella",
        }],
    },
    // ------------------------------------------------------------------
    // DARK SOULS III
    // ------------------------------------------------------------------
    Guide {
        appid: DS3,
        matches: &["usurpation of fire"],
        missable: true,
        rules: &[
            "The strictest questline in the game. Draw out your true strength with Yoel of Londor FIVE times (level ups are free) to gain five Dark Sigils — Yoel dies after enough game progress, so do it early.",
            "NEVER heal the Dark Sigil with the Fire Keeper — that voids the ending for the playthrough.",
            "Follow Yuria's instructions: keep her alive, don't give the Fire Keeper the eyes in front of her plans, and complete Anri's 'wedding' ceremony in the Darkmoon Tomb (Anor Londo) using the Sword of Avowal.",
            "At the end, simply attack the First Flame instead of lighting it... the usurpation happens automatically if the quest is complete.",
        ],
        locations: &[
            "Undead Settlement road — recruit Yoel of Londor early",
            "Firelink Shrine — Yoel levels / Yuria appears after 5 sigils",
            "Church of Yorshka & Anor Londo — Anri questline steps",
            "Darkmoon Tomb (behind illusory wall below Anor Londo) — the ceremony",
            "Kiln of the First Flame — the ending itself",
        ],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — Yuria of Londor questline",
                url: "https://darksouls3.wiki.fextralife.com/Yuria+of+Londor",
            },
            Resource {
                label: "Fextralife — Endings",
                url: "https://darksouls3.wiki.fextralife.com/Endings",
            },
        ],
    },
    Guide {
        appid: DS3,
        matches: &["end of fire"],
        missable: true,
        rules: &[
            "Find the Eyes of a Fire Keeper in Untended Graves (the dark Firelink behind Oceiros) and give them to the Fire Keeper before the final boss.",
            "At the Kiln, summon the Fire Keeper with the sign after beating the Soul of Cinder — don't touch the bonfire first.",
            "Back up your save before the final boss if you want multiple ending achievements this cycle.",
        ],
        locations: &[
            "Untended Graves — hidden area behind Oceiros, the Consumed King (Consumed King's Garden)",
            "Illusory wall in the dark Firelink Shrine — Eyes of a Fire Keeper",
            "Kiln of the First Flame — summon sign after the final boss",
        ],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — Endings",
                url: "https://darksouls3.wiki.fextralife.com/Endings",
            },
            Resource {
                label: "Fextralife — Untended Graves",
                url: "https://darksouls3.wiki.fextralife.com/Untended+Graves",
            },
        ],
    },
    Guide {
        appid: DS3,
        matches: &["master of rings"],
        missable: false,
        rules: &[
            "The big one: every ring, including +1 versions (NG+ only) and +2 versions (NG++ only) — plan your three playthroughs around it.",
            "A handful come from questlines and covenants (e.g., Darkmoon Ring from Blades of the Darkmoon rank; some from Leonhard/Rosaria's chain), so keep NPC quests alive as you go.",
            "You can rush NG+ and NG++ — only the +1/+2 ring locations matter on those laps.",
        ],
        locations: &[
            "Everywhere — use a per-playthrough checklist (link below) and sweep +1/+2 spots on each NG lap",
        ],
        phase: "NG++",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Rings list (with NG+ locations)",
            url: "https://darksouls3.wiki.fextralife.com/Rings",
        }],
    },
    Guide {
        appid: DS3,
        matches: &["master of miracles"],
        missable: true,
        rules: &[
            "35 miracles — DLC miracles (Way of White Corona, Projected Heal, Lightning Arrow) do NOT count.",
            "Buy out Irina's stock from every Braille Divine Tome, and keep her questline on the 'good' path (don't feed her the dark tomes — give those to Karla; the spells count from either teacher).",
            "Covenant rewards are required: 10 + 30 Sunlight Medals (Warrior of Sunlight) and 10 + 30 Proofs of a Concord Kept (Blades of the Darkmoon). Both farm offline — Lothric Knights (medals) and Silver Knights in Anor Londo (proofs).",
            "Don't kill quest NPCs who teach miracles before you've bought everything.",
        ],
        locations: &[
            "Undead Settlement — free Irina (Tower Key not required, sewer route)",
            "Anor Londo — Silver Knight farming for Proofs of a Concord Kept",
            "Lothric Castle — Sunlight Medal farming",
            "Braille tomes — Farron Keep swamp, Carthus Catacombs, and beyond",
        ],
        phase: "NG",
        checklist: &[
            "## Irina — base stock",
            "Caressing Tears",
            "Heal (also Cleric start)",
            "Homeward",
            "Replenishment",
            "Heal Aid (Shrine Handmaid; Herald start)",
            "## Irina — after Tome of Carim",
            "Force",
            "Med Heal",
            "Tears of Denial",
            "## Irina — after Tome of Lothric",
            "Blessed Weapon",
            "Bountiful Light",
            "Magic Barrier",
            "## Deep Braille Divine Tome (Irina or Karla)",
            "Deep Protection",
            "Gnaw",
            "## Londor Braille Divine Tome (Irina or Karla)",
            "Dark Blade",
            "Dead Again",
            "Vow of Silence",
            "## Covenant rewards",
            "Sacred Oath (Warriors of Sunlight +1 — 10 Sunlight Medals)",
            "Great Lightning Spear (Warriors of Sunlight +2 — 30 Sunlight Medals)",
            "Darkmoon Blade (Blades of the Darkmoon +2 — 30 Proofs of a Concord Kept)",
            "## Boss-soul transpositions (Ludleth)",
            "Bountiful Sunlight (Soul of Rosaria)",
            "Lifehunt Scythe (Soul of Aldrich)",
            "Lightning Storm (Soul of the Nameless King)",
            "Soothing Sunlight (Soul of the Dancer)",
            "Sunlight Spear (Soul of the Lords)",
            "## World pickups",
            "Atonement (Farron Keep)",
            "Divine Pillars of Light (Grand Archives, golden cage)",
            "Dorhys' Gnawing (Irithyll, illusory railing)",
            "Emit Force (Siegward of Catarina)",
            "Great Heal (Irithyll swamp, dead Man-grub)",
            "Great Magic Barrier (Archdragon Peak, by Havel)",
            "Lightning Blade (Irithyll Dungeon, elevator drop)",
            "Lightning Spear (Farron Keep bridge)",
            "Lightning Stake (Carthus Sandworm drop)",
            "Seek Guidance (Cathedral of the Deep)",
            "Wrath of the Gods (Profaned Capital, swamp chapel)",
        ],
        resources: &[Resource {
            label: "Fextralife — Miracles list",
            url: "https://darksouls3.wiki.fextralife.com/Miracles",
        }],
    },
    Guide {
        appid: DS3,
        matches: &["master of sorceries"],
        missable: true,
        rules: &[
            "34 sorceries — DLC sorceries (Frozen Weapon, Snap Freeze, Great Soul Dregs, Old Moonlight) do NOT count.",
            "Orbeck of Vinheim must survive the whole quest: give him at least one scroll early or he leaves Firelink for good. Bring him all four scrolls (Sage's, Golden, Logan's, Crystal) and buy everything.",
            "Three sorceries are boss-soul transpositions via Ludleth — don't use those souls for weapons if you still need the spell.",
        ],
        locations: &[
            "Road of Sacrifices, Crucifixion Woods (crab lake) — Sage's Scroll",
            "Farron Keep — Golden Scroll (dark cave)",
            "Smouldering Lake — Logan's Scroll (demon cleric on the ridge)",
            "Grand Archives — Crystal Scroll",
        ],
        phase: "NG",
        checklist: &[
            "## Orbeck — base stock",
            "Farron Dart",
            "Soul Arrow (also Sorcerer start / Handmaid)",
            "Great Soul Arrow",
            "Heavy Soul Arrow",
            "Great Heavy Soul Arrow",
            "Farron Flashsword",
            "Spook",
            "Aural Decoy (Orbeck or Yoel)",
            "Magic Weapon (Orbeck or Yoel)",
            "Magic Shield (Orbeck or Yoel)",
            "Soul Greatsword (Orbeck or Yoel)",
            "## Orbeck — after Sage's Scroll",
            "Farron Hail",
            "Great Farron Dart",
            "Pestilent Mist",
            "## Orbeck — after Golden Scroll",
            "Cast Light",
            "Hidden Body",
            "Hidden Weapon",
            "Repair",
            "Twisted Wall of Light",
            "## Orbeck — after Logan's Scroll",
            "Homing Soulmass",
            "Soul Spear",
            "## Orbeck — after Crystal Scroll",
            "Crystal Magic Weapon",
            "Crystal Soul Spear",
            "Homing Crystal Soulmass",
            "## Karla",
            "Affinity (Karla)",
            "Dark Edge (Karla)",
            "## Boss-soul transpositions (Ludleth)",
            "Crystal Hail (Soul of a Crystal Sage)",
            "Deep Soul (Soul of the Deacons of the Deep)",
            "White Dragon Breath (Soul of Consumed Oceiros)",
            "## World & covenant",
            "Chameleon (Old Woman of Londor, Irithyll)",
            "Great Magic Weapon (Road of Sacrifices)",
            "Great Magic Shield (Irithyll Dungeon corpse-grub drop)",
            "Soul Stream (Grand Archives, illusory wall)",
            "Great Deep Soul (Aldrich Faithful +1 — 10 Human Dregs)",
        ],
        resources: &[
            Resource {
                label: "Fextralife — Orbeck of Vinheim",
                url: "https://darksouls3.wiki.fextralife.com/Orbeck+of+Vinheim",
            },
            Resource {
                label: "Fextralife — Sorceries list",
                url: "https://darksouls3.wiki.fextralife.com/Sorceries",
            },
        ],
    },
    Guide {
        appid: DS3,
        matches: &["master of pyromancies"],
        missable: true,
        rules: &[
            "27 pyromancies — DLC pyromancies (Flame Fan, Floating Chaos, Seething Chaos) do NOT count.",
            "Tomes split between two teachers: Cornyx takes Great Swamp, Carthus, and Izalith; only Karla accepts the Quelana and Grave Warden tomes. Keep both alive until you've bought everything.",
            "Warmth comes from the Mound Makers covenant (30 Vertebra Shackles — farmable offline by killing the mad-phantom NPC invaders repeatedly is slow; co-op trading is faster). Three more are boss-soul transpositions.",
        ],
        locations: &[
            "Undead Settlement — free Cornyx (cage)",
            "Irithyll Dungeon — free Karla (Jailer's Key Ring from Profaned Capital)",
            "Tomes — Undead Settlement, Catacombs of Carthus, Smouldering Lake, Grand Archives",
        ],
        phase: "NG",
        checklist: &[
            "## Cornyx — base stock",
            "Fireball (also Pyromancer start)",
            "Fire Surge",
            "Flash Sweat",
            "Great Combustion",
            "## Cornyx — after Great Swamp Tome",
            "Bursting Fireball",
            "Fire Orb",
            "Poison Mist",
            "Profuse Sweat",
            "## Cornyx — after Carthus Tome",
            "Acid Surge",
            "Carthus Beacon",
            "Carthus Flame Arc",
            "## Cornyx — after Izalith Tome",
            "Chaos Storm",
            "Great Chaos Fire Orb",
            "## Karla — after Quelana Tome",
            "Fire Whip",
            "Firestorm",
            "Rapport",
            "## Karla — after Grave Warden Tome",
            "Black Fire Orb",
            "Black Flame",
            "## Boss-soul transpositions (Ludleth)",
            "Black Serpent (Soul of High Lord Wolnir)",
            "Boulder Heave (Soul of a Stray Demon)",
            "Chaos Bed Vestiges (Soul of the Old Demon King)",
            "## World & covenant",
            "Iron Flesh (Farron Keep)",
            "Power Within (Grand Archives, behind movable bookcase)",
            "Profaned Flame (Irithyll Dungeon, Giant Slave cell)",
            "Sacred Flame (Smouldering Lake, past Tsorig)",
            "Toxic Mist (Smouldering Lake ruins, lava pool)",
            "Warmth (Mound Makers — 30 Vertebra Shackles)",
        ],
        resources: &[Resource {
            label: "Fextralife — Pyromancies list (who gets which tome)",
            url: "https://darksouls3.wiki.fextralife.com/Pyromancies",
        }],
    },
    Guide {
        appid: DS3,
        matches: &["ultimate estus", "ultimate bonfire"],
        missable: false,
        rules: &[
            "Collect all 11 Estus Shards (max flask) or all 10 Undead Bone Shards (max bonfire). None are missable — just well hidden.",
        ],
        locations: &[
            "Scattered across every region — the wiki checklist below has all of them with screenshots",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — Estus Shard locations",
                url: "https://darksouls3.wiki.fextralife.com/Estus+Shard",
            },
            Resource {
                label: "Fextralife — Undead Bone Shard locations",
                url: "https://darksouls3.wiki.fextralife.com/Undead+Bone+Shard",
            },
        ],
    },
    // ------------------------------------------------------------------
    // SEKIRO
    // ------------------------------------------------------------------
    Guide {
        appid: SEKIRO,
        matches: &["shura"],
        missable: true,
        rules: &[
            "On the Ashina Castle rooftop (after the Divine Dragon prerequisites mid-game), choose to OBEY the Iron Code with Owl. This ends the game early with two exclusive boss fights.",
            "This choice locks out the other three endings for the playthrough — most players do Shura as a short second run (it's only a few hours with routing) or back up the save before the choice.",
        ],
        locations: &["Ashina Castle rooftop — the choice with Great Shinobi Owl"],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Sekiro endings",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Endings",
        }],
    },
    Guide {
        appid: SEKIRO,
        matches: &["purification"],
        missable: true,
        rules: &[
            "Strict hidden steps: after refusing Owl, eavesdrop on Kuro and Emma (multiple specific eavesdrops at Kuro's room / the Old Grave), talk to Emma repeatedly, and receive the Father's Bell Charm.",
            "Use the bell charm at the Hirata Estate to enter the SECOND Hirata memory and defeat Owl (Father).",
            "The Aromatic Flower you receive enables the Purification choice at the end. Miss an eavesdrop and the chain can stall — follow a step-by-step guide.",
        ],
        locations: &[
            "Kuro's Room / Old Grave — eavesdropping steps",
            "Hirata Estate (second memory) — Owl (Father) boss",
            "Fountainhead Palace → final sequence — choose Purification",
        ],
        phase: "Multiple runs",
        checklist: &[
            "Refused Owl at the Ashina Castle rooftop",
            "All Kuro / Emma eavesdrops completed",
            "Father's Bell Charm received from Emma",
            "Owl (Father) defeated in second Hirata memory",
            "Aromatic Flower in inventory",
            "Chose Purification at the final decision",
        ],
        resources: &[Resource {
            label: "Fextralife — Purification ending walkthrough",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Endings",
        }],
    },
    Guide {
        appid: SEKIRO,
        matches: &["dragon's homecoming", "homecoming"],
        missable: true,
        rules: &[
            "Requires the Divine Child questline: give her rice repeatedly, get the Holy Chapter: Infested, then bring her both Serpent Viscera (Fresh — the Great Serpent's shrine cave in Sunken Valley; Dried — the second serpent reached via the Senpou Temple kite puzzle) to receive the Frozen Tears.",
            "Choose 'Return' at the final decision while holding the Frozen Tears.",
            "Can be combined with Purification prep up to the final choice — back up your save at the end to take both.",
        ],
        locations: &[
            "Senpou Temple, Mt. Kongo — Divine Child of Rejuvenation",
            "Sunken Valley / Great Serpent cave — Fresh Serpent Viscera",
            "Senpou Temple depths — Dried Serpent Viscera",
        ],
        phase: "Multiple runs",
        checklist: &[
            "Rice given to the Divine Child (repeat until quest advances)",
            "Holy Chapter: Infested obtained",
            "Fresh Serpent Viscera (Sunken Valley shrine cave)",
            "Dried Serpent Viscera (Senpou kite puzzle)",
            "Frozen Tears received",
            "Chose Return at the final decision",
        ],
        resources: &[Resource {
            label: "Fextralife — Return / Dragon's Homecoming ending",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Endings",
        }],
    },
    Guide {
        appid: SEKIRO,
        matches: &["man without equal"],
        missable: true,
        rules: &[
            "Defeat every boss in the game — including the Shura-route exclusives (Emma & Isshin Ashina) AND the normal-route finale (Isshin, the Sword Saint / Divine Dragon). Impossible in one run without a save backup: plan two playthroughs minimum.",
            "Includes all optional bosses: Demon of Hatred, Great Shinobi Owl, Owl (Father), Headless Ape rematch, and both Hirata memories.",
        ],
        locations: &["Everywhere — use a full boss checklist and tick them per playthrough"],
        phase: "Multiple runs",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Bosses checklist",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Bosses",
        }],
    },
    Guide {
        appid: SEKIRO,
        matches: &["height of technique", "all skills"],
        missable: false,
        rules: &[
            "You need every esoteric text (some are quest-gated — e.g., complete Tengu's rat-hunt for the Ashina text; late-game texts come from endgame areas and NPC steps) and a mountain of skill points.",
            "Expect to farm or carry into NG+ — total XP needed far exceeds one playthrough.",
        ],
        locations: &[
            "Ashina Castle — Tengu of Ashina (rat mission → esoteric text)",
            "Senpou Temple, Fountainhead Palace — remaining texts (see checklist)",
        ],
        phase: "NG+",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Skills & esoteric texts",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Skills",
        }],
    },
    Guide {
        appid: SEKIRO,
        matches: &["prosthetic"],
        missable: false,
        rules: &[
            "Fully upgrading every prosthetic tool needs rare materials — most notably Lapis Lazuli, which only appears in the Fountainhead Palace and NG+ sources. Plan on more than one playthrough's worth of materials.",
            "Find all 10 base prosthetic tools first (a couple are easy to walk past — e.g., the Sabimaru and the Finger Whistle).",
        ],
        locations: &[
            "Fountainhead Palace — Lapis Lazuli sources",
            "Full tool + upgrade material checklist on the wiki",
        ],
        phase: "NG+",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Prosthetic tools & upgrades",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Prosthetic+Tools",
        }],
    },
    // ------------------------------------------------------------------
    // ARMORED CORE VI
    // ------------------------------------------------------------------
    Guide {
        appid: AC6,
        matches: &["fires of raven"],
        missable: true,
        rules: &[
            "One of two endings available in your first/second playthrough. Ending choice is driven by which decision missions you accept in the final chapters (side with Carla / burn Rubicon).",
            "You can replay via NG+ to pick the other branch — decision missions are clearly marked with mutually exclusive choices.",
        ],
        locations: &["Chapter 4-5 decision missions — pick the 'Fires of Raven' branch"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — AC6 endings & decision missions",
            url: "https://armoredcore6.wiki.fextralife.com/Trophy+%26+Achievement+Guide",
        }],
    },
    Guide {
        appid: AC6,
        matches: &["liberator of rubicon"],
        missable: true,
        rules: &[
            "The counterpart branch to Fires of Raven — side with Ayre when the decision missions split. Take whichever you skipped on your other run via NG+.",
        ],
        locations: &["Chapter 4-5 decision missions — pick the 'Liberator of Rubicon' branch"],
        phase: "NG+",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — AC6 endings & decision missions",
            url: "https://armoredcore6.wiki.fextralife.com/Trophy+%26+Achievement+Guide",
        }],
    },
    Guide {
        appid: AC6,
        matches: &["alea iacta est"],
        missable: false,
        rules: &[
            "The true ending — ONLY available in NG++ (third playthrough). New exclusive decision missions appear across the campaign; complete the new branch to unlock the final route.",
            "You must make the specific new-mission choices in NG++ — follow a mission-order guide so you don't have to run a fourth lap.",
        ],
        locations: &["NG++ exclusive missions across all chapters"],
        phase: "NG++",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — True ending (NG++) route",
                url: "https://armoredcore6.wiki.fextralife.com/Trophy+%26+Achievement+Guide",
            },
            Resource {
                label: "Fextralife — AC6 Walkthrough (mission routes)",
                url: "https://armoredcore6.wiki.fextralife.com/Walkthrough",
            },
        ],
    },
    Guide {
        appid: AC6,
        matches: &["asset holder"],
        missable: false,
        rules: &[
            "Own every part. Several parts come only from: NG+/NG++ exclusive missions, hidden part containers inside missions, and Arena completion — sweep the Arena every chapter.",
            "Money is the real constraint late — replay high-payout missions to afford the last purchases.",
        ],
        locations: &[
            "Parts Shop (buy everything), Arena rewards, hidden containers — full checklist on the wiki",
        ],
        phase: "NG++",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — AC6 Trophy Guide (all-parts section)",
            url: "https://armoredcore6.wiki.fextralife.com/Trophy+%26+Achievement+Guide",
        }],
    },
    Guide {
        appid: AC6,
        matches: &["combat log", "battle log", "loghunt"],
        missable: false,
        rules: &[
            "Combat logs drop from specific marked enemies inside missions — many only exist in NG+ / NG++ mission variants, so log hunting spans all three playthroughs.",
            "Use the Loghunt list from the ALLMIND terminal to track tiers; replaying a mission lets you re-grab any log you missed.",
        ],
        locations: &["Marked enemies across missions — full per-mission log list on the wiki"],
        phase: "NG++",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Combat Logs per mission",
            url: "https://armoredcore6.wiki.fextralife.com/Combat+Logs",
        }],
    },
    // ------------------------------------------------------------------
    // DARK SOULS: REMASTERED
    // ------------------------------------------------------------------
    Guide {
        appid: DS1,
        matches: &["knight's honor"],
        missable: true,
        rules: &[
            "Collect all 47 rare weapons on ONE character — the true long-haul achievement. Several boss souls forge MULTIPLE weapons (Sif's makes three, Quelaag's, the Butterfly's, the Golem's and Gwyndolin's make two each), so a solo run needs souls from NG+ and NG++ — or have another player drop weapons/souls for you (trades count).",
            "DLC weapons do NOT count: Obsidian Greatsword, Guardian Tail, Gough's Greatbow etc. are not needed. Neither are the Gargoyle Tail Axe, Crystal Halberd, or other rare-but-not-required drops.",
            "Boss-soul weapons need a +10 base weapon of the right class ascended at the Giant Blacksmith — stock up on large titanite.",
            "For the drop farming (Black Knights, Silver Knights, Channelers), wear the Covetous Gold Serpent Ring and hold 10 humanity to max Item Discovery.",
        ],
        locations: &[
            "Giant Blacksmith, Anor Londo — all boss-soul ascensions",
            "Kiln of the First Flame — respawning Black Knights for all four weapon drops",
            "Anor Londo — Silver Knight weapons, chest/corpse pickups",
            "Ash Lake — the Stone Dragon's tail (Dragon Greatsword)",
        ],
        phase: "NG++",
        checklist: &[
            "## Tail cuts",
            "Drake Sword (Hellkite Drake's tail, Undead Burg bridge)",
            "Dragon King Greataxe (Gaping Dragon's tail)",
            "Priscilla's Dagger (Priscilla's tail, Painted World)",
            "Moonlight Greatsword (Seath's blue tail, The Duke's Archives)",
            "Dragon Greatsword (Stone Dragon's tail, Ash Lake)",
            "## Boss-soul weapons (Giant Blacksmith)",
            "Moonlight Butterfly Horn (Butterfly soul + spear/thrusting sword)",
            "Crystal Ring Shield (Butterfly soul + shield)",
            "Quelaag's Furysword (Quelaag soul + curved sword)",
            "Chaos Blade (Quelaag soul + katana)",
            "Greatsword of Artorias (Sif soul + Broken Straight Sword or hilt)",
            "Greatsword of Artorias, Cursed (Sif soul + dagger/greatsword/straight sword)",
            "Greatshield of Artorias (Sif soul + shield)",
            "Golem Axe (Iron Golem core + axe)",
            "Dragon Bone Fist (Iron Golem core + fist weapon)",
            "Lifehunt Scythe (Priscilla soul + halberd/whip)",
            "Dragonslayer Spear (Ornstein soul + spear/thrusting sword)",
            "Smough's Hammer (Smough soul + hammer)",
            "Darkmoon Bow (Gwyndolin soul + bow)",
            "Tin Darkmoon Catalyst (Gwyndolin soul + catalyst)",
            "Great Lord Greatsword (Gwyn soul + dagger/greatsword/straight sword)",
            "## Covenant weapons",
            "Gravelord Sword (join the Gravelord Servant covenant)",
            "Dark Hand (join the Darkwraith covenant, or Darkwraith drop)",
            "## Enemy drops",
            "Black Knight Sword (sword Black Knights)",
            "Black Knight Greatsword (greatsword Black Knights)",
            "Black Knight Greataxe (greataxe Black Knights)",
            "Black Knight Halberd (halberd Black Knights)",
            "Black Knight Shield (any Black Knight)",
            "Channeler's Trident (Channelers — Parish/Depths/Archives)",
            "Crescent Axe (Patches — drop or buy)",
            "Crest Shield (Oscar, Undead Asylum revisit)",
            "Giant's Halberd (Anor Londo Sentinels, or Giant Blacksmith)",
            "Silver Knight Straight Sword (Anor Londo)",
            "Silver Knight Spear (Anor Londo)",
            "Silver Knight Shield (Anor Londo)",
            "Stone Greatsword (Giant Stone Knights, or Shiva)",
            "Stone Greatshield (Giant Stone Knights, Darkroot Garden)",
            "Grant (Black Phantom Paladin Leeroy, Tomb of the Giants)",
            "Sanctus (Black Phantom Paladin Leeroy, Tomb of the Giants)",
            "## Corpse & chest loot",
            "Astora's Straight Sword (Valley of Drakes, by the Undead Dragon)",
            "Dragon Crest Shield (Valley of Drakes, by the Undead Dragon)",
            "Havel's Greatshield (Anor Londo, illusory-wall chest)",
            "Dragon Tooth (Anor Londo, illusory-wall chest)",
            "Dragonslayer Greatbow (Anor Londo, broken window ledge)",
            "Velka's Rapier (Painted World, behind the Annex Key door)",
            "Bloodshield (Painted World, by the Undead Dragon)",
            "Black Iron Greatshield (Anor Londo, next to the painting)",
            "Effigy Shield (Tomb of the Giants)",
        ],
        resources: &[Resource {
            label: "Fextralife — DS1 Trophy & Achievement Guide (Knight's Honor list)",
            url: "https://darksouls.wiki.fextralife.com/Trophy+%26+Achievement+Guide",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["dark wraith", "darkwraith"],
        missable: true,
        rules: &[
            "You must reach Darkstalker Kaathe in the Abyss: defeat the Four Kings BEFORE placing the Lordvessel for Frampt. If you've already placed it via Frampt, Kaathe won't appear this playthrough.",
            "Four Kings before Lordvessel means fighting them without the usual endgame power — level up and bring Havel's/stamina gear.",
        ],
        locations: &[
            "New Londo Ruins → The Abyss (requires the Covenant of Artorias ring from Sif)",
            "Kaathe appears after the Four Kings die (if Frampt hasn't been served)",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Darkwraith covenant",
            url: "https://darksouls.wiki.fextralife.com/Darkwraith",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["wisdom of a sage"],
        missable: true,
        rules: &[
            "24 sorceries, all on one character. DLC sorceries (Dark Bead, Dark Orb, Dark Fog, Pursuers) do NOT count.",
            "Big Hat Logan's questline is the trap: free him in Sen's Fortress (cell key on the upper walkway), buy his stock at Firelink, free him AGAIN in the Duke's Archives, buy the rest — then after he goes hollow, defeat him for White Dragon Breath.",
            "If Logan dies or you skip a step, several sorceries are gone until NG+.",
            "Also buy out Griggs and Dusk of Oolacile (rescue her golem in Darkroot Basin; don't kill her).",
        ],
        locations: &[
            "Sen's Fortress — Logan's cage",
            "Firelink Shrine — Logan & Griggs vendors",
            "The Duke's Archives — Logan's second cell + hollow Logan (Seath's arena)",
            "Darkroot Basin — Dusk (Crystal Golem rescue)",
        ],
        phase: "NG",
        checklist: &[
            "## Griggs / Logan basics",
            "Soul Arrow (Griggs, Rickert, or Logan)",
            "Great Soul Arrow (Griggs or Logan)",
            "Heavy Soul Arrow (Griggs, Rickert, or Logan)",
            "Great Heavy Soul Arrow (Griggs or Logan)",
            "Magic Weapon (Griggs or Logan)",
            "Magic Shield (Griggs or Logan)",
            "Aural Decoy (Griggs)",
            "Fall Control (Griggs)",
            "Homing Soulmass (Logan; Griggs after Logan leaves)",
            "Soul Spear (Logan; Griggs after Logan leaves)",
            "## Logan in The Duke's Archives",
            "Homing Crystal Soulmass (Logan, Archives only)",
            "Crystal Soul Spear (Logan, Archives only)",
            "Crystal Magic Weapon (Logan, Archives only)",
            "White Dragon Breath (defeat hollow Logan in Seath's arena)",
            "## Dusk of Oolacile",
            "Hidden Body (Dusk)",
            "Hidden Weapon (Dusk)",
            "Cast Light (Dusk)",
            "Chameleon (Dusk)",
            "Repair (Dusk)",
            "## World pickups & others",
            "Great Magic Weapon (Anor Londo)",
            "Strong Magic Shield (The Duke's Archives)",
            "Hush (Sen's Fortress chest)",
            "Remedy (Blighttown)",
            "Resist Curse (Ingward, New Londo Ruins)",
        ],
        resources: &[
            Resource {
                label: "Fextralife — Sorceries checklist",
                url: "https://darksouls.wiki.fextralife.com/Sorceries",
            },
            Resource {
                label: "Fextralife — Big Hat Logan questline",
                url: "https://darksouls.wiki.fextralife.com/Big+Hat+Logan",
            },
        ],
    },
    Guide {
        appid: DS1,
        matches: &["bond of a pyromancer"],
        missable: true,
        rules: &[
            "19 pyromancies, all on one character. The DLC's Black Flame does NOT count.",
            "Rescue Laurentius in the Depths (roll into the barrel — don't hit it); later, if he asks about your pyromancy, DON'T tell him where you learned advanced flames or he leaves and hollows.",
            "Quelana appears in Blighttown swamp once you carry an upgraded pyromancy flame; buy everything and complete her task for Fire Tempest.",
            "Eingyi (behind the illusory wall past Quelaag) sells Poison/Toxic Mist — Toxic Mist requires answering him with the egghead status.",
            "Chaos pyromancies come from the Chaos Servant covenant (Great Chaos Fireball at join, Chaos Storm at +2 — 30 humanity total).",
        ],
        locations: &[
            "The Depths — Laurentius rescue",
            "Blighttown swamp — Quelana of Izalith",
            "Quelaag's Domain — Eingyi + Chaos Servant covenant",
            "Painted World of Ariamis & Lost Izalith — world pickups",
        ],
        phase: "NG",
        checklist: &[
            "## Laurentius of the Great Swamp",
            "Fireball (Laurentius or Quelana; Pyromancer start)",
            "Fire Orb (Laurentius or Quelana)",
            "Combustion (Laurentius or Quelana)",
            "Flash Sweat (Laurentius)",
            "Iron Flesh (Laurentius)",
            "## Quelana of Izalith",
            "Great Fireball (Quelana)",
            "Great Combustion (Quelana)",
            "Fire Whip (Quelana)",
            "Firestorm (Quelana)",
            "Fire Tempest (complete Quelana's task)",
            "Undead Rapport (Quelana)",
            "## Eingyi & Chaos Servant",
            "Poison Mist (Eingyi or Blighttown)",
            "Toxic Mist (Eingyi — needs egghead status)",
            "Great Chaos Fireball (join Chaos Servant covenant)",
            "Chaos Storm (Chaos Servant +2 — 30 humanity)",
            "## World pickups",
            "Acid Surge (Painted World of Ariamis)",
            "Fire Surge (Painted World of Ariamis)",
            "Power Within (Blighttown)",
            "Chaos Fire Whip (Lost Izalith)",
        ],
        resources: &[Resource {
            label: "Fextralife — Pyromancies checklist",
            url: "https://darksouls.wiki.fextralife.com/Pyromancies",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["prayer of a maiden"],
        missable: true,
        rules: &[
            "All 23 miracles. Unusually, this one is cumulative across characters — spells collected on different characters all count.",
            "Reah of Thorolund's questline is still the trap: rescue her party in the Tomb of the Giants, then buy ALL her miracles at the Undead Parish before she moves to the Duke's Archives (where she can die).",
            "Covenant miracles: Lightning Spear / Great Lightning Spear / Sunlight Spear from Warrior of Sunlight (Sunlight Spear also needs Gwyn's soul offered at the altar — one full run reaches it in NG+), Darkmoon Blade from Blade of the Darkmoon +1, Gravelord miracles from Nito's covenant.",
        ],
        locations: &[
            "Tomb of the Giants — Reah's rescue + Replenishment",
            "Undead Parish — Reah's vendor window + Sunlight Altar",
            "Anor Londo — Darkmoon covenant + Sunlight Blade (Darkmoon Tomb)",
            "The Catacombs — Gravelord covenant",
        ],
        phase: "NG",
        checklist: &[
            "## Petrus & Reah of Thorolund",
            "Heal (Petrus, Reah, or Patches; Cleric start)",
            "Great Heal Excerpt (Petrus, Reah, or Patches)",
            "Great Heal (Reah)",
            "Force (Petrus or Reah)",
            "Homeward (Petrus or Reah)",
            "Seek Guidance (Petrus or Reah)",
            "Magic Barrier (Reah)",
            "Wrath of the Gods (Reah)",
            "Replenishment (Reah, at the Tomb of the Giants)",
            "## Other NPCs",
            "Emit Force (Siegmeyer of Catarina, Firelink)",
            "Karmic Justice (Oswald of Carim)",
            "Bountiful Sunlight (Pisaca drop, The Duke's Archives)",
            "Soothing Sunlight (Pisaca drop, The Duke's Archives)",
            "## Covenants",
            "Lightning Spear (join Warrior of Sunlight)",
            "Great Lightning Spear (Warrior of Sunlight +1 — 10 Sunlight Medals)",
            "Sunlight Spear (offer Gwyn's soul at the Sunlight Altar)",
            "Darkmoon Blade (Blade of the Darkmoon +1 — 10 Souvenirs of Reprisal)",
            "Gravelord Sword Dance (join Gravelord Servant)",
            "Gravelord Greatsword Dance (Gravelord Servant +1 — Eyes of Death)",
            "## World pickups",
            "Sunlight Blade (Darkmoon Tomb, after Gwyndolin)",
            "Great Magic Barrier (Ash Lake)",
            "Tranquil Walk of Peace (The Catacombs)",
            "Vow of Silence (Painted World of Ariamis)",
        ],
        resources: &[Resource {
            label: "Fextralife — Miracles checklist",
            url: "https://darksouls.wiki.fextralife.com/Miracles",
        }],
    },
    // ------------------------------------------------------------------
    // DARK SOULS II (both versions — matched via SOTFS appid)
    // ------------------------------------------------------------------
    Guide {
        appid: DS2,
        matches: &["master of sorcery"],
        missable: false,
        rules: &[
            "Several sorceries are forged from boss souls by Straid or Ornifex — beating some bosses more than once is required. Burn Bonfire Ascetics to respawn a boss instead of waiting for NG+.",
            "Buy out every sorcery vendor (Carhillion, Magerold, Straid) and check covenant rewards.",
        ],
        locations: &[
            "The Lost Bastille — Straid (trade boss souls)",
            "Brightstone Cove Tseldora — Ornifex (trade boss souls)",
            "Majula/No-man's Wharf — Carhillion",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — DS2 Sorceries checklist",
            url: "https://darksouls2.wiki.fextralife.com/Sorceries",
        }],
    },
    Guide {
        appid: DS2,
        matches: &["master of miracles"],
        missable: false,
        rules: &[
            "Includes covenant rewards — notably the Heirs of the Sun rank reward (Sunlight Medals: earn via co-op, or farm medal drops offline with Bonfire Ascetics) and boss-soul miracles from Straid/Ornifex.",
            "Licia of Lindeldt sells the bulk — buy her out early.",
        ],
        locations: &[
            "Heide's Tower of Flame → Majula — Licia",
            "Sunlight altar (Heirs of the Sun covenant) — see the wiki for its spot in your version",
            "The Lost Bastille / Brightstone Cove Tseldora — boss-soul miracles",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — DS2 Miracles checklist",
            url: "https://darksouls2.wiki.fextralife.com/Miracles",
        }],
    },
    Guide {
        appid: DS2,
        matches: &["master of pyromancy"],
        missable: false,
        rules: &[
            "Buy out Rosabeth of Melfia (unpetrify her with a Fragrant Branch of Yore below Majula's Shaded Woods gate) and collect world pickups; a couple come from boss souls and covenant ranks.",
        ],
        locations: &[
            "Shaded Woods entrance — Rosabeth (petrified)",
            "World pickups — see checklist",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — DS2 Pyromancies checklist",
            url: "https://darksouls2.wiki.fextralife.com/Pyromancies",
        }],
    },
    Guide {
        appid: DS2,
        matches: &["master of hexes"],
        missable: false,
        rules: &[
            "Requires the Pilgrims of Dark covenant: find Darkdiver Grandahl in all THREE locations, clear the three Dark Chasm of Old dungeons (Human Effigy toll each), and defeat the Darklurker for the final hex.",
            "Felkin the Outcast only sells hexes once you have 20 Intelligence and 20 Faith — talk to him again at those stats. Boss-soul hexes come via Straid/Ornifex.",
        ],
        locations: &[
            "Shaded Woods, Black Gulch, Drangleic Castle — Grandahl's three spots",
            "Dark Chasm of Old — three dungeons + Darklurker",
            "Huntsman's Copse — Felkin",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[
            Resource {
                label: "Fextralife — DS2 Hexes checklist",
                url: "https://darksouls2.wiki.fextralife.com/Hexes",
            },
            Resource {
                label: "Fextralife — Darklurker",
                url: "https://darksouls2.wiki.fextralife.com/Darklurker",
            },
        ],
    },
    // ------------------------------------------------------------------
    // ELDEN RING — remaining shardbearers & Great Runes
    // ------------------------------------------------------------------
    Guide {
        appid: ER,
        matches: &["shardbearer godrick", "godrick"],
        missable: false,
        rules: &[
            "The first mandatory shardbearer. If he's a wall, explore Limgrave and the Weeping Peninsula first — levels and Ashes of War carry this fight.",
        ],
        locations: &["Stormveil Castle — throne room (Limgrave)"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Godrick the Grafted",
            url: "https://eldenring.wiki.fextralife.com/Godrick+the+Grafted",
        }],
    },
    Guide {
        appid: ER,
        matches: &["shardbearer radahn", "radahn"],
        missable: false,
        rules: &[
            "Fought at the Radahn Festival at Redmane Castle: reach the castle via the bridge in far south Caelid, or the teleporter at Impassable Greatbridge. The festival activates once you arrive (or after touching an Altus grace).",
            "Summon signs litter the whole arena — this fight is designed around re-summoning NPC allies. Torrent is allowed.",
            "Defeating Radahn also makes Nokron (Ranni's questline, Mimic Tear) accessible — you'll be back here for other achievements.",
        ],
        locations: &["Redmane Castle → Wailing Dunes, Caelid"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Starscourge Radahn",
            url: "https://eldenring.wiki.fextralife.com/Starscourge+Radahn",
        }],
    },
    Guide {
        appid: ER,
        matches: &["shardbearer rykard", "rykard"],
        missable: false,
        rules: &[
            "Reach him through Volcano Manor: either complete the Manor's assassination questline (Tanith's contracts — also unlocks other rewards) or get grabbed by the Abductor Virgin at the bottom of Raya Lucaria's waterwheel.",
            "Pick up the Serpent-Hunter greatspear ON the arena floor — the fight is balanced around it.",
        ],
        locations: &["Volcano Manor → Mt. Gelmir, Audience Pathway"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Rykard, Lord of Blasphemy",
            url: "https://eldenring.wiki.fextralife.com/Rykard,+Lord+of+Blasphemy",
        }],
    },
    Guide {
        appid: ER,
        matches: &["shardbearer morgott", "morgott"],
        missable: false,
        rules: &[
            "Entering Leyndell requires holding two Great Runes. Morgott waits at the base of the Erdtree.",
            "While you're in Leyndell: grab the Bolt of Gransax now if you're chasing Legendary Armaments — the window closes after Maliketh.",
        ],
        locations: &["Leyndell, Royal Capital — Elden Throne approach"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Morgott, the Omen King",
            url: "https://eldenring.wiki.fextralife.com/Morgott+the+Omen+King",
        }],
    },
    Guide {
        appid: ER,
        matches: &["great rune"],
        missable: false,
        rules: &[
            "Restore any shardbearer's Great Rune at its Divine Tower (each boss has its own tower — e.g. Godrick's is across the bridge east of Stormveil), then equip it at a site of grace and activate it with a Rune Arc.",
        ],
        locations: &["Divine Towers — Limgrave, Caelid, Liurnia, Leyndell, East Altus"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Great Runes",
            url: "https://eldenring.wiki.fextralife.com/Great+Runes",
        }],
    },
    // ------------------------------------------------------------------
    // DARK SOULS: REMASTERED — covenants
    // ------------------------------------------------------------------
    Guide {
        appid: DS1,
        matches: &["blade of the darkmoon"],
        missable: true,
        rules: &[
            "Join in the Darkmoon Tomb: from the Anor Londo bonfire room, the rotating staircase leads DOWN to a sealed door — equip the Darkmoon Seance Ring (Catacombs illusory wall) to open it, then pray at the covenant sign.",
            "Do NOT kill Gwyndolin before joining — that locks the covenant for the playthrough.",
        ],
        locations: &[
            "The Catacombs — Darkmoon Seance Ring (illusory wall near the second bonfire)",
            "Anor Londo — Darkmoon Tomb below the rotating staircase",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Blade of the Darkmoon",
            url: "https://darksouls.wiki.fextralife.com/Blade+of+the+Dark+Moon",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["path of the dragon"],
        missable: false,
        rules: &[
            "Join by approaching the Everlasting Dragon at the end of Ash Lake — a long optional trek: Blighttown swamp → Great Hollow (hidden behind illusory walls in a chest room) → Ash Lake.",
            "Don't attack the Stone Dragon; it's the covenant leader.",
        ],
        locations: &["Blighttown → The Great Hollow → Ash Lake (far end)"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Path of the Dragon",
            url: "https://darksouls.wiki.fextralife.com/Path+of+the+Dragon",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["forest hunter"],
        missable: true,
        rules: &[
            "Join by speaking to Alvina (the giant cat) in Darkroot Garden — behind the door sealed by the Crest of Artorias (20,000 souls from Andre of Astora).",
            "Attacking the forest NPCs or Alvina before joining can void the covenant — talk first, fight never.",
        ],
        locations: &[
            "Undead Parish — buy the Crest of Artorias from Andre",
            "Darkroot Garden — Alvina's window, past the sealed door",
        ],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Forest Hunter covenant",
            url: "https://darksouls.wiki.fextralife.com/Forest+Hunter",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["chaos servant"],
        missable: false,
        rules: &[
            "Join through the illusory wall behind Quelaag's Domain (talk to Eingyi, answer 'yes'): the Fair Lady (Quelaag's sister) accepts humanity offerings.",
            "Offering 30 humanity opens the Lost Izalith shortcut — which is also how you save Solaire from the Sunlight Maggot if you care about his questline.",
        ],
        locations: &["Quelaag's Domain — illusory wall behind the boss arena"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Chaos Servant covenant",
            url: "https://darksouls.wiki.fextralife.com/Chaos+Servant",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["gravelord"],
        missable: false,
        rules: &[
            "Join by lying in the empty coffin in the Catacombs (near the second bonfire, behind the wheel skeletons) while holding at least one Eye of Death — Nito inducts you.",
            "You need an Eye of Death first: three sit in a Catacombs corpse pile near the Titanite Demon.",
        ],
        locations: &["The Catacombs — coffin nook before Nito's domain"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Gravelord Servant covenant",
            url: "https://darksouls.wiki.fextralife.com/Gravelord+Servant",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["princess"],
        missable: true,
        rules: &[
            "Join by speaking to Gwynevere in Anor Londo after defeating Ornstein & Smough.",
            "Killing Gwynevere (which darkens Anor Londo permanently) locks the covenant for the playthrough — join before any dark deeds.",
        ],
        locations: &["Anor Londo — Gwynevere's chamber, past O&S"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Princess's Guard covenant",
            url: "https://darksouls.wiki.fextralife.com/Princess+Guard",
        }],
    },
    Guide {
        appid: DS1,
        matches: &["warrior of sunlight"],
        missable: false,
        rules: &[
            "Pray at the Sunlight Altar (broken statue on the bridge below the Hellkite Drake, Undead Parish side) with 25 Faith — the requirement drops by 5 for each co-op session you've completed as a summon.",
        ],
        locations: &["Undead Parish — Sunlight Altar under the drake bridge"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Warrior of Sunlight covenant",
            url: "https://darksouls.wiki.fextralife.com/Warrior+of+Sunlight",
        }],
    },
    // ------------------------------------------------------------------
    // DARK SOULS III — covenants
    // ------------------------------------------------------------------
    Guide {
        appid: DS3,
        matches: &["blades of the darkmoon"],
        missable: true,
        rules: &[
            "Join via Yorshka, hidden above Anor Londo: from the top of the rotating tower mechanism, walk the INVISIBLE bridge from the broken walkway toward the prison tower.",
            "Don't attack or kill Yorshka — that locks the covenant for the playthrough.",
        ],
        locations: &["Anor Londo — invisible walkway from the rotating tower to Prison Tower"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Blades of the Darkmoon (DS3)",
            url: "https://darksouls3.wiki.fextralife.com/Blades+of+the+Darkmoon",
        }],
    },
    Guide {
        appid: DS3,
        matches: &["aldrich faithful"],
        missable: false,
        rules: &[
            "Join via Archdeacon McDonnell, behind an illusory wall in Irithyll's Water Reserve: after Pontiff Sulyvahn, take the stairs down near the two Sulyvahn beasts — the wall is at the back of the room.",
        ],
        locations: &["Irithyll of the Boreal Valley — Water Reserve (post-Pontiff), illusory wall"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Aldrich Faithful",
            url: "https://darksouls3.wiki.fextralife.com/Aldrich+Faithful",
        }],
    },
    Guide {
        appid: DS3,
        matches: &["mound-makers", "mound makers"],
        missable: true,
        rules: &[
            "Easiest join: BEFORE defeating the Curse-Rotted Greatwood, find Hodrick in the spiked cage carried by the hollow manservant northwest of the Undead Settlement's central square — climb in.",
            "If the Greatwood is already dead, the only remaining path is through Sirris' questline (answer her summons at Keep Ruins) — slower and easy to break.",
        ],
        locations: &["Undead Settlement — cage-carrier hollow near the central plaza"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Mound-Makers",
            url: "https://darksouls3.wiki.fextralife.com/Mound-Makers",
        }],
    },
    Guide {
        appid: DS3,
        matches: &["rosaria's fingers"],
        missable: true,
        rules: &[
            "Join at Rosaria's Bed Chamber in the Cathedral of the Deep (past the giant slugs, up from the Deacons area).",
            "Quest-conflict warning: offering Pale Tongues fails Sirris' questline. Joining alone is safe — you can join, get the achievement, and never rank up.",
            "Leonhard's questline ends with Rosaria's soul being taken — join before finishing his quest.",
        ],
        locations: &["Cathedral of the Deep — Rosaria's Bed Chamber"],
        phase: "NG",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Rosaria's Fingers",
            url: "https://darksouls3.wiki.fextralife.com/Rosaria's+Fingers",
        }],
    },
    // ------------------------------------------------------------------
    // SEKIRO — stat cap
    // ------------------------------------------------------------------
    Guide {
        appid: SEKIRO,
        matches: &["peak physical strength"],
        missable: false,
        rules: &[
            "Max Vitality and Posture = all 40 Prayer Beads (10 necklaces). Most come from minibosses — many optional and easy to walk past — plus a few purchases and pickups.",
            "Miss any and they carry over: NG+ repeats bead sources, so nothing is permanently lost, but a per-area checklist saves you a full extra playthrough.",
        ],
        locations: &["Everywhere — miniboss checklist per area on the wiki"],
        phase: "NG+",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — Prayer Bead locations",
            url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Prayer+Beads",
        }],
    },
    // ------------------------------------------------------------------
    // ARMORED CORE VI — arena & data logs
    // ------------------------------------------------------------------
    Guide {
        appid: AC6,
        matches: &["arena"],
        missable: false,
        rules: &[
            "New Arena entrants unlock as the story progresses — and more appear in NG+ and NG++. Sweep the Arena tab every chapter; completion also feeds OS Tuning chips and parts money.",
        ],
        locations: &["ALLMIND Arena tab — check after every chapter, all three playthroughs"],
        phase: "NG++",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — AC6 Arena",
            url: "https://armoredcore6.wiki.fextralife.com/Arena",
        }],
    },
    Guide {
        appid: AC6,
        matches: &["data log"],
        missable: false,
        rules: &[
            "Data logs hide inside mission maps (often off the objective path) and some only exist in NG+/NG++ mission variants. Replay missions freely — collection is cumulative.",
        ],
        locations: &["Hidden containers across missions — per-mission list on the wiki"],
        phase: "NG++",
        checklist: &[],
        resources: &[Resource {
            label: "Fextralife — AC6 log locations (combat & data)",
            url: "https://armoredcore6.wiki.fextralife.com/Combat+Logs",
        }],
    },
];
