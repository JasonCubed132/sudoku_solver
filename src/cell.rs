use std::collections::HashSet;

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