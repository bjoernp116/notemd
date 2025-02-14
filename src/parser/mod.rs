#![allow(dead_code)]
use anyhow::Result;

use crate::music::{bar::Bar, note_value::NotePitch};




enum Element {
    Bar(Bar),
    PlainText(String)
}

#[derive(Clone)]
enum ParsingState {
    Text, Bar, Expression
}

struct Document {
    data: Vec<Element>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            data: Vec::new(),
        }
    }

    pub fn parse_bar(input: String) -> Bar {
        let pitch: NotePitch = NotePitch::C;
        let octave: u8 = 0;

        for c in input.chars() {
            match c {
                'A'..'H' => continue,
                _ => {}

            }
        }
        todo!()
    }

    pub fn parse(&mut self, input: String) -> Result<()> {
        let mut buffer = String::new();
        let mut state = ParsingState::Text;

        for c in input.chars() {
            match (c, state.clone()) {
                ('|', ParsingState::Text) => {
                    self.data.push(Element::PlainText(buffer.clone()));
                    buffer.clear();
                    state = ParsingState::Bar;
                },
                ('|', ParsingState::Bar) => {
                    let bar = Self::parse_bar(buffer.clone());
                    self.data.push(Element::Bar(bar));
                    buffer.clear();
                    state = ParsingState::Text;
                },
                ('(', ParsingState::Bar) => {

                }
                _ => buffer.push(c)
            }
        }

        Ok(())
    }
}
