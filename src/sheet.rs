use itertools::Itertools;
use std::cell::Cell;
use std::collections::BTreeMap;

pub const SHEET_WIDTH: u32 = 2400;
pub const SHEET_HEIGHT: u32 = 1250;
pub const TOOL_WIDTH: u32 = 10;

#[must_use]
pub const fn bit_length() -> u32 {
    SHEET_HEIGHT.ilog2() + width_bit_length()
}

#[must_use]
pub const fn column_end() -> u32 {
    SHEET_WIDTH - 1
}

#[must_use]
pub const fn row_end() -> u32 {
    SHEET_HEIGHT - 1
}

#[must_use]
pub const fn width_bit_length() -> u32 {
    SHEET_WIDTH.ilog2() + 1
}
#[derive(Clone, Debug, Default)]
pub struct Sheet {
    pub entries: BTreeMap<u32, PlacedItem>,
    balanced: Cell<bool>,
    last_row: Cell<u32>,
}

impl Sheet {
    pub fn add_item(&mut self, item: &Item) -> Option<u32> {
        if self.entries.is_empty() {
            self.entries
                .insert(0, PlacedItem::place_on_empty_sheet(item));
            Some(0)
        } else if let Some(placed_item) = self.find_region(item) {
            let binary = placed_item.position.binary();
            self.entries.insert(binary, placed_item);
            Some(binary)
        } else {
            None
        }
    }

    pub fn remove_item_by_binary(&mut self, item: &Item) {
        self.balanced.set(true);
        self.entries.retain(|_, placed| placed.item != *item);
    }

    pub fn find_region(&self, item: &Item) -> Option<PlacedItem> {
        if self.entries.len() > 1 {
            let iter = self.entries.iter();
            let y_pair = iter.clone();

            let mut possible_regions = if self.balanced.get()
                && let Some(regions) = self.find_regions_post_balanced(item)
            {
                vec![Some(regions)]
            } else {
                iter.cartesian_product(y_pair)
                    .filter(|((first, _), (second, _))| first > second)
                    .map(|((_, right_item), (_, left_item))| {
                        self.find_region_between_items(item, left_item, right_item)
                    })
                    .collect::<Vec<Option<_>>>()
            };
            if possible_regions.is_empty() || possible_regions.contains(&None) {
                self.find_region_at_end(item).inspect(|pos| {
                    self.last_row.set(pos.position.y);
                })
            } else {
                possible_regions.pop().unwrap()
            }
        } else {
            self.find_adjacent_to_first_item(item)
        }
    }

    pub fn find_region_at_end(&self, item: &Item) -> Option<PlacedItem> {
        let last_row = self.last_row.get();
        let mut iter = self
            .entries
            .iter()
            .filter(|(_, item)| item.position.y == last_row)
            .peekable();
        iter.clone().find_map(|(_, last)| {
            let start = last.next_possible_column();
            let next = iter.peek();
            match (self.check_item_with_boundary(&start, item), next) {
                (Some(_), None) => Some(PlacedItem::new(item.to_owned(), start)),
                (Some(_), Some((_, n)))
                    if let Some(Orientation::Normalised | Orientation::Flipped) =
                        self.check_item_with_neighbour(&start, item, n) =>
                {
                    Some(PlacedItem::new(item.to_owned(), start))
                }
                (Some(_), Some(_)) | (None, _) => {
                    let start = last.next_possible_row();
                    self.check_item_with_boundary(&start, item)
                        .map(|_| PlacedItem::new(item.to_owned(), start))
                }
            }
        })
    }

    pub fn find_adjacent_to_first_item(&self, item: &Item) -> Option<PlacedItem> {
        let (_, first) = self.entries.iter().next().unwrap();
        let start = first.next_possible_column();
        if self.check_item_with_boundary(&start, item).is_some() {
            Some(PlacedItem::new(item.to_owned(), start))
        } else {
            let start = first.next_possible_row();
            self.check_item_with_boundary(&start, item)
                .map(|_| PlacedItem::new(item.to_owned(), start))
        }
    }

    pub fn find_region_between_items(
        &self,
        item: &Item,
        left: &PlacedItem,
        right: &PlacedItem,
    ) -> Option<PlacedItem> {
        let start = left.next_possible_column();
        self.check_item_is_valid(start, item, right)
    }

    pub fn check_item_is_valid(
        &self,
        start: Position,
        item: &Item,
        right: &PlacedItem,
    ) -> Option<PlacedItem> {
        match (
            self.check_item_with_boundary(&start, item),
            self.check_item_with_neighbour(&start, item, right),
        ) {
            (None, _) | (_, None) => None,
            (Some(Orientation::Normalised), Some(Orientation::Normalised))
            | (Some(Orientation::Flipped), Some(Orientation::Flipped)) => {
                Some(PlacedItem::new(item.to_owned(), start))
            }
            (Some(Orientation::Flipped), Some(Orientation::Normalised)) => {
                item.flip();
                if self
                    .check_item_with_neighbour(&start, item, right)
                    .is_some()
                {
                    Some(PlacedItem::new(item.to_owned(), start))
                } else {
                    None
                }
            }
            (Some(Orientation::Normalised), Some(Orientation::Flipped)) => {
                item.flip();
                if self.check_item_with_boundary(&start, item).is_some() {
                    Some(PlacedItem::new(item.to_owned(), start))
                } else {
                    None
                }
            }
        }
    }

    pub fn check_item_with_boundary(
        &self,
        position: &Position,
        item: &Item,
    ) -> Option<Orientation> {
        if self.check_boundary(position, item) {
            Some(Orientation::Normalised)
        } else {
            item.flip();
            if self.check_boundary(position, item) {
                Some(Orientation::Flipped)
            } else {
                None
            }
        }
    }

    pub fn check_item_with_neighbour(
        &self,
        position: &Position,
        item: &Item,
        right: &PlacedItem,
    ) -> Option<Orientation> {
        if self.check_neighbour_conflict(position, item, right) {
            Some(Orientation::Normalised)
        } else {
            item.flip();
            if self.check_neighbour_conflict(position, item, right) {
                Some(Orientation::Flipped)
            } else {
                None
            }
        }
    }

    #[must_use]
    pub fn check_boundary(&self, start: &Position, item: &Item) -> bool {
        let binary = start.binary();
        let column_end = column_end();
        let row_end = row_end();
        let width_bit = SHEET_WIDTH.next_power_of_two() - 1;
        let height_bit = (SHEET_HEIGHT.next_power_of_two() - 1) << width_bit_length();

        column_end.saturating_sub(binary & width_bit) >= (item.width.get() - 1)
            && row_end.saturating_sub((binary & height_bit) >> width_bit_length())
                >= (item.height.get() - 1)
    }

    #[must_use]
    pub fn check_neighbour_conflict(
        &self,
        start: &Position,
        item: &Item,
        right: &PlacedItem,
    ) -> bool {
        let binary = start.binary();
        let right_start = right.position.binary();
        right_start.saturating_sub(binary) > item.width.get()
    }

    pub fn find_regions_post_balanced(&self, item: &Item) -> Option<PlacedItem> {
        if self.entries.contains_key(&0) {
            None
        } else {
            let pos = Position::new(0, 0);
            let (_, first) = self.entries.iter().next().unwrap();
            self.check_item_is_valid(pos, item, first)
                .inspect(|_| self.balanced.set(true))
        }
    }
}

impl std::fmt::Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entries.iter().try_for_each(|(_, item)| {
            writeln!(
                f,
                "({}, {}) @ ({}, {})",
                item.item.height.get(),
                item.item.width.get(),
                item.position.x,
                item.position.y
            )
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    width: Cell<u32>,
    height: Cell<u32>,
    pub count: u8,
}

impl Item {
    #[must_use]
    pub fn new(width: u32, height: u32, count: u8) -> Self {
        Self {
            width: Cell::new(width),
            height: Cell::new(height),
            count,
        }
    }

    pub fn align_item(&self) {
        if self.width > self.height {
            self.flip();
        }
    }

    pub fn flip(&self) {
        let width = self.width.get();
        let height = self.height.get();
        self.width.set(height);
        self.height.set(width);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlacedItem {
    pub item: Item,
    pub position: Position,
}

impl PlacedItem {
    fn new(item: Item, position: Position) -> Self {
        Self { item, position }
    }
    fn place_on_empty_sheet(item: &Item) -> Self {
        item.align_item();
        PlacedItem::new(item.to_owned(), Position::new(0, 0))
    }

    fn next_possible_column(&self) -> Position {
        Position::new(
            self.position.x + 1 + self.item.width.get() + TOOL_WIDTH,
            self.position.y,
        )
    }

    fn next_possible_row(&self) -> Position {
        Position::new(
            self.position.x,
            self.position.y + self.item.height.get() + TOOL_WIDTH,
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    #[must_use]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn binary(&self) -> u32 {
        self.y * SHEET_WIDTH.next_power_of_two() + self.x
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Normalised,
    Flipped,
}
