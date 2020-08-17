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

type Prop = u8;
type Value = u32;
pub struct Hero1 {
    props: HashMap<Prop, Value>
}

impl Hero1 {
    pub fn new() -> Self {
        Self {
            props: HashMap::new()
        }
    }

    pub fn set_prop(&mut self, prop: Prop, value: Value) {
        self.props.insert(prop, value);
    }

    pub fn get_prop(&self, prop: Prop) -> Value {
        *self.props.get(&prop).unwrap_or(&0)
    }
}

use std::cmp::Eq;
use std::hash::Hash;
use num::traits::{Zero};

pub struct Hero2<P, V> {
    props: HashMap<P, V>
}

impl<P: Hash + Eq, V: Zero + Copy> Hero2<P, V> {
    pub fn new() -> Self {
        Self {
            props: HashMap::new()
        }
    }

    pub fn set_prop(&mut self, prop: P, value: V) {
        self.props.insert(prop, value);
    }

    pub fn get_prop(&self, prop: P) -> V {
        *self.props.get(&prop).unwrap_or(&V::zero())
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

    #[test]
    fn second_hero() {
        let mut hero = Hero1::new();
        hero.set_prop(1, 100);
        assert_eq!(100, hero.get_prop(1));
    }

    #[test]
    fn third_hero() {
        let mut hero = Hero2::<u8, u32>::new();
        hero.set_prop(1, 100);
        assert_eq!(100, hero.get_prop(1));
    }
}
