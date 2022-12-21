// horloge a mots data - file generated by build.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LED {
    Moins,
    Et,
    DixMin,
    Vingt,
    CinqMin,
    Le,
    Quart,
    Demi,
}
pub static LED_POSITIONS: [(u8, u8); 8] = [
    (0, 2), // Moins
    (0, 3), // Et
    (0, 5), // DixMin
    (1, 4), // Vingt
    (1, 5), // CinqMin
    (1, 2), // Le
    (1, 3), // Quart
    (0, 4), // Demi
];
// ---
pub const NB_HOURS_LED : usize = 0;
pub static HOURS_LED: [[Option<LED>;NB_HOURS_LED];0] = [
];
// ---
pub const NB_MIN5_LED : usize = 3;
pub static MIN5_LED: [[Option<LED>;NB_MIN5_LED];12] = [
    [None, None, None, ],
    [Some(LED::CinqMin), None, None, ],
    [Some(LED::DixMin), None, None, ],
    [Some(LED::Et), Some(LED::Quart), None, ],
    [Some(LED::Vingt), None, None, ],
    [Some(LED::Vingt), Some(LED::CinqMin), None, ],
    [Some(LED::Et), Some(LED::Demi), None, ],
    [Some(LED::Moins), Some(LED::Vingt), Some(LED::CinqMin), ],
    [Some(LED::Moins), Some(LED::Vingt), None, ],
    [Some(LED::Moins), Some(LED::Le), Some(LED::Quart), ],
    [Some(LED::Moins), Some(LED::DixMin), None, ],
    [Some(LED::Moins), Some(LED::CinqMin), None, ],
];
// ---
pub const NB_MINUTES_LED : usize = 0;
pub static MINUTES_LED: [[Option<LED>;NB_MINUTES_LED];0] = [
];
// ---
// Interleaving patterns for 32 values and 32 ticks
pub const INTERLEAVE_PATTERNS: [u32;32] = [
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000001,
    0b00000000000000010000000000000001,
    0b00000000001000000000010000000001,
    0b00000001000000010000000100000001,
    0b00000010000010000001000001000001,
    0b00000100001000010000010000100001,
    0b00001000010001000010001000010001,
    0b00010001000100010001000100010001,
    0b00010001001000100100010010001001,
    0b00010010010010010001001001001001,
    0b00100100100100100100100100100101,
    0b00100101001001010010010100100101,
    0b00101001010010100101001010010101,
    0b00101010010101010010101001010101,
    0b00101010101010100101010101010101,
    0b01010101010101010101010101010101,
    0b01010101010101011010101010101011,
    0b01010101101010110101010110101011,
    0b01010110101101011010110101101011,
    0b01011011010110110101101101011011,
    0b01011011011011011011011011011011,
    0b01101101101101110110110110110111,
    0b01101110110111011011101101110111,
    0b01110111011101110111011101110111,
    0b01110111101110111101110111101111,
    0b01111011110111110111101111011111,
    0b01111101111101111110111110111111,
    0b01111111011111110111111101111111,
    0b01111111110111111111101111111111,
    0b01111111111111110111111111111111,
    0b01111111111111111111111111111111,
];