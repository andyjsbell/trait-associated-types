#![allow(dead_code)]

use std::collections::HashMap;

struct Hero {
    props : HashMap<u8, u32>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
