// custom build script, generating LED code to src/data.rs

use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

// TODO present this as a 2D board ? make enum have the value of LC in the enum value
// TODO try other durations patterns to reach 16 Max ? 
// Trying to merge None in the middle is not useful since minutes and hours are independent.
static LED_DURATIONS:[(&str, usize);29]= [
    ("Une",1),
    ("Deux",2),
    ("Trois",3),
    ("Quatre",3),
    ("Cinq",2),
    ("Six",2),
    ("Sept",2),
    ("Huit",2),
    ("Neuf",2),
    ("Onze",2),
    ("Mi",1),
    ("Di",1),
    ("X",1),
    ("Minuit",3),
    ("Heure",3),
    ("S",1),
    ("Et",1),
    ("Moins",3),
    ("DixMin",2),
    ("Vingt",3),
    ("CinqMin",2),
    ("Le",1),
    ("Quart",3),
    ("Demie",3),
    ("DesBananes",5),
    ("Dot1",1),
    ("Dot2",1),
    ("Dot3",1),
    ("Dot4",1),
];

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

static MINUTES_LED: [&str;5] = [
    "",
    "Dot1",
    "Dot2",
    "DesBananes Dot3",
    "DesBananes Dot4",
];

fn write_elements<'a>(file: &mut File, title: &str, max_leds: usize, elts: &[&'a str], durations: &HashMap<&str, usize>) {
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
                let nb = *durations.get(word).unwrap();
                leds += nb; // total used so far
                for _ in 0..nb {
                    write!(file, "Some(LED::{}), ", word).unwrap();
                }
            }
        }

        for _ in leds..max_leds {
            write!(file, "None, ").unwrap();
        }
        writeln!(file, "],").unwrap();
    }
    writeln!(file, "];").unwrap();
}

fn main() {
    let mut file = File::create("./src/data.rs").unwrap();

    writeln!(
        &mut file,
        "// horloge a mots data - this is a generated file"
    )
    .unwrap();

    // write all words enum
    writeln!(&mut file, "#[derive(Debug,Clone,Copy,PartialEq)]").unwrap();
    writeln!(&mut file, "pub enum LED {{").unwrap();
    
    let led_durations = HashMap::from(LED_DURATIONS);

    for (word,_len) in LED_DURATIONS.iter() {
        writeln!(&mut file, "    {},", word).unwrap();
    }
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "HOURS_LED", 7, &HOURS_LED, &led_durations);
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "MIN5_LED", 8, &MINUTES_5_LED, &led_durations);
    writeln!(&mut file, "// ---").unwrap();
    write_elements(&mut file, "MINUTES_LED", 6, &MINUTES_LED, &led_durations);
}
