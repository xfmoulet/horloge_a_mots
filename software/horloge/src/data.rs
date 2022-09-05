// horloge a mots data - this is a generated file
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LED {
    IlEst,
    Une,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Onze,
    Mi,
    Di,
    X,
    Minuit,
    Heure,
    S,
    Et,
    Moins,
    DixMin,
    Vingt,
    CinqMin,
    Le,
    Quart,
    Demie,
    EtDes,
    Bananes,
    Dot1,
    Dot2,
    Dot3,
    Dot4,
}
pub static LED_POSITIONS: [(u8, u8); 31] = [
    (0, 0), // IlEst
    (1, 1), // Une
    (1, 0), // Deux
    (3, 0), // Trois
    (2, 0), // Quatre
    (5, 1), // Cinq
    (4, 1), // Six
    (2, 1), // Sept
    (3, 1), // Huit
    (0, 1), // Neuf
    (4, 2), // Onze
    (0, 2), // Mi
    (1, 2), // Di
    (2, 2), // X
    (3, 2), // Minuit
    (5, 2), // Heure
    (4, 4), // S
    (5, 3), // Et
    (0, 3), // Moins
    (3, 3), // DixMin
    (1, 3), // Vingt
    (2, 3), // CinqMin
    (4, 3), // Le
    (0, 4), // Quart
    (1, 4), // Demie
    (2, 4), // EtDes
    (0, 5), // Bananes
    (1, 5), // Dot1
    (2, 5), // Dot2
    (3, 5), // Dot3
    (4, 5), // Dot4
];
// ---
pub const NB_HOURS_LED: usize = 7;
pub static HOURS_LED: [[Option<LED>; NB_HOURS_LED]; 24] = [
    [
        Some(LED::Minuit),
        Some(LED::Minuit),
        Some(LED::Minuit),
        None,
        None,
        None,
        None,
    ],
    [
        Some(LED::Une),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        None,
        None,
        None,
    ],
    [
        Some(LED::Deux),
        Some(LED::Deux),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Trois),
        Some(LED::Trois),
        Some(LED::Trois),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
    ],
    [
        Some(LED::Quatre),
        Some(LED::Quatre),
        Some(LED::Quatre),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
    ],
    [
        Some(LED::Cinq),
        Some(LED::Cinq),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Six),
        Some(LED::Six),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Sept),
        Some(LED::Sept),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Huit),
        Some(LED::Huit),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Neuf),
        Some(LED::Neuf),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Di),
        Some(LED::X),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Onze),
        Some(LED::Onze),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [Some(LED::Mi), Some(LED::Di), None, None, None, None, None],
    [
        Some(LED::Une),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        None,
        None,
        None,
    ],
    [
        Some(LED::Deux),
        Some(LED::Deux),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Trois),
        Some(LED::Trois),
        Some(LED::Trois),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
    ],
    [
        Some(LED::Quatre),
        Some(LED::Quatre),
        Some(LED::Quatre),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
    ],
    [
        Some(LED::Cinq),
        Some(LED::Cinq),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Six),
        Some(LED::Six),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Sept),
        Some(LED::Sept),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Huit),
        Some(LED::Huit),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Neuf),
        Some(LED::Neuf),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Di),
        Some(LED::X),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
    [
        Some(LED::Onze),
        Some(LED::Onze),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::Heure),
        Some(LED::S),
        None,
    ],
];
// ---
pub const NB_MIN5_LED: usize = 8;
pub static MIN5_LED: [[Option<LED>; NB_MIN5_LED]; 12] = [
    [None, None, None, None, None, None, None, None],
    [
        Some(LED::CinqMin),
        Some(LED::CinqMin),
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        Some(LED::DixMin),
        Some(LED::DixMin),
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        Some(LED::Et),
        Some(LED::Quart),
        Some(LED::Quart),
        Some(LED::Quart),
        None,
        None,
        None,
        None,
    ],
    [
        Some(LED::Vingt),
        Some(LED::Vingt),
        Some(LED::Vingt),
        None,
        None,
        None,
        None,
        None,
    ],
    [
        Some(LED::Vingt),
        Some(LED::Vingt),
        Some(LED::Vingt),
        Some(LED::CinqMin),
        Some(LED::CinqMin),
        None,
        None,
        None,
    ],
    [
        Some(LED::Et),
        Some(LED::Demie),
        Some(LED::Demie),
        Some(LED::Demie),
        None,
        None,
        None,
        None,
    ],
    [
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Vingt),
        Some(LED::Vingt),
        Some(LED::Vingt),
        Some(LED::CinqMin),
        Some(LED::CinqMin),
    ],
    [
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Vingt),
        Some(LED::Vingt),
        Some(LED::Vingt),
        None,
        None,
    ],
    [
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Le),
        Some(LED::Quart),
        Some(LED::Quart),
        Some(LED::Quart),
        None,
    ],
    [
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::DixMin),
        Some(LED::DixMin),
        None,
        None,
        None,
    ],
    [
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::Moins),
        Some(LED::CinqMin),
        Some(LED::CinqMin),
        None,
        None,
        None,
    ],
];
// ---
pub const NB_MINUTES_LED: usize = 6;
pub static MINUTES_LED: [[Option<LED>; NB_MINUTES_LED]; 5] = [
    [None, None, None, None, None, None],
    [Some(LED::Dot1), None, None, None, None, None],
    [Some(LED::Dot2), None, None, None, None, None],
    [
        Some(LED::EtDes),
        Some(LED::EtDes),
        Some(LED::Bananes),
        Some(LED::Bananes),
        Some(LED::Bananes),
        Some(LED::Dot3),
    ],
    [
        Some(LED::EtDes),
        Some(LED::EtDes),
        Some(LED::Bananes),
        Some(LED::Bananes),
        Some(LED::Bananes),
        Some(LED::Dot4),
    ],
];
