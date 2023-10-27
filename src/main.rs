mod hand;
mod tile;
mod yaku;
mod yaku_check;
mod result_shape;

use hand::Hand;
use tile::{Tile, Wind, Yuan};
use yaku::Yaku;
use yaku_check::{han_1::*, han_2::*, han_3::*, han_6::*, yakuman::*};

use std::{collections::HashMap, collections::HashSet};

#[derive(PartialEq, Eq)]
pub enum Wait {
    Head,  // 単騎
    One,   // カンチャン、ペンチャン
    Two,   // 両面
    Other, // シャボ、その他複合
}

pub enum Fuuro {
    Chi,
    Pon,
    Minkan,
    Ankan,
    Kakan,
    None, // 鳴いていない,
}

fn main() {
    let mut hand = Hand::new();
    hand.sort();
    println!("{:?}", hand);
}
