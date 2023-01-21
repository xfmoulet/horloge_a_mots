
// Big Panel ------------------------------------------------------------------------------------

// used for placement
static LED_PANEL: [&str; 6] = [
    "Mi Quatre Trois Il Est Deux",
    "Huit Six Cinq Neuf Une Sept",
    "Onze Heure S Di X Minuit",
    "DixMin Le Et CinqMin Moins Vingt",
    "Et2 Des Pile Quart Demi E",
    "xx Bananes Dot1 Dot2 Dot3 Dot4",
];

static LED_DURATIONS: [(&str, usize); 34] = [
    ("Il", 1),
    ("Est", 1),
    ("Une", 1),
    ("Deux", 1),
    ("Trois", 1),
    ("Quatre", 1),
    ("Cinq", 1),
    ("Six", 1),
    ("Sept", 1),
    ("Huit", 1),
    ("Neuf", 1),
    ("Onze", 1),
    ("Mi", 1),
    ("Di", 1),
    ("X", 1),
    ("Minuit", 1),
    ("Heure", 1),
    ("S", 1),
    ("Et", 1),
    ("Moins", 1),
    ("DixMin", 1),
    ("Vingt", 1),
    ("CinqMin", 1),
    ("Le", 1),
    ("Quart", 1),
    ("Demi", 2),
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
static HOURS_LED: [&str; 24] = [
    "Minuit",
    "Une Heure",
    "Deux Heure S",
    "Trois Heure S",
    "Quatre Heure S",
    "Cinq Heure S",
    "Six Heure S",
    "Sept Heure S",
    "Huit Heure S",
    "Neuf Heure S",
    "Di X Heure S",
    "Onze Heure S",
    "Mi Di",
    "Une Heure",
    "Deux Heure S",
    "Trois Heure S",
    "Quatre Heure S",
    "Cinq Heure S",
    "Six Heure S",
    "Sept Heure S",
    "Huit Heure S",
    "Neuf Heure S",
    "Di X Heure S",
    "Onze Heure S",
];

// Correspondence between 0-11 5-minutes packs and LEDs
static MINUTES_5_LED: [&str; 12] = [
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
static MINUTES_LED: [&str; 5] = [
    "",
    "Dot1",
    "Dot2",
    "Et2 Des Bananes Dot3",
    "Et2 Des Bananes Dot4",
];
