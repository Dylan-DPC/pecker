#![feature(let_chains, if_let_guard, iter_map_windows)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(clippy::missing_panics_doc)]

use crate::sheet::{Item, Sheet};

pub mod sheet;

#[derive(Clone, Debug, Default)]
struct Pecker {
    sheets: Vec<Sheet>,
    input: Vec<Item>,
    cur: usize,
}

impl Pecker {
    pub fn new(input: Vec<Item>) -> Pecker {
        Pecker {
            sheets: vec![Sheet::default()],
            input,
            cur: 0,
        }
    }

    pub fn run(&mut self) {
        self.input.iter_mut().for_each(|item| {
            loop {
                match self.sheets.get_mut(self.cur).map(|sh| sh.add_item(item)) {
                    Some(Some(())) => break,
                    Some(None) => {
                        self.cur += 1;

                        self.sheets.push(Sheet::default());
                    }
                    None => {
                        self.cur += 1;
                    }
                }
            }
        });
    }
}

fn main() {
    let input = vec![
        Item::new(1190, 87, 1),
        Item::new(300, 87, 1),
        Item::new(1262, 267, 1),
        Item::new(367, 1262, 1),
        Item::new(362, 1262, 1),
        Item::new(1190, 87, 1),
        Item::new(300, 87, 1),
        Item::new(1262, 267, 1),
        Item::new(367, 1262, 1),
        Item::new(362, 1262, 1),
        Item::new(300, 87, 1),
        Item::new(1262, 267, 1),
        Item::new(367, 1262, 1),
        Item::new(362, 1262, 1),
    ];
    let mut pecker = Pecker::new(input);
    pecker.run();
    dbg!(pecker.sheets);
}
