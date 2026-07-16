//! The supported FromSoftware catalog on Steam, plus game-wide notes and
//! hand-picked 100% resources that apply to every achievement in the game.

use crate::guides::Resource;

pub struct Game {
    pub appid: u32,
    pub title: &'static str,
    /// Playthrough-wide rules everyone chasing 100% should know up front.
    pub notes: &'static [&'static str],
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
