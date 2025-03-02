
use crate::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        self.items.get(index)
    }

    pub fn get_by_index2(&self, index: usize) -> MightAValue {
        match self.items.get(index) {
            Some(media) => MightAValue::Value(media),
            None => MightAValue::NoValue,
        }
    }
}

#[derive(Debug)]
pub enum MightAValue<'a> {
    Value(&'a Media),
    NoValue,
}