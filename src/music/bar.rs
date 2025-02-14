use super::note::Note;



struct Collum(Vec<Note>);


pub struct Bar {
    repeat: bool,
    collums: Vec<Collum>
}
