// Mini Panel 4x2 minutes mode ------------------------------------------------------------------------------------

#[cfg(feature = "mini_panel")]
// used for placement
static LED_PANEL: [&str; 6] = [
    "x1 x2", // unused
    "x3 x4", // unused
    "Moins Le",
    "Et Quart",
    "Demi Vingt",
    "DixMin CinqMin",
];

#[cfg(feature = "mini_panel")]
static LED_DURATIONS: [(&str, usize); 8] = [
    ("Moins", 1),
    ("Et", 1),
    ("DixMin", 1),
    ("Vingt", 1),
    ("CinqMin", 1),
    ("Le", 1),
    ("Quart", 1),
    ("Demi", 1),
];

// Correspondence bewteen 0-23 hour and corresponding LEDs to illuminate
#[cfg(feature = "mini_panel")]
static HOURS_LED: [&str; 0] = [];

// Correspondence between 0-11 5-minutes packs and LEDs
#[cfg(feature = "mini_panel")]
static MINUTES_5_LED: [&str; 12] = [
    "",
    "CinqMin",
    "DixMin",
    "Et Quart",
    "Vingt",
    "Vingt CinqMin",
    "Et Demi",
    "Moins Vingt CinqMin",
    "Moins Vingt",
    "Moins Le Quart",
    "Moins DixMin",
    "Moins CinqMin",
];

// Correspondence between 0-5 remaining minute and LEDs
#[cfg(feature = "mini_panel")]
static MINUTES_LED: [&str; 0] = [
];
