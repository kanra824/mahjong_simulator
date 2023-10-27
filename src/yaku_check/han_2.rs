use crate::result_shape::{OtherInfo, ResultShape};
use crate::yaku::Yaku;
use crate::Fuuro;
use crate::Tile;

// 二翻
pub fn check_double_reach(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::DoubleReach) {
        (Some(Yaku::DoubleReach), None)
    } else {
        (None, None)
    }
}

pub fn check_sansyoku_dojun(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut sansyoku_dojun = false;
    for i in 0..result_shape.body.len() - 2 {
        let (mentsu1, _) = &result_shape.body[i];
        if !result_shape.is_shuntsu(mentsu1) {
            continue;
        }
        for j in i + 1..result_shape.body.len() - 1 {
            let (mentsu2, _) = &result_shape.body[j];
            if !result_shape.is_shuntsu(mentsu2) {
                continue;
            }
            for k in j + 1..result_shape.body.len() {
                let (mentsu3, _) = &result_shape.body[k];
                if !result_shape.is_shuntsu(mentsu3) {
                    continue;
                }
                if mentsu1[0] == mentsu2[0] && mentsu2[0] == mentsu3[0] {
                    sansyoku_dojun = true;
                }
            }
        }
    }

    if sansyoku_dojun {
        (
            Some(Yaku::SansyokuDoujun(
                result_shape.other.contains(&OtherInfo::Menzen),
            )),
            None,
        )
    } else {
        (None, None)
    }
}

pub fn check_sansyoku_doko(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut sansyoku_doko = false;
    for i in 0..result_shape.body.len() - 2 {
        let (mentsu1, _) = &result_shape.body[i];
        if result_shape.is_shuntsu(mentsu1) {
            continue;
        }
        for j in i + 1..result_shape.body.len() - 1 {
            let (mentsu2, _) = &result_shape.body[j];
            if result_shape.is_shuntsu(mentsu2) {
                continue;
            }
            for k in j + 1..result_shape.body.len() {
                let (mentsu3, _) = &result_shape.body[k];
                if result_shape.is_shuntsu(mentsu3) {
                    continue;
                }
                if mentsu1[0] == mentsu2[0] && mentsu2[0] == mentsu3[0] {
                    sansyoku_doko = true;
                }
            }
        }
    }

    if sansyoku_doko {
        (Some(Yaku::SansyokuDoko), None)
    } else {
        (None, None)
    }
}

pub fn check_sananko(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_anko = 0;
    for (mentsu, fuuro) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            continue;
        }
        match fuuro {
            Fuuro::None | Fuuro::Ankan => {
                cnt_anko += 1;
            }
            _ => (),
        }
    }
    if cnt_anko == 3 {
        (Some(Yaku::Sanankou), None)
    } else {
        (None, None)
    }
}

pub fn check_ikkitsukan(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut check_m = vec![false; 3];
    let mut check_s = vec![false; 3];
    let mut check_p = vec![false; 3];
    for (mentsu, _) in result_shape.body.iter() {
        if !result_shape.is_shuntsu(mentsu) {
            continue;
        }
        match mentsu[0] {
            Tile::M(1, _) => {
                check_m[0] = true;
            }
            Tile::M(4, _) => {
                check_m[1] = true;
            }
            Tile::M(7, _) => {
                check_m[2] = true;
            }
            Tile::S(1, _) => {
                check_s[0] = true;
            }
            Tile::S(4, _) => {
                check_s[1] = true;
            }
            Tile::S(7, _) => {
                check_s[2] = true;
            }
            Tile::P(1, _) => {
                check_p[0] = true;
            }
            Tile::P(4, _) => {
                check_p[1] = true;
            }
            Tile::P(7, _) => {
                check_p[2] = true;
            }
            _ => (),
        }
    }

    if check_m.iter().fold(true, |acc, x| acc && *x)
        || check_s.iter().fold(true, |acc, x| acc && *x)
        || check_p.iter().fold(true, |acc, x| acc && *x)
    {
        (
            Some(Yaku::IkkiTsukan(
                result_shape.other.contains(&OtherInfo::Menzen),
            )),
            None,
        )
    } else {
        (None, None)
    }
}

pub fn check_chitoitsu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Chitoitsu) {
        (Some(Yaku::Chitoitsu), None)
    } else {
        (None, None)
    }
}

pub fn check_toitoiho(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut toitoiho = true;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            toitoiho = false;
            break;
        }
    }

    if toitoiho {
        (Some(Yaku::Toitoiho), None)
    } else {
        (None, None)
    }
}

pub fn check_honchan(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut honchan = true;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            honchan = honchan
                && match mentsu[0] {
                    Tile::M(1, _) | Tile::M(7, _) => true,
                    Tile::S(1, _) | Tile::S(7, _) => true,
                    Tile::P(1, _) | Tile::P(7, _) => true,
                    _ => false,
                }
        } else {
            honchan = honchan && mentsu[0].is_char_19();
        }
    }

    if honchan {
        (
            Some(Yaku::HonChan(
                result_shape.other.contains(&OtherInfo::Menzen),
            )),
            None,
        )
    } else {
        (None, None)
    }
}

pub fn check_sankantsu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_kan = 0;
    for (mentsu, fuuro) in result_shape.body.iter() {
        match fuuro {
            Fuuro::Minkan | Fuuro::Ankan => {
                cnt_kan += 1;
            }
            _ => (),
        }
    }
    if cnt_kan == 3 {
        (Some(Yaku::Sankantsu), None)
    } else {
        (None, None)
    }
}

pub fn check_shosangen(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_yuan = 0;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            continue;
        }
        match mentsu[0] {
            Tile::Y(_) => {
                cnt_yuan += 1;
            }
            _ => (),
        }
    }
    match result_shape.head {
        Tile::Y(_) if cnt_yuan == 2 => (Some(Yaku::Syousangen), None),
        _ => (None, None),
    }
}

pub fn check_honroutou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut honroutou = true;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) || !mentsu[0].is_char_19() {
            honroutou = false;
            break;
        }
    }
    if honroutou {
        (Some(Yaku::Honroutou), None)
    } else {
        (None, None)
    }
}
