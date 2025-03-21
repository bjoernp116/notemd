#![allow(dead_code)]
use anyhow::Result;

use crate::music::{bar::Bar, note_value::{Accidental, NotePitch}};




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
        let mut pitch: char = 'C';
        let mut octave: u8 = 0;
        let mut accidental: Accidental = Accidental::Natural;

        for c in input.chars() {
            match c {
                'A'..'H' => {
                    pitch = c;
                },
                '#' | 'b' | 'n' => {
                    accidental = match c {
                        '#' => Accidental::Sharp,
                        'b' => Accidental::Flat,
                        'n' => Accidental::Natural
                },
                '0'..'9' =>
                _ => {}

            }
        }
        todo!()
    }

    pub fn parse(&mut self, input: String) -> Result<()> {
        let mut bar_buffer = String::new();
        let mut buffer = String::new();
        let mut state = ParsingState::Text;

        for (_i, c) in input.chars().enumerate() {
            match (c, state.clone()) {
                ('|', ParsingState::Text) => {
                    self.data.push(Element::PlainText(buffer.clone()));
                    bar_buffer.clear();
                    state = ParsingState::Bar;
                },
                ('|', ParsingState::Bar) => {
                    let bar = Self::parse_bar(bar_buffer.clone());
                    self.data.push(Element::Bar(bar));
                    bar_buffer.clear();
                    state = ParsingState::Text;
                },
                _ => match state.clone() {
                    ParsingState::Bar => bar_buffer.push(c),
                    _ => buffer.push(c),
                }
            }
        }

        Ok(())
    }
}
