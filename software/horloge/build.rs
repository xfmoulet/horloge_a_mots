// This file is a custom build script
// It is run , generating LED code to src/data.rs

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// Big Panel ------------------------------------------------------------------------------------

#[cfg(feature = "big_panel")]
// used for placement
static LED_PANEL: [&str; 6] = [
    "IlEst Deux Quatre Trois",
    "Neuf Une Sept Huit Six Cinq",
    "Mi Di X Minuit Onze Heure S",
    "Moins Vingt CinqMin DixMin Le Et",
    "Quart Demi E EtDes Pile",
    "Bananes Dot1 Dot2 Dot3 Dot4",
];

#[cfg(feature = "big_panel")]
static LED_DURATIONS: [(&str, usize); 32] = [
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
    ("Demi", 2),
    ("E", 1),
    ("EtDes", 2),
    ("Bananes", 3),
    ("Dot1", 1),
    ("Dot2", 1),
    ("Dot3", 1),
    ("Dot4", 1),
];

// TODO try other durations patterns to reach 16 Max ?
// Correspondence bewteen 0-23 hour and corresponding LEDs to illuminate
#[cfg(feature = "big_panel")]
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
#[cfg(feature = "big_panel")]
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
#[cfg(feature = "big_panel")]
static MINUTES_LED: [&str; 5] = [
    "",
    "Dot1",
    "Dot2",
    "EtDes Bananes Dot3",
    "EtDes Bananes Dot4",
];

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

// Mini Panel (4x2) knightrider mode ------------------------------------------------------------------------------------

#[cfg(feature = "mini_demo")]
// used for placement
static LED_PANEL: [&str;6] = [
"x1 x2",
"x3 x4",
"L1 L2", 
"L3 L4", 
"L5 L6", 
"L7 L8"
];

#[cfg(feature = "mini_demo")]
static LED_DURATIONS: [(&str, usize); 8] = [
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
static HOURS_LED: [&str; 0] = [];

// Correspondence between 0-11 5-minutes packs and LEDs
#[cfg(feature = "mini_demo")]
static MINUTES_5_LED: [&str; 12] = [
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
static MINUTES_LED: [&str; 0] = [];

// Code Gen ------------------------------------------------------------------------------------

// Write to a file the table of option<LED>s (with name `title`, of `max_leds` size) from a list of strings (`elts`), repeating the LEDs accordingly to the durations table
fn write_elements<'a>(
    file: &mut File,
    title: &str,
    elts: &[&'a str],
    durations: &HashMap<&str, usize>,
) {
    let max_leds = elts
        .iter()
        .map(|s| {
            if !s.is_empty() {
                s.split(' ').map(|word| *durations.get(word).unwrap()).sum()
            } else {
                0
            }
        })
        .fold(0, |a, b| a.max(b));

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


fn write_patterns(file: &mut File) -> Result<(),std::io::Error> {
    file.write_all(b"// Interleaving patterns for 32 values and 32 ticks\nconst _INTERLEAVE_PATTERNS: [u32;32] = {\n")?;
    for value in 0..32 {
        write!(file, "    0b")?;
        for tick in 0..32 {
            let a = (tick + 1) * value / 32;
            let b = (tick + 0) * value / 32;
            file.write_all(if a != b { b"1" } else { b"0" })?;
        // print(value, sum(1 for a in s if a == "*"), s)
        }
        writeln!(file,",")?
    }
    file.write_all(b"};")?;
    Ok(())
}

// Generate the src/data.rs file according to LED tables
fn main() {
    let mut file = File::create("./src/data.rs").unwrap();

    writeln!(
        &mut file,
        "// horloge a mots data - file generated by build.rs"
    )
    .unwrap();

    // write all words enum
    writeln!(&mut file, "#[derive(Debug, Clone, Copy, PartialEq)]").unwrap();

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
        "pub static LED_POSITIONS: [(u8, u8); {}] = [",
        LED_DURATIONS.len()
    )
    .unwrap();
    for (word, _len) in LED_DURATIONS.iter() {
        if let Some((column, line)) = led_position(word) {
            writeln!(&mut file, "    ({column}, {line}), // {word}").unwrap();
        } else {
            panic!("position not found for LED {word}");
        }
    }
    writeln!(&mut file, "];").unwrap();

    // LED hours tables
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "HOURS_LED", &HOURS_LED, &led_durations);
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "MIN5_LED", &MINUTES_5_LED, &led_durations);
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "MINUTES_LED", &MINUTES_LED, &led_durations);

    writeln!(&mut file, "// ---").unwrap();
    write_patterns(&mut file).unwrap();
}
