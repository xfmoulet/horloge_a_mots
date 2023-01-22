
// Big Panel ------------------------------------------------------------------------------------

// used for placement
pub static LED_PANEL: [&str; 6] = [
    "Mi Quatre Trois Il Est Deux",
    "Huit Six Cinq Neuf Une Sept",
    "Onze Heure S Di X Minuit",
    "DixMin Le Et CinqMin Moins Vingt",
    "Et2 Des Pile Quart Demi E",
    "xx Bananes Dot1 Dot2 Dot3 Dot4",
];

pub static LED_DURATIONS: [(&str, usize); 34] = [
    ("Il", 2),
    ("Est", 2),
    ("Une", 1),
    ("Deux", 2),
    ("Trois", 3),
    ("Quatre", 3),
    ("Cinq", 2),
    ("Six", 2),
    ("Sept", 2),
    ("Huit", 2),
    ("Neuf", 2),
    ("Onze", 2),
    ("Mi", 2),
    ("Di", 2),
    ("X", 1),
    ("Minuit", 3),
    ("Heure", 3),
    ("S", 1),
    ("Et", 1),
    ("Moins", 2),
    ("DixMin", 1),
    ("Vingt", 2),
    ("CinqMin", 1),
    ("Le", 1),
    ("Quart", 2),
    ("Demi", 3),
    ("E", 1),
    ("Et2", 1),
    ("Des", 1),
    ("Bananes", 1),
    ("Dot1", 1),
    ("Dot2", 1),
    ("Dot3", 1),
    ("Dot4", 1),
];

// TODO try other durations patterns to reach 16 Max ?
// Correspondence bewteen 0-23 hour and corresponding LEDs to illuminate
pub static HOURS_LED: [&str; 24] = [
    "Il Est Minuit",
    "Il Est Une Heure",
    "Il Est Deux Heure S",
    "Il Est Trois Heure S",
    "Il Est Quatre Heure S",
    "Il Est Cinq Heure S",
    "Il Est Six Heure S",
    "Il Est Sept Heure S",
    "Il Est Huit Heure S",
    "Il Est Neuf Heure S",
    "Il Est Di X Heure S",
    "Il Est Onze Heure S",
    "Il Est Mi Di",
    "Il Est Une Heure",
    "Il Est Deux Heure S",
    "Il Est Trois Heure S",
    "Il Est Quatre Heure S",
    "Il Est Cinq Heure S",
    "Il Est Six Heure S",
    "Il Est Sept Heure S",
    "Il Est Huit Heure S",
    "Il Est Neuf Heure S",
    "Il Est Di X Heure S",
    "Il Est Onze Heure S",
];

// Correspondence between 0-11 5-minutes packs and LEDs
pub static MINUTES_5_LED: [&str; 12] = [
    "",
    "CinqMin",
    "DixMin",
    "Et Quart",
    "Vingt",
    "Vingt CinqMin",
    "Et Demi E",
    "Moins Vingt CinqMin",
    "Moins Vingt",
    "Moins Le Quart",
    "Moins DixMin",
    "Moins CinqMin",
];

// Correspondence between 0-5 remaining minute and LEDs
pub static MINUTES_LED: [&str; 5] = [
    "",
    "Dot1",
    "Dot2",
    "Et2 Des Bananes Dot3",
    "Et2 Des Bananes Dot4",
];
