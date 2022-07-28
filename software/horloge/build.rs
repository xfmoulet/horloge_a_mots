// This file is a custom build script
// It is run , generating LED code to src/data.rs

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// used for placement
static LED_PANEL: [&str; 6] = [
    "IlEst Deux Quatre Trois",
    "Neuf Une Sept Huit Six Cinq",
    "Mi Di X Minuit Onze Heure",
    "Moins Vingt CinqMin DixMin Le Et",
    "Quart Demie EtDes Pile S",    // S Here physically is after "Heure"
    "Bananes Dot1 Dot2 Dot3 Dot4", // Alt: use "presque"
];

static LED_DURATIONS: [(&str, usize); 31] = [
    ("IlEst", 3),
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
    ("Demie", 3),
    ("EtDes", 2),
    ("Bananes", 3),
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
    "Et Demie",
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
    "EtDes Bananes Dot3",
    "EtDes Bananes Dot4",
];

// Write to a file the table of option<LED>s (with name `title`, of `max_leds` size) from a list of strings (`elts`), repeating the LEDs accordingly to the durations table
fn write_elements<'a>(
    file: &mut File,
    title: &str,
    max_leds: usize,
    elts: &[&'a str],
    durations: &HashMap<&str, usize>,
) {
    writeln!(file, "pub const NB_{} : usize = {};", title, max_leds).unwrap();
    writeln!(
        file,
        "pub static {}: [[Option<LED>;NB_{}];{}] = [",
        title,
        title,
        elts.len()
    )
    .unwrap();

    for words in elts.iter() {
        write!(file, "    [").unwrap();
        let mut leds = 0;
        if !words.is_empty() {
            for word in words.split(' ') {
                let nb = *durations.get(word).unwrap(); // word.len() / 3 +1
                leds += nb; // total used so far
                for _ in 0..nb {
                    write!(file, "Some(LED::{}), ", word).unwrap();
                }
            }
        }

        /* Insert None LEDs in unused slots.
           Trying to skip None/merge hours and minutes between hours and min5 by example
           is not useful since all combinations will happen so we must support worst case.
        */
        for _ in leds..max_leds {
            write!(file, "None, ").unwrap();
        }
        writeln!(file, "],").unwrap();
    }
    writeln!(file, "];").unwrap();
}

// slow find but not necessary to optimize
fn led_position(name: &str) -> Option<(u8, u8)> {
    for (line, words) in LED_PANEL.iter().enumerate() {
        for (column, word) in words.split(' ').enumerate() {
            if word == name {
                return Some((column as u8, line as u8));
            }
        }
    }
    None
}

// Generate the src/data.rs file according to LED tables
fn main() {
    let mut file = File::create("./src/data.rs").unwrap();

    writeln!(
        &mut file,
        "// horloge a mots data - this is a generated file"
    )
    .unwrap();

    // write all words enum
    writeln!(&mut file, "#[derive(Debug,Clone,Copy,PartialEq)]").unwrap();

    let led_durations = HashMap::from(LED_DURATIONS);

    // LED enum
    writeln!(&mut file, "pub enum LED {{").unwrap();
    for (word, _len) in LED_DURATIONS.iter() {
        writeln!(&mut file, "    {word},").unwrap();
    }
    writeln!(&mut file, "}}").unwrap();

    // LED positions
    writeln!(
        &mut file,
        "pub static LED_POSITIONS : [(u8, u8);{}] = [",
        LED_DURATIONS.len()
    )
    .unwrap();
    for (word, _len) in LED_DURATIONS.iter() {
        if let Some((column, line)) = led_position(word) {
            writeln!(&mut file, "    ({column},{line}), // {word}").unwrap();
        } else {
            panic!("position not found for LED {word}");
        }
    }
    writeln!(&mut file, "];").unwrap();

    // LED hours tables
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "HOURS_LED", 7, &HOURS_LED, &led_durations);
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "MIN5_LED", 8, &MINUTES_5_LED, &led_durations);
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "MINUTES_LED", 6, &MINUTES_LED, &led_durations);
}
