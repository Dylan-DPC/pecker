#![feature(let_chains, if_let_guard, iter_map_windows)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(clippy::missing_panics_doc)]

pub mod sheet;

use crate::sheet::{Item, Sheet};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Pecker {
    sheets: Vec<Sheet>,
    input: Vec<Item>,
    item_map: HashMap<usize, HashMap<usize, u32>>,
    cur: usize,
}

impl Pecker {
    pub fn new(input: Vec<Item>) -> Pecker {
        Pecker {
            sheets: vec![Sheet::default()],
            input,
            item_map: HashMap::default(),
            cur: 0,
        }
    }

    pub fn run(&mut self) {
        self.input.iter_mut().enumerate().for_each(|(index, item)| {
            (0..item.count).rev().enumerate().for_each(|_| {
                loop {
                    match self.sheets.get_mut(self.cur).map(|sh| sh.add_item(item)) {
                        Some(Some(n)) if let Some(its) = self.item_map.get_mut(&self.cur) => {
                            its.insert(index, n);
                            break;
                        }
                        Some(Some(n)) => {
                            let mut map = HashMap::new();
                            map.insert(index, n);

                            self.item_map.insert(self.cur, map);
                            break;
                        }
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
        });
    }
}

fn main() {
    let input = vec![
        Item::new(950, 1830, 2),
        Item::new(730, 300, 1),
        Item::new(550, 300, 4),
    ];
    let mut pecker = Pecker::new(input);
    pecker.run();
    // dbg!(pecker.sheets);
}
