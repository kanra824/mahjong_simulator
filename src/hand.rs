use crate::tile::{Tile, Yuan};
use crate::ResultShape;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Hand {
    tiles: Vec<Tile>,
    // 鳴いた牌
    called: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
impl Hand {
    pub fn new() -> Self {
        let tiles = vec![
            Tile::Y(Yuan::Haku),
            Tile::Y(Yuan::Haku),
            Tile::Y(Yuan::Haku),
            Tile::Y(Yuan::Hatsu),
            Tile::Y(Yuan::Hatsu),
            Tile::Y(Yuan::Hatsu),
            Tile::Y(Yuan::Chun),
            Tile::Y(Yuan::Chun),
            Tile::Y(Yuan::Chun),
            Tile::M(1, false),
            Tile::M(1, false),
            Tile::M(1, false),
            Tile::M(9, false),
        ];

        Hand {
            tiles,
            called: vec![],
        }
    }

    pub fn sort(&mut self) {
        self.tiles.sort();
    }

    pub fn enumerate_result_shape(
        &self,
        last: Tile,
        me: usize,
        house: usize,
        win: Option<usize>,
    ) -> Vec<ResultShape> {
        // アガリ判定をするのは場の状態を持つ struct では？
        // そのほうがリーチや天和、地和の判定がしやすい
        // 場は各プレイヤーの Hand をもっているはずだから、ここで見れる情報は場のクラスも見れる。
        unimplemented!();
    }

    pub fn calc_score(&self) {}
}
