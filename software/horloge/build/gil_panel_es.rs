// used for placement
pub static LED_PANEL: [&str; 6] = [
    "Una S2 La On S1 E",
    "Cuatro Nce O Och Tres Dos",
    "Siete Nueve Xxx Cinco Xxx Diez",
    "Xxx Xxx Seis Xxx Doce Xxx",
    "Xxx Veinte Menos Xxx Y Diez2",
    "Cuarto Xxx Cinco2 Veinti Media Xxx",
];

pub static LED_DURATIONS: [(&str, usize); 26] = [
    ("Una", 2),
    ("S2", 1),
    ("La", 1),
    ("On", 1),
    ("S1", 1),
    ("E", 1),
    ("Cuatro", 3),
    ("Nce", 2),
    ("O", 1),
    ("Och", 2),
    ("Tres", 2),
    ("Dos", 2),
    ("Siete", 3),
    ("Nueve", 3),
    ("Cinco", 3),
    ("Diez", 2),
    ("Seis", 2),
    ("Doce", 2),
    ("Veinte", 3),
    ("Menos", 3),
    ("Y", 1),
    ("Diez2", 2),
    ("Cuarto", 3),
    ("Cinco2", 3),
    ("Veinti", 3),
    ("Media", 3),
];

// TODO try other durations patterns to reach 16 Max ?
// Correspondence bewteen 0-23 hour and corresponding LEDs to illuminate
pub static HOURS_LED: [&str; 24] = [
    "S1 On La S2 Doce",
    "E S1 La Una",
    "S1 On La S2 Dos",
    "S1 On La Tres",
    "S1 On La S2 Cuatro",
    "S1 On La S2 Cinco",
    "S1 On La S2 Seis",
    "S1 On La S2 Siete",
    "S1 On La S2 O Och",
    "S1 On La S2 Nueve",
    "S1 On La S2 Diez",
    "S1 On La S2 O Nce",
    "S1 On La S2 Doce",
    "E S1 La Una",
    "S1 On La S2 Dos",
    "S1 On La Tres",
    "S1 On La S2 Cuatro",
    "S1 On La S2 Cinco",
    "S1 On La S2 Seis",
    "S1 On La S2 Siete",
    "S1 On La S2 O Och",
    "S1 On La S2 Nueve",
    "S1 On La S2 Diez",
    "S1 On La S2 O Nce",
];

// Correspondence between 0-11 5-minutes packs and LEDs
pub static MINUTES_5_LED: [&str; 12] = [
    "",
    "Y Cinco2",
    "Y Diez2",
    "Y Cuarto",
    "Y Veinte",
    "Y Veinti Cinco2",
    "Y Media",
    "Menos Veinti Cinco2",
    "Menos Veinte",
    "Menos Cuarto",
    "Menos Diez2",
    "Menos Cinco2",
];

// Correspondence between 0-5 remaining minute and LEDs
pub static MINUTES_LED: [&str; 5] = ["", "", "", "", ""];
