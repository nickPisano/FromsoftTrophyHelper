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
        resources: &[
            Resource { label: "Fextralife — Ranni the Witch full questline", url: "https://eldenring.wiki.fextralife.com/Ranni+the+Witch" },
            Resource { label: "Fextralife — Endings overview", url: "https://eldenring.wiki.fextralife.com/Endings" },
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
        resources: &[
            Resource { label: "Fextralife — Lord of Frenzied Flame ending", url: "https://eldenring.wiki.fextralife.com/Endings" },
            Resource { label: "Fextralife — Millicent questline (for Miquella's Needle)", url: "https://eldenring.wiki.fextralife.com/Millicent" },
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
        locations: &[
            "Leyndell, Ashen Capital — Elden Throne, after Radagon / Elden Beast",
        ],
        resources: &[
            Resource { label: "Fextralife — Endings overview", url: "https://eldenring.wiki.fextralife.com/Endings" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Legendary Armaments checklist", url: "https://eldenring.wiki.fextralife.com/Legendary+Armaments" },
        ],
    },
    Guide {
        appid: ER,
        matches: &["legendary sorceries", "legendary incantations", "legendary sorceries and incantations"],
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
        resources: &[
            Resource { label: "Fextralife — Legendary Sorceries and Incantations", url: "https://eldenring.wiki.fextralife.com/Legendary+Sorceries+and+Incantations" },
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
        resources: &[
            Resource { label: "Fextralife — Legendary Talismans checklist", url: "https://eldenring.wiki.fextralife.com/Legendary+Talismans" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Legendary Ashen Remains checklist", url: "https://eldenring.wiki.fextralife.com/Legendary+Ashen+Remains" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Mohg, Lord of Blood", url: "https://eldenring.wiki.fextralife.com/Mohg,+Lord+of+Blood" },
            Resource { label: "Fextralife — White Mask Varre questline", url: "https://eldenring.wiki.fextralife.com/White-Faced+Varre" },
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
        resources: &[
            Resource { label: "Fextralife — Malenia, Blade of Miquella", url: "https://eldenring.wiki.fextralife.com/Malenia+Blade+of+Miquella" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Yuria of Londor questline", url: "https://darksouls3.wiki.fextralife.com/Yuria+of+Londor" },
            Resource { label: "Fextralife — Endings", url: "https://darksouls3.wiki.fextralife.com/Endings" },
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
        resources: &[
            Resource { label: "Fextralife — Endings", url: "https://darksouls3.wiki.fextralife.com/Endings" },
            Resource { label: "Fextralife — Untended Graves", url: "https://darksouls3.wiki.fextralife.com/Untended+Graves" },
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
        resources: &[
            Resource { label: "Fextralife — Rings list (with NG+ locations)", url: "https://darksouls3.wiki.fextralife.com/Rings" },
        ],
    },
    Guide {
        appid: DS3,
        matches: &["master of miracles"],
        missable: true,
        rules: &[
            "Buy out Irina's stock from every Braille Divine Tome, and keep her questline on the 'good' path (don't feed her the dark tomes — give those to Karla).",
            "Covenant rewards are required: 10 + 30 Sunlight Medals (Warrior of Sunlight) and 10 + 30 Proofs of a Concord Kept (Blades of the Darkmoon). Both farm offline — Lothric Knights (medals) and Silver Knights in Anor Londo (proofs).",
            "Don't kill quest NPCs who teach miracles before you've bought everything.",
        ],
        locations: &[
            "Undead Settlement — free Irina (Tower Key not required, sewer route)",
            "Anor Londo — Silver Knight farming for Proofs of a Concord Kept",
            "Lothric Castle — Sunlight Medal farming",
            "Braille tomes — Farron Keep swamp, Carthus Catacombs, and beyond",
        ],
        resources: &[
            Resource { label: "Fextralife — Miracles list", url: "https://darksouls3.wiki.fextralife.com/Miracles" },
        ],
    },
    Guide {
        appid: DS3,
        matches: &["master of sorceries"],
        missable: true,
        rules: &[
            "Orbeck of the Vinheim must survive the whole quest: give him at least one scroll early or he leaves Firelink for good.",
            "Bring him all four scrolls (Sage's, Golden, Logan's, Crystal) and buy every sorcery he sells.",
            "A few sorceries come from elsewhere (e.g., Karla's dark sorceries) — check the list before finishing the game.",
        ],
        locations: &[
            "Road of Sacrifices, Crucifixion Woods (crab lake) — Sage's Scroll",
            "Farron Keep — Golden Scroll (dark cave)",
            "Smouldering Lake — Logan's Scroll (demon cleric on the ridge)",
            "Grand Archives — Crystal Scroll",
        ],
        resources: &[
            Resource { label: "Fextralife — Orbeck of Vinheim", url: "https://darksouls3.wiki.fextralife.com/Orbeck+of+Vinheim" },
            Resource { label: "Fextralife — Sorceries list", url: "https://darksouls3.wiki.fextralife.com/Sorceries" },
        ],
    },
    Guide {
        appid: DS3,
        matches: &["master of pyromancies"],
        missable: true,
        rules: &[
            "Pyromancy tomes split between two teachers: Cornyx takes the standard tomes, while Karla is the only one who will accept the dark ones (Grave Warden Pyromancy Tome) — check the list so you hand each tome to the right teacher.",
            "Keep both Cornyx and Karla alive until you've bought everything.",
            "A few pyromancies drop in the world or from covenant farming (Wolf's Blood Swordgrass ranks for Watchdogs of Farron reward pyromancy).",
        ],
        locations: &[
            "Undead Settlement — free Cornyx (cage)",
            "Irithyll Dungeon — free Karla (cell key from the Jailbreaker's area)",
            "Tomes — Undead Settlement, Catacombs of Carthus, Smouldering Lake, Grand Archives",
        ],
        resources: &[
            Resource { label: "Fextralife — Pyromancies list (who gets which tome)", url: "https://darksouls3.wiki.fextralife.com/Pyromancies" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Estus Shard locations", url: "https://darksouls3.wiki.fextralife.com/Estus+Shard" },
            Resource { label: "Fextralife — Undead Bone Shard locations", url: "https://darksouls3.wiki.fextralife.com/Undead+Bone+Shard" },
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
        locations: &[
            "Ashina Castle rooftop — the choice with Great Shinobi Owl",
        ],
        resources: &[
            Resource { label: "Fextralife — Sekiro endings", url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Endings" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Purification ending walkthrough", url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Endings" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Return / Dragon's Homecoming ending", url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Endings" },
        ],
    },
    Guide {
        appid: SEKIRO,
        matches: &["man without equal"],
        missable: true,
        rules: &[
            "Defeat every boss in the game — including the Shura-route exclusives (Emma & Isshin Ashina) AND the normal-route finale (Isshin, the Sword Saint / Divine Dragon). Impossible in one run without a save backup: plan two playthroughs minimum.",
            "Includes all optional bosses: Demon of Hatred, Great Shinobi Owl, Owl (Father), Headless Ape rematch, and both Hirata memories.",
        ],
        locations: &[
            "Everywhere — use a full boss checklist and tick them per playthrough",
        ],
        resources: &[
            Resource { label: "Fextralife — Bosses checklist", url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Bosses" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Skills & esoteric texts", url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Skills" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Prosthetic tools & upgrades", url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Prosthetic+Tools" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — AC6 endings & decision missions", url: "https://armoredcore6.wiki.fextralife.com/Endings" },
        ],
    },
    Guide {
        appid: AC6,
        matches: &["liberator of rubicon"],
        missable: true,
        rules: &[
            "The counterpart branch to Fires of Raven — side with Ayre when the decision missions split. Take whichever you skipped on your other run via NG+.",
        ],
        locations: &["Chapter 4-5 decision missions — pick the 'Liberator of Rubicon' branch"],
        resources: &[
            Resource { label: "Fextralife — AC6 endings & decision missions", url: "https://armoredcore6.wiki.fextralife.com/Endings" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — True ending (NG++) route", url: "https://armoredcore6.wiki.fextralife.com/Endings" },
            Resource { label: "PowerPyx — AC6 all endings guide", url: "https://www.powerpyx.com/armored-core-6-fires-of-rubicon-trophy-guide-roadmap/" },
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
        resources: &[
            Resource { label: "Fextralife — Parts & where each comes from", url: "https://armoredcore6.wiki.fextralife.com/Parts" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Combat Logs per mission", url: "https://armoredcore6.wiki.fextralife.com/Combat+Logs" },
        ],
    },
    // ------------------------------------------------------------------
    // DARK SOULS: REMASTERED
    // ------------------------------------------------------------------
    Guide {
        appid: DS1,
        matches: &["knight's honor"],
        missable: true,
        rules: &[
            "Collect every rare weapon — the true long-haul achievement. Several boss souls forge MULTIPLE weapons (Sif's and Quelaag's especially), so you need repeat souls from NG+ (and a third Sif soul in NG++ unless you trade with another player).",
            "Cut tails on first meeting: Priscilla's Dagger, Moonlight Greatsword (Seath's blue tail — he can't be re-tailed after death), Gargoyle Tail Axe, Guardian Tail, Obsidian Greatsword (Kalameet, DLC).",
            "Don't break the Crystal Halberd chest run in Anor Londo, and remember the Black Knight weapon drops are RNG — farm the Kiln knights if unlucky.",
        ],
        locations: &[
            "Boss soul weapons — Giant Blacksmith (Anor Londo) ascension",
            "Tail cuts — Painted World (Priscilla), Duke's Archives (Seath), Undead Parish/Anor Londo (Gargoyles), Sanctuary Garden & Royal Wood (DLC)",
            "Full 49-weapon checklist on the wiki",
        ],
        resources: &[
            Resource { label: "Fextralife — Knight's Honor weapon checklist", url: "https://darksouls.wiki.fextralife.com/Knight's+Honor" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — Darkwraith covenant", url: "https://darksouls.wiki.fextralife.com/Darkwraith" },
        ],
    },
    Guide {
        appid: DS1,
        matches: &["wisdom of a sage"],
        missable: true,
        rules: &[
            "All sorceries. Big Hat Logan's questline is the trap: free him in Sen's Fortress (cell key on the upper walkway), buy his stock at Firelink, free him AGAIN in the Duke's Archives, buy the rest — then after he goes hollow, defeat him for White Dragon Breath.",
            "If Logan dies or you skip a step, several sorceries are gone until NG+.",
            "Also buy out Griggs and Dusk of Oolacile (rescue her golem in Darkroot Basin; don't kill her).",
        ],
        locations: &[
            "Sen's Fortress — Logan's cage",
            "Firelink Shrine — Logan & Griggs vendors",
            "The Duke's Archives — Logan's second cell + hollow Logan (Seath's arena)",
            "Darkroot Basin — Dusk (Crystal Golem rescue)",
        ],
        resources: &[
            Resource { label: "Fextralife — Sorceries checklist", url: "https://darksouls.wiki.fextralife.com/Sorceries" },
            Resource { label: "Fextralife — Big Hat Logan questline", url: "https://darksouls.wiki.fextralife.com/Big+Hat+Logan" },
        ],
    },
    Guide {
        appid: DS1,
        matches: &["bond of a pyromancer"],
        missable: true,
        rules: &[
            "All pyromancies. Rescue Laurentius in the Depths (roll into the barrel — don't hit it); later, if he asks about your pyromancy, DON'T tell him where you learned advanced flames or he leaves and hollows.",
            "Quelana appears in Blighttown swamp once you carry a pyromancy flame (upgraded); buy everything and complete her task for Fire Tempest.",
            "Eingyi (behind the illusory wall near Quelaag's domain) sells egg-related pyromancies — you may need the egghead status to trade with him for one of them.",
            "Chaos pyromancies come from the Chaos Servant covenant shop.",
        ],
        locations: &[
            "The Depths — Laurentius rescue",
            "Blighttown swamp — Quelana of Izalith",
            "Quelaag's Domain — Eingyi + Chaos Servant covenant",
        ],
        resources: &[
            Resource { label: "Fextralife — Pyromancies checklist", url: "https://darksouls.wiki.fextralife.com/Pyromancies" },
        ],
    },
    Guide {
        appid: DS1,
        matches: &["prayer of a maiden"],
        missable: true,
        rules: &[
            "All miracles. Reah of Thorolund's questline is missable: rescue her party in the Tomb of the Giants, buy ALL her miracles at the Undead Parish before she moves to the Duke's Archives (where she can die).",
            "Covenant miracles: Great Lightning Spear (Warrior of Sunlight rank — 10 Sunlight Medals), Darkmoon Blade (Blade of the Darkmoon rank — 10 Souvenirs of Reprisal), Gravelord miracles from Nito's covenant.",
            "A few one-off miracles hide in late zones (Tomb of the Giants, Duke's Archives) — follow the checklist so you don't finish the game one pickup short.",
        ],
        locations: &[
            "Tomb of the Giants — Reah's rescue + late pickups",
            "Undead Parish — Reah's vendor window",
            "Covenants — Sunlight altar (Undead Parish), Darkmoon (Anor Londo), Gravelord (Catacombs)",
        ],
        resources: &[
            Resource { label: "Fextralife — Miracles checklist", url: "https://darksouls.wiki.fextralife.com/Miracles" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — DS2 Sorceries checklist", url: "https://darksouls2.wiki.fextralife.com/Sorceries" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — DS2 Miracles checklist", url: "https://darksouls2.wiki.fextralife.com/Miracles" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — DS2 Pyromancies checklist", url: "https://darksouls2.wiki.fextralife.com/Pyromancies" },
        ],
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
        resources: &[
            Resource { label: "Fextralife — DS2 Hexes checklist", url: "https://darksouls2.wiki.fextralife.com/Hexes" },
            Resource { label: "Fextralife — Darklurker", url: "https://darksouls2.wiki.fextralife.com/Darklurker" },
        ],
    },
];
