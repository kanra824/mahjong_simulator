use crate::tile::{Tile, Wind, Yuan};
use crate::yaku::Yaku;
use crate::Wait;
use crate::{OtherInfo, ResultShape};

// ドラ
pub fn check_dora(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.dora_cnt > 0 {
        return (Some(Yaku::Dora(result_shape.dora_cnt)), None);
    }
    (None, None)
}

// 一翻
pub fn check_reach(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Reach) {
        return (Some(Yaku::Reach), None);
    }
    (None, None)
}

pub fn check_ippatsu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Ippatsu) {
        return (Some(Yaku::Ippatsu), None);
    }
    (None, None)
}

pub fn check_tsumo(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.win.is_none() && result_shape.other.contains(&OtherInfo::Menzen) {
        return (Some(Yaku::Tsumo), None);
    }
    (None, None)
}

pub fn check_yakuhai(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            continue;
        }
        match mentsu[0] {
            Tile::W(Wind::East) => {
                let field_is_east = result_shape.field_wind == Wind::East;
                let mine_is_east = result_shape.wind == Wind::East;
                if field_is_east && mine_is_east {
                    return (Some(Yaku::DoubleEast), None);
                } else if field_is_east || mine_is_east {
                    return (Some(Yaku::East), None);
                }
            }
            Tile::W(Wind::South) => {
                let field_is_south = result_shape.field_wind == Wind::South;
                let mine_is_south = result_shape.wind == Wind::South;
                if field_is_south && mine_is_south {
                    return (Some(Yaku::DoubleSouth), None);
                } else if field_is_south || mine_is_south {
                    return (Some(Yaku::South), None);
                }
            }
            Tile::W(Wind::West) => {
                let mine_is_west = result_shape.wind == Wind::West;
                if mine_is_west {
                    return (Some(Yaku::West), None);
                }
            }
            Tile::W(Wind::North) => {
                let mine_is_north = result_shape.wind == Wind::North;
                if mine_is_north {
                    return (Some(Yaku::North), None);
                }
            }
            Tile::Y(Yuan::Haku) => {
                return (Some(Yaku::Haku), None);
            }
            Tile::Y(Yuan::Hatsu) => {
                return (Some(Yaku::Hatsu), None);
            }
            Tile::Y(Yuan::Chun) => {
                return (Some(Yaku::Chun), None);
            }
            _ => (),
        }
    }
    (None, None)
}

pub fn check_tanyao(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.head.is_char_19() {
        return (None, None);
    }

    let tanyao = result_shape
        .body
        .iter()
        .all(|(mentsu, _)| mentsu.iter().all(|tile| !tile.is_char_19()));

    if tanyao {
        (Some(Yaku::Tanyao), None)
    } else {
        (None, None)
    }
}

pub fn check_pinfu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.wait != Wait::Two {
        return (None, None);
    }
    if !result_shape.other.contains(&OtherInfo::Menzen) {
        return (None, None);
    }

    let shuntsu = result_shape
        .body
        .iter()
        .all(|(mentsu, _)| result_shape.is_shuntsu(mentsu));
    if !shuntsu {
        return (None, None);
    }

    if result_shape.head.is_char() {
        return (None, None);
    }

    (Some(Yaku::Pinfu), None)
}

pub fn check_ipeko(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if !result_shape.other.contains(&OtherInfo::Menzen) {
        return (None, None);
    }
    let mut ipeko = false;
    for i in 0..result_shape.body.len() - 1 {
        let (mentsu1, _) = &result_shape.body[i];
        if !result_shape.is_shuntsu(mentsu1) {
            continue;
        }
        for j in i + 1..result_shape.body.len() {
            let (mentsu2, _) = &result_shape.body[j];
            if !result_shape.is_shuntsu(mentsu2) {
                continue;
            }
            if mentsu1 == mentsu2 {
                ipeko = true;
            }
        }
    }

    if ipeko {
        (Some(Yaku::Ipeko), None)
    } else {
        (None, None)
    }
}

pub fn check_haitei(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Haitei) {
        (Some(Yaku::Haitei), None)
    } else {
        (None, None)
    }
}

pub fn check_houtei(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Houtei) {
        (Some(Yaku::Houtei), None)
    } else {
        (None, None)
    }
}

pub fn check_rinshan(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Rinshan) {
        (Some(Yaku::Rinshan), None)
    } else {
        (None, None)
    }
}

pub fn check_chankan(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Chankan) {
        (Some(Yaku::Chankan), None)
    } else {
        (None, None)
    }
}
