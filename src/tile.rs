#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
pub enum Yuan {
    Haku,
    Hatsu,
    Chun,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
pub enum Tile {
    S(usize, bool),
    M(usize, bool),
    P(usize, bool),
    W(Wind),
    Y(Yuan),
}

impl Tile {
    pub fn is_char(&self) -> bool {
        match self {
            Tile::W(_) | Tile::Y(_) => true,
            _ => false,
        }
    }

    pub fn is_19(&self) -> bool {
        match self {
            Tile::M(1, _)
            | Tile::M(9, _)
            | Tile::S(1, _)
            | Tile::S(9, _)
            | Tile::P(1, _)
            | Tile::P(9, _) => true,
            _ => false,
        }
    }

    pub fn is_char_19(&self) -> bool {
        self.is_char() || self.is_19()
    }
}
