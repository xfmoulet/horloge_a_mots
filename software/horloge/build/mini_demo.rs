// Mini Panel (4x2) knightrider mode ------------------------------------------------------------------------------------

#[cfg(feature = "mini_demo")]
// used for placement
pub static LED_PANEL: [&str;6] = [
"x1 x2",
"x3 x4",
"L1 L2", 
"L3 L4", 
"L5 L6", 
"L7 L8"
];

#[cfg(feature = "mini_demo")]
pub static LED_DURATIONS: [(&str, usize); 8] = [
    ("L1", 1),
    ("L2", 4),
    ("L3", 1),
    ("L4", 1),
    ("L5", 1),
    ("L6", 1),
    ("L7", 1),
    ("L8", 1),
];

// TODO try other durations patterns to reach 16 Max ?
// Correspondence bewteen 0-23 hour and corresponding LEDs to illuminate
#[cfg(feature = "mini_demo")]
pub static HOURS_LED: [&str; 0] = [];

// Correspondence between 0-11 5-minutes packs and LEDs
#[cfg(feature = "mini_demo")]
pub static MINUTES_5_LED: [&str; 12] = [
    "L1",
    "L3",
    "L5",
    "L7",
    "L8",
    "L6",
    "L4",
    "L2",
    "L1 L3 L5 L7",
    "L1 L4 L5 L8",
    "L2 L4 L6 L8",
    "L2 L3 L6 L7",
];

// Correspondence between 0-5 remaining minute and LEDs
#[cfg(feature = "mini_demo")]
pub static MINUTES_LED: [&str; 0] = [];
