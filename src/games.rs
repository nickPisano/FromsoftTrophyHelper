//! The supported FromSoftware catalog on Steam, plus game-wide notes and
//! hand-picked 100% resources that apply to every achievement in the game.

use crate::guides::Resource;

pub struct Game {
    pub appid: u32,
    pub title: &'static str,
    /// Playthrough-wide rules everyone chasing 100% should know up front.
    pub notes: &'static [&'static str],
    /// How the game's systems interact with the hunt: NG+ carryover,
    /// save-backup how-to with real paths, stat requirements.
    pub mechanics: &'static [&'static str],
    /// Established farming spots and covenant grind numbers.
    pub farming: &'static [&'static str],
    pub resources: &'static [Resource],
}

pub const GAMES: &[Game] = &[
    Game {
        appid: 1245620,
        title: "ELDEN RING",
        notes: &[
            "Nothing is lost forever — anything you miss can be picked up on the next New Game+ cycle. Within a single playthrough, though, two things can slip away: the Bolt of Gransax (grab it in Leyndell BEFORE defeating Maliketh — the city becomes the Ashen Capital afterwards and the spear is gone for that cycle), and your choice of ending.",
            "There are three ending achievements (Elden Lord, Age of the Stars, Lord of Frenzied Flame). To get all three in one run, back up your save folder before touching the final boss' remembrance choices, then reload and pick each ending. Otherwise plan on multiple NG cycles.",
            "Accepting the Three Fingers' Frenzied Flame locks you into that ending unless you complete Millicent's questline and use Miquella's Needle in Dragonlord Placidusax's arena. Safest order: do it last, or after a save backup.",
            "Achievement progress is account-wide and pops the moment the condition is met, even in NG+.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy %APPDATA%\\EldenRing\\<your SteamID>\\ (file ER0000.sl2). On Steam Deck/Linux it's in steamapps/compatdata/1245620/pfx/drive_c/users/steamuser/AppData/Roaming/EldenRing/. Restore by copying it back.",
            "NG+ carries over: your character, all items/weapons/spells/talismans, map progress, and spirit ashes. Reset each cycle: great runes, quest states, world bosses — so questline spells/weapons can be re-earned.",
            "Steam Cloud can overwrite a restored save — disable cloud sync for Elden Ring (game Properties) while doing save-backup ending runs.",
            "Legendary Sorceries needs vendors/quests intact (Sellen, Ranni); Greyoll's Roar needs 3 Dragon Hearts banked.",
        ],
        farming: &[
            "Runes: Mohgwyn Palace — from the Palace Approach Ledge-Road grace, shoot the giant crow across the blood lake or clear the sleeping albinaurics (~40-60k per minute lap late-game).",
            "Early runes: take the Third Church of Marika waygate to Greyoll's Dragonbarrow and whittle down sleeping Elder Dragon Greyoll from her tail — a huge one-time payout available almost immediately.",
            "Smithing stones: unlock the bell bearings (Raya Lucaria Crystal Tunnel, Sealed Tunnel, Zamor Ruins, Farum Azula) so the Twin Maiden Husks sell every tier.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Elden Ring Wiki (interactive map, checklists)",
                url: "https://eldenring.wiki.fextralife.com/Elden+Ring+Wiki",
            },
            Resource {
                label: "PowerPyx — Elden Ring 100% Achievement Guide & Roadmap",
                url: "https://www.powerpyx.com/elden-ring-trophy-guide-roadmap/",
            },
            Resource {
                label: "Steam Community guides for Elden Ring",
                url: "https://steamcommunity.com/app/1245620/guides/",
            },
        ],
    },
    Game {
        appid: 374320,
        title: "DARK SOULS III",
        notes: &[
            "Plan for three playthroughs (NG, NG+, NG++): Master of Rings requires the +1 and +2 ring variants that only exist in NG+ and NG++.",
            "The Usurpation of Fire ending has the strictest questline in the game (Yoel/Yuria + Anri). Read its rules BEFORE leveling up at the shrine handmaid — healing the Dark Sigil at any point voids it for the run.",
            "Back up your save before the final boss to collect multiple ending achievements in one cycle.",
            "Covenant spell/item rewards (Sunlight Medals, Proofs of a Concord Kept, Wolf's Blood Swordgrass) count toward the spell achievements and can all be farmed offline from specific enemies.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy %APPDATA%\\DarkSoulsIII\\<your SteamID>\\ (file DS30000.sl2). Steam Deck/Linux: steamapps/compatdata/374320/pfx/drive_c/users/steamuser/AppData/Roaming/DarkSoulsIII/. Disable Steam Cloud for the game while juggling ending saves.",
            "NG+ carries over everything except bonfires, keys, and quest states — rings you own persist, and the +1/+2 ring variants spawn fresh in NG+/NG++ locations.",
            "Spell-school achievements pop the moment the last spell enters your inventory — attunement stats don't matter for the achievement, only for casting.",
            "Master of Infusion needs every coal delivered to Andre, then one infusion of each gem type — a single cheap weapon re-infused over and over works; you just need one of each gem.",
        ],
        farming: &[
            "Proofs of a Concord Kept: the two respawning Silver Knights at the Anor Londo stairs — Covetous Gold Serpent Ring + Crystal Sage's Rapier boost discovery (~10-13% drop).",
            "Sunlight Medals: the three Lothric Knights just below the Grand Archives bonfire drop them directly.",
            "Wolf's Blood Swordgrass: the Ghru pack outside Farron Keep Perimeter bonfire.",
            "Vertebra Shackles (30 for Warmth): the slowest grind offline — Mound-Maker invasion/co-op wins are the intended source, so budget real time here or arrange trades.",
            "Souls: Grand Archives' three Winged Knights (drop ~25k with avarice gear) via the roof shortcut.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Dark Souls III Wiki",
                url: "https://darksouls3.wiki.fextralife.com/Dark+Souls+3+Wiki",
            },
            Resource {
                label: "Dark Souls III Wikidot (clean checklists)",
                url: "http://darksouls3.wikidot.com/",
            },
            Resource {
                label: "Steam Community guides for Dark Souls III",
                url: "https://steamcommunity.com/app/374320/guides/",
            },
        ],
    },
    Game {
        appid: 814380,
        title: "Sekiro: Shadows Die Twice",
        notes: &[
            "There are four endings. The Shura ending ends the game early and locks out the other three for that playthrough — most players save Shura for a short second run, or back up their save before the choice on the Ashina Castle rooftop.",
            "'Man Without Equal' needs bosses from BOTH the Shura route and the normal route, so it takes at least two playthroughs (or a save backup).",
            "Learning every skill takes far more XP than one playthrough provides — expect to chain NG+ cycles or farm late-game areas.",
            "Save backups are the accepted way to shorten the road to 100%: copy the save folder before each point of no return.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy %APPDATA%\\Sekiro\\<your SteamID>\\ (S0000.sl2). Steam Deck/Linux: steamapps/compatdata/814380/pfx/drive_c/users/steamuser/AppData/Roaming/Sekiro/.",
            "NG+ stacks: prayer beads, skills, prosthetics, and money/XP progress all carry forward, so grind achievements finish naturally across cycles.",
        ],
        farming: &[
            "XP/sen late-game: the Fountainhead Palace Okami warriors near the Great Sakura, with both bell-demon off and Kuro's Charm intact for speed.",
            "Divine Confetti and treasure carp scales are limited per cycle — check the wiki lists before spending.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Sekiro Wiki",
                url: "https://sekiroshadowsdietwice.wiki.fextralife.com/Sekiro+Shadows+Die+Twice+Wiki",
            },
            Resource {
                label: "PowerPyx — Sekiro 100% Guide & Roadmap",
                url: "https://www.powerpyx.com/sekiro-shadows-die-twice-trophy-guide-roadmap/",
            },
            Resource {
                label: "Steam Community guides for Sekiro",
                url: "https://steamcommunity.com/app/814380/guides/",
            },
        ],
    },
    Game {
        appid: 1888160,
        title: "ARMORED CORE VI: FIRES OF RUBICON",
        notes: &[
            "The three story endings require three playthroughs: NG and NG+ each unlock new decision missions, and the third ending (Alea Iacta Est) only becomes available in NG++.",
            "Collecting all parts and all combat/battle logs also spans all three playthroughs — several missions and log-carrying enemies only appear in NG+ and NG++ mission variants.",
            "Nothing is permanently missable: you can replay any mission from the mission select once cleared.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy %APPDATA%\\ArmoredCore6\\<your SteamID>\\. Steam Deck/Linux: steamapps/compatdata/1888160/pfx/drive_c/users/steamuser/AppData/Roaming/ArmoredCore6/.",
            "NG+/NG++ keep all parts, money, and OS tuning — only the mission list resets, with new decision missions inserted.",
        ],
        farming: &[
            "COAM (money): replay high-payout missions from the sortie list; Arena rematches also pay out and feed OS Tuning chips.",
            "Missed a log or part container? Replay the mission — collection is cumulative across replays.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Armored Core VI Wiki",
                url: "https://armoredcore6.wiki.fextralife.com/",
            },
            Resource {
                label: "Fextralife — AC6 Trophy & Achievement Guide",
                url: "https://armoredcore6.wiki.fextralife.com/Trophy+%26+Achievement+Guide",
            },
            Resource {
                label: "Steam Community guides for Armored Core VI",
                url: "https://steamcommunity.com/app/1888160/guides/",
            },
        ],
    },
    Game {
        appid: 570940,
        title: "DARK SOULS: REMASTERED",
        notes: &[
            "Knight's Honor (all rare weapons) is the long pole: several boss souls each forge more than one weapon, so you need souls from NG+ (and one from NG++) unless you trade with another player. Start a checklist on day one.",
            "Cut tails on your FIRST encounter — five Knight's Honor weapons only come from tail cuts: the Drake Sword (Hellkite), Dragon King Greataxe (Gaping Dragon), Priscilla's Dagger, Moonlight Greatsword (Seath), and Dragon Greatsword (Ash Lake's Stone Dragon). DLC weapons are NOT required.",
            "To join the Darkwraith covenant you must reach Kaathe in the Abyss: defeat the Four Kings BEFORE placing the Lordvessel with Frampt, or Kaathe won't appear that cycle.",
            "Several NPC vendors (Logan, Laurentius, Reah) can die or go hollow — buy out their spell stock as soon as you can afford it.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy Documents\\NBGI\\DARK SOULS REMASTERED\\<numeric folder>\\ (file DRAKS0005.sl2). Steam Deck/Linux: steamapps/compatdata/570940/pfx/drive_c/users/steamuser/Documents/NBGI/DARK SOULS REMASTERED/.",
            "NG+ carries over everything except keys, embers delivered, and quest states — boss-soul weapons you forged persist, and bosses give souls again for the next round of forging.",
            "Knight's Honor and the spell achievements track ONE character (only Prayer of a Maiden is account-cumulative) — pick a main and stick with it.",
            "Weapon-ascension achievements each need the matching ember delivered to the right blacksmith plus a +5/+10/+15 base — the wiki trophy guide lists each ember's location.",
        ],
        farming: &[
            "Item Discovery: hold 10 soft humanity and wear the Covetous Gold Serpent Ring (Sen's Fortress chest) before farming any rare drop — this is the difference between minutes and hours.",
            "Souvenirs of Reprisal (10 for Darkmoon Blade): farm the harpies in the Painted World offline.",
            "Sunlight Medals (10 for Great Lightning Spear): the two respawning Chaos Bugs in Lost Izalith's shortcut tunnel drop them.",
            "Humanity: rats in the Depths drop it; the Chaos Servant shortcut needs 30 total — twin humanities from Undead Burg's female merchant stock help.",
            "Souls: the Painted World or Kiln Black Knights late; the baby skeletons outside Nito's arena early-mid.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Dark Souls Wiki",
                url: "https://darksouls.wiki.fextralife.com/Dark+Souls+Wiki",
            },
            Resource {
                label: "Dark Souls Wikidot (clean checklists)",
                url: "http://darksouls.wikidot.com/",
            },
            Resource {
                label: "Steam Community guides for Dark Souls: Remastered",
                url: "https://steamcommunity.com/app/570940/guides/",
            },
        ],
    },
    Game {
        appid: 211420,
        title: "DARK SOULS: Prepare to Die Edition",
        notes: &[
            "Same achievement list as the Remastered version, so the same rules apply: Knight's Honor needs boss souls across NG+/NG++ (or trades), cut boss tails on first meeting, and reach Kaathe (Four Kings before placing the Lordvessel) for the Darkwraith covenant.",
            "PTDE was delisted from the Steam store in 2018 but still runs for existing owners. Online (GFWL-era) services are gone — every achievement is earnable fully offline.",
            "Buy out spell vendors (Logan, Laurentius, Reah) before their questlines can kill or hollow them.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy Documents\\NBGI\\DarkSouls\\<folder>\\ (DRAKS0005.sl2).",
            "All Remastered mechanics notes apply (single-character tracking, NG+ soul carryover); see the Remastered entry.",
        ],
        farming: &[
            "Same farming routes as Remastered: 10 humanity + Covetous Gold Serpent Ring before any rare-drop farm, Painted World harpies for Souvenirs, Lost Izalith bugs for medals.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Dark Souls Wiki",
                url: "https://darksouls.wiki.fextralife.com/Dark+Souls+Wiki",
            },
            Resource {
                label: "Dark Souls Wikidot (clean checklists)",
                url: "http://darksouls.wikidot.com/",
            },
            Resource {
                label: "Steam Community guides for Prepare to Die Edition",
                url: "https://steamcommunity.com/app/211420/guides/",
            },
        ],
    },
    Game {
        appid: 335300,
        title: "DARK SOULS II: Scholar of the First Sin",
        notes: &[
            "The four 'Master of...' spell achievements need boss-soul spells from Straid and Ornifex — several bosses must be beaten more than once. Burn Bonfire Ascetics to respawn bosses instead of playing into NG++ if you prefer one long playthrough.",
            "Master of Hexes requires the full Pilgrims of Dark questline (Grandahl's three shrines) and defeating the Darklurker.",
            "Covenant rank rewards count toward spell achievements; most covenants can be ranked up offline (Bell Keepers and Rat King via Bonfire Ascetics, Company of Champions by farming).",
            "Nothing is permanently missable thanks to Bonfire Ascetics — but they raise that bonfire's area difficulty permanently for the cycle.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy %APPDATA%\\DarkSoulsII\\<your SteamID>\\ (DS2SOFS0000.sl2). Steam Deck/Linux: steamapps/compatdata/335300/pfx/drive_c/users/steamuser/AppData/Roaming/DarkSoulsII/.",
            "Bonfire Ascetics are portable NG+: burning one respawns that area's boss at +1 intensity — the standard way to re-earn boss souls for Straid/Ornifex spells without a full new cycle.",
            "Stat gates for the spell schools: hexes need 20 INT and 20 FTH before Felkin sells; respec is possible with a Soul Vessel at the Things Betwixt firekeepers.",
            "Company of Champions (join at the Majula obelisk) counts for Covenant of the Fittest and can be joined/left freely.",
        ],
        farming: &[
            "Boss souls: burn a Bonfire Ascetic at the bonfire before the boss — cheaper than NG+ laps for the Master-of-X spells.",
            "Sunlight Medals: offline, medals drop from the Falconers in Things Betwixt after burning an ascetic there.",
            "Souls: the Giant Lord memory run with the Symbol of Avarice and silver serpent rings is the classic (~300k per 2-minute lap late-game).",
            "Ascetics themselves: sold by Chloanne (Stone Trader) once she reaches Majula; also common chest loot.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Dark Souls II Wiki",
                url: "https://darksouls2.wiki.fextralife.com/Dark+Souls+2+Wiki",
            },
            Resource {
                label: "Dark Souls II Wikidot (clean checklists)",
                url: "http://darksouls2.wikidot.com/",
            },
            Resource {
                label: "Steam Community guides for DS2: SOTFS",
                url: "https://steamcommunity.com/app/335300/guides/",
            },
        ],
    },
    Game {
        appid: 236430,
        title: "DARK SOULS II (2014)",
        notes: &[
            "Same achievement list philosophy as Scholar of the First Sin: spell achievements need repeat boss souls (Bonfire Ascetics or NG+/NG++), Master of Hexes needs the Darklurker, and covenant ranks feed the spell lists.",
            "If you own both versions, achievements track separately per version.",
        ],
        mechanics: &[
            "Save backup: with Steam closed, copy %APPDATA%\\DarkSoulsII\\<your SteamID>\\ (DARKSII0000.sl2 in the original).",
            "Bonfire Ascetics work exactly as in SOTFS — portable NG+ for re-earning boss souls; the SOTFS mechanics notes apply here too.",
        ],
        farming: &[
            "Same routes as SOTFS: ascetic the boss bonfires for souls, Giant Lord memory for level souls, Chloanne for ascetic restock.",
        ],
        resources: &[
            Resource {
                label: "Fextralife Dark Souls II Wiki",
                url: "https://darksouls2.wiki.fextralife.com/Dark+Souls+2+Wiki",
            },
            Resource {
                label: "Dark Souls II Wikidot (clean checklists)",
                url: "http://darksouls2.wikidot.com/",
            },
            Resource {
                label: "Steam Community guides for Dark Souls II",
                url: "https://steamcommunity.com/app/236430/guides/",
            },
        ],
    },
];
