use crate::{ResultShape, OtherInfo, Wait};
use crate::yaku::Yaku;
use crate::tile::{Tile, Yuan, Wind};
use crate::Fuuro;

pub fn check_suanko(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
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
    if cnt_anko == 4 {
        (Some(Yaku::SuAnko), None)
    } else {
        (None, None)
    }
}

pub fn check_daisangen(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
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
    if cnt_yuan == 3 {
        (Some(Yaku::Daisangen), None)
    } else {
        (None, None)
    }
}

pub fn check_kokushimusou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Kokushimusou) {
        (Some(Yaku::KokushiMusou), None)
    } else {
        (None, None)
    }
}

pub fn check_ryuiisou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut ryuiisou = true;
    for (mentsu, _) in result_shape.body.iter() {
        for tile in mentsu {
            match tile {
                Tile::Y(Yuan::Hatsu)
                | Tile::S(2, _)
                | Tile::S(3, _)
                | Tile::S(4, _)
                | Tile::S(6, _)
                | Tile::S(8, _) => (),
                _ => {
                    ryuiisou = false;
                    break;
                }
            }
        }
        if !ryuiisou {
            break;
        }
    }

    match result_shape.head {
        Tile::Y(Yuan::Hatsu)
        | Tile::S(2, _)
        | Tile::S(3, _)
        | Tile::S(4, _)
        | Tile::S(6, _)
        | Tile::S(8, _) => (),
        _ => {
            ryuiisou = false;
        }
    }

    if ryuiisou {
        (Some(Yaku::Ryuiisou), None)
    } else {
        (None, None)
    }
}

pub fn check_tsuiisou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut tsuiisou = true;
    for (mentsu, _) in result_shape.body.iter() {
        for tile in mentsu {
            match tile {
                Tile::Y(_) | Tile::W(_) => (),
                _ => {
                    tsuiisou = false;
                    break;
                }
            }
        }
        if !tsuiisou {
            break;
        }
    }

    match result_shape.head {
        Tile::Y(_) | Tile::W(_) => (),
        _ => {
            tsuiisou = false;
        }
    }

    if tsuiisou {
        (Some(Yaku::Tsuiisou), None)
    } else {
        (None, None)
    }
}

pub fn check_chinroto(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut chinroto = true;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            chinroto = false;
            break;
        }
        chinroto = chinroto && mentsu[0].is_19();
    }

    if chinroto {
        (Some(Yaku::Chinroutou), None)
    } else {
        (None, None)
    }
}

pub fn check_sukantsu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_kan = 0;
    for (mentsu, fuuro) in result_shape.body.iter() {
        match fuuro {
            Fuuro::Minkan | Fuuro::Ankan => {
                cnt_kan += 1;
            }
            _ => (),
        }
    }
    if cnt_kan == 4 {
        (Some(Yaku::Suukantsu), None)
    } else {
        (None, None)
    }
}

pub fn check_syousuushi(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_wind = 0;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            continue;
        }
        match mentsu[0] {
            Tile::W(_) => {
                cnt_wind += 1;
            }
            _ => (),
        }
    }
    match result_shape.head {
        Tile::W(_) => {
            cnt_wind += 1;
        }
        _ => (),
    }
    if cnt_wind == 4 {
        (Some(Yaku::Syousuushi), None)
    } else {
        (None, None)
    }
}

pub fn check_churen_poutou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Churenpoutou) {
        (Some(Yaku::ChurenPoutou), None)
    } else {
        (None, None)
    }
}

pub fn check_chihou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Chihou) {
        (Some(Yaku::Chihou), None)
    } else {
        (None, None)
    }
}

pub fn check_tenho(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.other.contains(&OtherInfo::Tenhou) {
        (Some(Yaku::Tenhou), None)
    } else {
        (None, None)
    }
}

pub fn check_suanko_tanki(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
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
    if cnt_anko == 4 && result_shape.wait == Wait::One {
        (Some(Yaku::SuankoTanki), Some(Yaku::SuAnko))
    } else {
        (None, None)
    }
}

pub fn check_kokushimusou_13(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if result_shape.head == result_shape.last {
        (Some(Yaku::KokushiMusou13), Some(Yaku::KokushiMusou))
    } else {
        (None, None)
    }
}

pub fn check_junsei_churenpoutou(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    if !result_shape.other.contains(&OtherInfo::Churenpoutou) {
        return (None, None);
    }
    let mut cnt = vec![0; 9];
    for (mentsu, _) in result_shape.body.iter() {
        match mentsu[0] {
            Tile::M(n, _) => {
                cnt[n - 1] += 1;
            }
            Tile::S(n, _) => {
                cnt[n - 1] += 1;
            }
            Tile::P(n, _) => {
                cnt[n - 1] += 1;
            }
            _ => (),
        }
    }

    let mut junsei = true;
    junsei = junsei && cnt[0] == 3;
    junsei == junsei && cnt[8] == 3;
    for i in 1..=7 {
        junsei = junsei && cnt[i] == 1;
    }

    if junsei {
        (Some(Yaku::JunseiChurenPoutou), Some(Yaku::ChurenPoutou))
    } else {
        (None, None)
    }
}

pub fn check_daisuushi(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
    let mut cnt_wind = 0;
    for (mentsu, _) in result_shape.body.iter() {
        if result_shape.is_shuntsu(mentsu) {
            continue;
        }
        match mentsu[0] {
            Tile::W(_) => {
                cnt_wind += 1;
            }
            _ => (),
        }
    }
    if cnt_wind == 4 {
        (Some(Yaku::Daisuushi), Some(Yaku::Syousuushi))
    } else {
        (None, None)
    }
}