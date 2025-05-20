#![feature(let_chains, if_let_guard, iter_map_windows, btree_extract_if)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![allow(clippy::missing_panics_doc)]

pub mod sheet;
pub mod balancer;

use crate::sheet::{Item, Sheet};
use crate::balancer::Balancer;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, Default)]
pub struct Pecker {
    sheets: Vec<Rc<RefCell<Sheet>>>,
    input: Vec<Item>,
    item_map: HashMap<usize, HashMap<usize, u32>>,
    cur: usize,
}

impl Pecker {
    #[must_use]
    pub fn new(input: Vec<Item>) -> Pecker {
        Pecker {
            sheets: vec![Rc::default()],
            input,
            item_map: HashMap::default(),
            cur: 0,
        }
    }

    pub fn run(&mut self) {
        self.input.iter_mut().enumerate().for_each(|(index, item)| {
            (0..item.count).rev().enumerate().for_each(|_| {
                loop {
                    match self.sheets.get_mut(self.cur).map(|sh| sh.borrow_mut().add_item(item)) {
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

                            self.sheets.push(Rc::new(RefCell::new(Sheet::default())));
                        }
                        None => {
                            self.cur += 1;
                        }
                    }
                }
            });
        });
        println!("pre-balancing");

        let mut balancer = Balancer::new(self);
        balancer.balance();
        println!("number of sheets {}", self.sheets.len());
    }

    pub fn clear_empty_sheets(&mut self) {
        let _ = self.sheets.extract_if(.., |sheet| sheet.borrow().entries.is_empty());
    }

}

fn main() {
    let input = vec![
        Item::new(1250, 1100, 1),
        Item::new(1250, 340, 2),
        Item::new(1100, 340, 2),
        Item::new(361, 611, 1),
        Item::new(361 ,511, 1),
        Item::new(716, 311, 1),
        Item::new(716, 316, 1),
        Item::new(211, 611, 1),
        Item::new(511, 716, 1),
        Item::new(211, 511, 1),
    ];
    /*
    let input = vec![
Item::new(2000,110,4),
Item::new(2000,100,4),
Item::new(810,2000,2),
Item::new(940,810,2),
Item::new(1380,80,2),
Item::ne>(1510,810,2),
Item::new(780,95,6),
Item::new(200,95,4),
Item::new(290,95,6),
Item::new(1240,95,8),
Item::new(640,95,4),
Item::new(1340,90,4),
Item::new(1460,90,6),
Item::new(640,90,6),
Item::new(1700,80,2),
Item::new(400,80,10),
Item::new(700,80,14),
Item::new(800,80,6),
    ];
    */
    let mut pecker = Pecker::new(input);
    pecker.run();
    dbg!(&pecker);
    pecker.sheets.iter().for_each(|sheet| {
        println!("{}", sheet.borrow());
    });
}

