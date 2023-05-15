use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct Cell {
    contents: u32,
    annotations: HashSet<u32>
}

impl Cell {
    pub fn new_blank() -> Self {
        Self { contents: 0, annotations: HashSet::new() }
    }

    pub fn new_from_number(num: u32) -> Self {
        Self { contents: num, annotations: HashSet::new() }
    }

    pub fn set(&mut self, num: u32) {
        self.contents = num;
    }

    pub fn get(&self) -> u32 {
        self.contents
    }

    pub fn to_str_unannotated(&self) -> String {
        if self.contents == 0 {
            String::from(" ")
        } else {
            self.contents.to_string()
        }
    }

    pub fn add_annotation(&mut self, num: u32) -> bool {
        self.annotations.insert(num)
    }

    pub fn remove_annotation(&mut self, num: u32) -> bool {
        self.annotations.remove(&num)
    }

    pub fn set_annotations(&mut self, annotations: HashSet<u32>) {
        self.annotations = annotations;
    }

    pub fn get_annotations(&mut self) -> HashSet<u32> {
        self.annotations.clone()
    }

    pub fn clear_annotations(&mut self) {
        self.annotations.clear()
    }
}

#[cfg(test)]
mod cell_tests {
    use super::*;

    #[test]
    fn can_make_blank_cell() {
        let cell = Cell::new_blank();
        let cell2 = Cell::new_blank();

        assert_eq!(cell, cell2);
    }

    #[test]
    fn can_make_filled_cell() {
        let cell = Cell::new_from_number(1);

        assert_eq!(cell.get(), 1);
    }

    #[test]
    fn can_set_cell() {
        let mut cell = Cell::new_blank();
        cell.set(1);

        assert_eq!(cell.get(), 1);
    }

    #[test]
    fn can_add_annotation() {
        let mut cell = Cell::new_blank();
        cell.add_annotation(1);

        assert_eq!(cell.get_annotations(), HashSet::from([1]));
    }

    #[test]
    fn can_remote_annotation() {
        let mut cell = Cell::new_blank();
        cell.add_annotation(1);
        cell.remove_annotation(1);

        assert_eq!(cell.get_annotations(), HashSet::from([]));
    }

    #[test]
    fn can_set_annotations() {
        let mut cell = Cell::new_blank();
        cell.set_annotations(HashSet::from([1]));

        assert_eq!(cell.get_annotations(), HashSet::from([1]));
    }

    #[test]
    fn can_clear_annotations() {
        let mut cell = Cell::new_blank();
        cell.set_annotations(HashSet::from([1]));
        cell.clear_annotations();

        assert_eq!(cell.get_annotations(), HashSet::from([]));
    }
}