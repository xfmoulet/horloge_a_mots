// used for placement
pub static LED_PANEL: [&str; 6] = [
    "Deux Trois X Est Quatre Il",
    "Sept Cinq Six Une Neuf Huit",
    "Minuit S Heure Onze Di Mi",
    "Vingt Et Le DixMin Moins CinqMin",
    "E Pile Demi Des Quart Et2",
    "Dot4 xxx Dot3 Dot2 Dot1 Bananes",
];

pub static LED_DURATIONS: [(&str, usize); 34] = [
    ("Il", 1),
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
    ("Mi", 1),
    ("Di", 1),
    ("X", 1),
    ("Minuit", 3),
    ("Heure", 3),
    ("S", 1),
    ("Et", 1),
    ("Moins", 3),
    ("DixMin", 2),
    ("Vingt", 3),
    ("CinqMin", 2),
    ("Le", 1),
    ("Quart", 3),
    ("Demi", 2),
    ("E", 1),
    ("Et2", 1),
    ("Des", 2),
    ("Bananes", 3),
    ("Dot1", 1),
    ("Dot2", 1),
    ("Dot3", 1),
    ("Dot4", 1)
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
    "Dot3",
    "Dot4",
];
