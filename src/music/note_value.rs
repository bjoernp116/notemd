use std::convert::From;

pub struct NoteValue {
    pitch: NotePitch,
    relative: i32, // Semitones from Middle C
    octave: u8
}

pub enum NotePitch {
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    Es,
    Fb,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
    A,
    As,
    Bb,
    B,
    Bs,
    Cb
}

#[derive(Debug)]
pub enum Accidental {
    Sharp, Flat, Natural
}

impl From<(char, Accidental)> for NotePitch {
    fn from(value: (char, Accidental)) -> NotePitch {
        use Accidental::*;
        match value {
            ('C', Natural) => NotePitch::C,
            ('C', Sharp) => NotePitch::Cs,
            ('D', Flat) => NotePitch::Db,
            ('D', Natural) => NotePitch::D,
            ('D', Sharp) => NotePitch::Ds,
            ('E', Flat) => NotePitch::Eb,
            ('E', Natural) => NotePitch::E,
            ('E', Sharp) => NotePitch::Es,
            ('F', Flat) => NotePitch::Fb,
            ('F', Natural) => NotePitch::F,
            ('F', Sharp) => NotePitch::Fs,
            ('G', Flat) => NotePitch::Gb,
            ('G', Natural) => NotePitch::G,
            ('G', Sharp) => NotePitch::Gs,
            ('A', Flat) => NotePitch::Ab,
            ('A', Natural) => NotePitch::A,
            ('A', Sharp) => NotePitch::As,
            ('B', Flat) => NotePitch::Bb,
            ('B', Natural) => NotePitch::B,
            ('B', Sharp) => NotePitch::Bs,
            ('C', Flat) => NotePitch::Cb,
            _ => panic!("Unknowned Note: {}{:?}", value.0, value.1)
        }
    }
}

