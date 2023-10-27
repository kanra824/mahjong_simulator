use crate::result_shape::{OtherInfo, ResultShape};
use crate::tile::Tile;
use crate::yaku::Yaku;

pub fn check_chinitsu(result_shape: &ResultShape) -> (Option<Yaku>, Option<Yaku>) {
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
            _ => {
                check_m = false;
                check_s = false;
                check_p = false;
            }
        }
    }

    if check_m || check_s || check_p {
        let menzen = result_shape.other.contains(&OtherInfo::Menzen);
        (Some(Yaku::Chinitsu(menzen)), Some(Yaku::Honitsu(menzen)))
    } else {
        (None, None)
    }
}
