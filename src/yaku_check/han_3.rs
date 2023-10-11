use crate::tile::Tile;
use crate::yaku::Yaku;
use crate::{OtherInfo, ResultShape};

// 三翻

pub fn check_ryanpeko(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_peko = 0;
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
                cnt_peko += 1;
            }
        }
    }

    if result_shape.other.contains(&OtherInfo::Menzen) && cnt_peko == 2 {
        (Some(Yaku::Ryanpeko), Some(Yaku::Ipeko))
    } else {
        (None, None)
    }
}

pub fn check_junchan(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut junchan = true;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            junchan = junchan
                && match mentsu[0] {
                    Tile::M(1, _) | Tile::M(7, _) => true,
                    Tile::S(1, _) | Tile::S(7, _) => true,
                    Tile::P(1, _) | Tile::P(7, _) => true,
                    _ => false,
                }
        } else {
            junchan = junchan && mentsu[0].is_19();
        }
    }

    if junchan {
        (
            Some(Yaku::Junchan(
                result_shape.other.contains(&OtherInfo::Menzen),
            )),
            Some(Yaku::HonChan(
                result_shape.other.contains(&OtherInfo::Menzen),
            )),
        )
    } else {
        (None, None)
    }
}

pub fn check_honitsu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut check_m = true;
    let mut check_s = true;
    let mut check_p = true;
    for (mentsu, _) in result_shape.body.iter() {
        match mentsu[0] {
            Tile::M(_, _) => {
                check_s = false;
                check_p = false;
            }
            Tile::S(_, _) => {
                check_m = false;
                check_p = false;
            }
            Tile::P(_, _) => {
                check_m = false;
                check_s = false;
            }
            _ => (),
        }
    }

    if check_m || check_s || check_p {
        (
            Some(Yaku::Honitsu(
                result_shape.other.contains(&OtherInfo::Menzen),
            )),
            None,
        )
    } else {
        (None, None)
    }
}
