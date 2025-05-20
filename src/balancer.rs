use crate::Pecker;
use crate::sheet::Item;
use std::collections::BTreeMap;

pub struct Balancer<'a> {
    pecker: &'a mut Pecker,
}
impl<'a> Balancer<'a> {
    pub fn new(pecker: &'a mut Pecker) -> Self {
        Self { pecker }
    }

    pub fn balance(&mut self) {
        let changed = self.pecker.sheets.iter().enumerate().fold(
            BTreeMap::<usize, Vec<_>>::new(),
            |mut acc, (key, sheet)| {
                sheet
                    .borrow()
                    .entries
                    .iter()
                    .filter_map(|(_, entry)| {
                        self.pecker
                            .sheets
                            .iter()
                            .enumerate()
                            .filter(|(k, _)| *k < key)
                            .find_map(|(_sk, sh)| {
                                sh.borrow_mut()
                                    .add_item(&entry.item)
                                    .map(|_| (entry.item.clone(), entry.position.binary()))
                            })
                    })
                    .for_each(|(pos, _)| {
                        acc.entry(key)
                            .and_modify(|list| list.push(pos.clone()))
                            .or_insert(vec![pos]);
                    });
                acc
            },
        );

        self.remove_old_positions(&changed);
        self.pecker.clear_empty_sheets();
    }

    #[allow(clippy::needless_for_each)]
    fn remove_old_positions(&mut self, changes: &BTreeMap<usize, Vec<Item>>) {
        changes.iter().for_each(|(sheet_index, list)| {
            list.iter().for_each(|change| {
                self.pecker
                    .sheets
                    .get(*sheet_index)
                    .unwrap()
                    .borrow_mut()
                    .remove_item_by_binary(change);
            });
        });
    }
}
