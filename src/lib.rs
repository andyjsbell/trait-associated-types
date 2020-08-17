#![allow(dead_code)]

use std::collections::HashMap;

pub struct Hero {
    props : HashMap<u8, u32>,
}

impl Hero {
    pub fn new() -> Self {
        Self {
            props: HashMap::new()
        }
    }

    pub fn set_prop(&mut self, prop: u8, value: u32) {
        self.props.insert(prop, value);
    }

    pub fn get_prop(&self, prop: u8) -> u32 {
        *self.props.get(&prop).unwrap_or(&0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_hero() {
        let mut hero = Hero::new();
        hero.set_prop(1, 100);
        assert_eq!(100, hero.get_prop(1));
    }
}
