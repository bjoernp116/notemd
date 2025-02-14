use super::note_value::NoteValue;




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
