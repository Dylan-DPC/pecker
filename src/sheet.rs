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
    entries: BTreeMap<u32, PlacedItem>,
}

impl Sheet {
    pub fn add_item(&mut self, item: &mut Item) -> Option<()> {
        if self.entries.is_empty() {
            self.entries
                .insert(0, PlacedItem::place_on_empty_sheet(item));
            Some(())
        } else if let (Some(placed_item), position) = self.find_region(item) {
            self.entries.insert(position, placed_item);
            Some(())
        } else {
            todo!()
        }
    }

    pub fn find_region(&mut self, item: &mut Item) -> (Option<PlacedItem>, u32) {
        if self.entries.len() > 1 {
            let region = self
                .entries
                .iter()
                .map_windows(|[(code, item), (next_code, next_item)]| todo!())
                .fold(None, |picked, item| todo!());
            (region, 0)
        } else {
            (self.find_adjacent_to_first_item(item), 1)
        }
    }

    pub fn find_adjacent_to_first_item(&self, item: &mut Item) -> Option<PlacedItem> {
        let first = self.entries.iter().next().unwrap();
        let start = first.1.next_possible_column();
        if self.check_item_with_boundary(&start, item).is_some() {
            Some(PlacedItem::new(item.to_owned(), start))
        } else {
            let start = first.1.next_possible_row();
            self.check_item_with_boundary(&start, item)
                .map(|_| PlacedItem::new(item.to_owned(), start))
        }
    }

    pub fn check_item_with_boundary(
        &self,
        position: &Position,
        item: &mut Item,
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

    #[must_use]
    pub fn check_boundary(&self, start: &Position, item: &Item) -> bool {
        let binary = start.binary();
        let column_end = column_end();
        let row_end = row_end();
        let width_bit = SHEET_WIDTH.next_power_of_two() - 1;
        let height_bit = (SHEET_HEIGHT.next_power_of_two() - 1) << width_bit_length();

        column_end.saturating_sub(binary & width_bit) > item.width
            && (row_end.saturating_sub((binary & height_bit) >> width_bit_length())) > item.height
    }
}

#[derive(Clone, Debug, Default)]
pub struct Item {
    width: u32,
    height: u32,
    count: u8,
}

impl Item {
    #[must_use]
    pub fn new(width: u32, height: u32, count: u8) -> Self {
        Self {
            width,
            height,
            count,
        }
    }

    pub fn align_item(&mut self) {
        if self.width < self.height {
            self.flip();
        }
    }

    pub fn flip(&mut self) {
        let width = self.width;
        let height = self.height;
        self.width = height;
        self.height = width;
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlacedItem {
    item: Item,
    position: Position,
}

impl PlacedItem {
    fn new(item: Item, position: Position) -> Self {
        Self { item, position }
    }
    fn place_on_empty_sheet(item: &mut Item) -> Self {
        item.align_item();
        PlacedItem::new(item.to_owned(), Position::new(0, 0))
    }

    fn next_possible_column(&self) -> Position {
        Position::new(
            self.position.x + 1 + self.item.width + TOOL_WIDTH,
            self.position.y,
        )
    }

    fn next_possible_row(&self) -> Position {
        Position::new(
            self.position.x,
            self.position.y + self.item.height + TOOL_WIDTH,
        )
    }
}

#[derive(Clone, Debug, Default)]
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
