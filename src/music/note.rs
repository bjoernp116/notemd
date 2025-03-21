use super::note_value::NoteValue;
use std::convert::From;




pub enum NoteLength {
    Sixteen = 16,
    Eight = 8,
    Quarter = 4,
    Half = 2,
    Whole = 1
}


pub struct Note {
    value: NoteValue,
    length: NoteLength
}

pub fn parse_note(string: String) -> Note {

}
