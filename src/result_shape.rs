use std::collections::{HashSet, HashMap};

use crate::tile::{Tile, Wind};
use crate::yaku::Yaku;
use crate::yaku_check::{han_1, han_2, han_3, han_6, yakuman};
use crate::{Fuuro, Wait};

#[derive(PartialEq, Eq, Hash)]
pub enum OtherInfo {
    Yakuman,      // 役満で上がった
    Chitoitsu,    // 七対子
    Kokushimusou, // 国士無双
    Churenpoutou, // 九蓮宝燈
    Rinshan,      // 嶺上開花
    Ippatsu,      // 一発
    Menzen,       // 門前
    Haitei,       // 海底摸月
    Houtei,       // 河底撈魚
    Chankan,      // 槍槓
    Reach,        // リーチ
    DoubleReach,  // ダブリー
    Chihou,       // 地和
    Tenhou,       // 天和
}

// アガリ形
// 点数計算に必要な情報を格納する
pub struct ResultShape {
    pub wait: Wait,                    // アガリ時の待ち
    pub last: Tile,                    // 最後にツモった牌
    pub head: Tile,                    // アタマ
    pub body: Vec<(Vec<Tile>, Fuuro)>, // 3 * 4 のメンツ TODO: 登録時にバリデーションする. 順子の場合ソートも行う
    pub yaku: HashSet<Yaku>,           // calc_han() で役が登録される
    pub win: Option<usize>,            // ロンした相手. ツモの場合は None
    pub dora_cnt: usize,               // ドラの数
    pub field_wind: Wind,              // 場風
    pub wind: Wind,                    // 自風
    pub me: usize,                     // 自分のプレイヤーID
    pub house: usize,                  // 親のプレイヤーID
    pub other: HashSet<OtherInfo>,     // その他の情報
}

impl ResultShape {
    pub fn new(
        wait: Wait,
        last: Tile,
        head: Tile,
        body: Vec<(Vec<Tile>, Fuuro)>,
        yaku: HashSet<Yaku>,
        win: Option<usize>,
        dora_cnt: usize,
        field_wind: Wind,
        wind: Wind,
        me: usize,
        house: usize,
        other: HashSet<OtherInfo>,
    ) -> Self {
        let mut res = ResultShape {
            wait,
            last,
            head,
            body,
            yaku,
            win,
            dora_cnt,
            field_wind,
            wind,
            me,
            house,
            other,
        };
        res.upd_yaku();
        res.validate();
        res
    }

    pub fn is_shuntsu(&self, body: &Vec<Tile>) -> bool {
        if body[0].is_char() {
            return false;
        }
        if body[0] == body[1] {
            return false;
        }

        return true;
    }

    pub fn validate(&self) {
        // body がメンツになっているかどうかチェック
        unimplemented!();
    }

    pub fn set_yaku<F: Fn(&ResultShape) -> (Option<Yaku>, Option<Yaku>)>(&mut self, check: F) {
        let res = check(&self);
        if let Some(yaku) = res.0 {
            self.yaku.insert(yaku);
            if let Some(remove_yaku) = res.1 {
                self.yaku.remove(&remove_yaku);
            }
        }
    }

    pub fn check_yakuman(&mut self) {
        let yaku_v = vec![
            yakuman::check_suanko,
            yakuman::check_daisangen,
            yakuman::check_kokushimusou,
            yakuman::check_ryuiisou,
            yakuman::check_tsuiisou,
            yakuman::check_chinroto,
            yakuman::check_sukantsu,
            yakuman::check_syousuushi,
            yakuman::check_churen_poutou,
            yakuman::check_chihou,
            yakuman::check_tenho,
            yakuman::check_suanko_tanki,
            yakuman::check_kokushimusou_13,
            yakuman::check_junsei_churenpoutou,
            yakuman::check_daisuushi,
        ];

        for yaku in yaku_v {
            self.set_yaku(yaku);
        }
    }

    pub fn check_1han(&mut self) {
        let yaku_v = vec![
            han_1::check_reach,
            han_1::check_ippatsu,
            han_1::check_tsumo,
            han_1::check_yakuhai,
            han_1::check_tanyao,
            han_1::check_pinfu,
            han_1::check_ipeko,
            han_1::check_haitei,
            han_1::check_houtei,
            han_1::check_rinshan,
            han_1::check_chankan,
        ];
        for yaku in yaku_v {
            self.set_yaku(yaku);
        }
    }

    pub fn check_2han(&mut self) {
        let yaku_v = vec![
            han_2::check_double_reach,
            han_2::check_sansyoku_dojun,
            han_2::check_sansyoku_doko,
            han_2::check_sananko,
            han_2::check_ikkitsukan,
            han_2::check_chitoitsu,
            han_2::check_toitoiho,
            han_2::check_honchan,
            han_2::check_sankantsu,
            han_2::check_shosangen,
            han_2::check_honroutou,
        ];
        for yaku in yaku_v {
            self.set_yaku(yaku);
        }
    }

    pub fn check_3han(&mut self) {
        let yaku_v = vec![
            han_3::check_ryanpeko,
            han_3::check_junchan,
            han_3::check_honitsu,
        ];
        for yaku in yaku_v {
            self.set_yaku(yaku);
        }
    }

    pub fn check_6han(&mut self) {
        self.set_yaku(han_6::check_chinitsu);
    }

    pub fn upd_yaku(&mut self) {
        // 役満は役満以外と同時に成立しない
        self.check_yakuman();
        if self.yaku.len() > 0 {
            return;
        }
        self.check_1han();
        self.check_2han();
        self.check_3han();
        self.check_6han();
    }

    pub fn calc_yakuman(&self) -> usize {
        let mut res = 0;
        for yaku in self.yaku.iter() {
            res += yaku.yakuman_cnt();
        }
        res
    }

    pub fn calc_han(&self) -> usize {
        let mut res = 0;
        for yaku in self.yaku.iter() {
            res += yaku.han();
        }
        res
    }

    pub fn calc_fu(&self) -> usize {
        if self.other.contains(&OtherInfo::Chitoitsu) {
            return 25;
        }

        if self.other.contains(&OtherInfo::Yakuman) {
            return 0;
        }

        if self.yaku.contains(&Yaku::Pinfu) && self.win.is_none() {
            return 20;
        }

        let mut res = 20;
        if self.win.is_none() {
            res += 2;
        } else if self.other.contains(&OtherInfo::Menzen) {
            res += 10;
        }

        for (mentsu, fuuro) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                continue;
            }

            if mentsu[0].is_char_19() {
                match fuuro {
                    Fuuro::None => {
                        res += 4;
                    }
                    Fuuro::Kakan | Fuuro::Minkan => {
                        res += 8;
                    }
                    Fuuro::Ankan => {
                        res += 16;
                    }
                    _ => res += 2,
                }
            } else {
                match fuuro {
                    Fuuro::None => {
                        res += 8;
                    }
                    Fuuro::Kakan | Fuuro::Minkan => {
                        res += 16;
                    }
                    Fuuro::Ankan => {
                        res += 32;
                    }
                    _ => res += 4,
                }
            }
        }

        match &self.head {
            Tile::Y(_) => {
                res += 2;
            }
            Tile::W(wind) if *wind == self.wind || *wind == self.field_wind => {
                res += 2;
            }
            _ => (),
        }

        match self.wait {
            Wait::One | Wait::Head => {
                res += 2;
            }
            _ => (),
        }

        if res % 10 > 0 {
            res += 10 - res % 10;
        }

        res
    }

    pub fn calc_score(&self) -> Vec<i32> {
        if self.other.contains(&OtherInfo::Yakuman) {
            let yakuman = self.calc_yakuman();
            let val = self.score_val(
                self.me == self.house,
                self.win.is_none(),
                0,
                0,
                yakuman as i32,
            );
            return val;
        } else {
            let han = self.calc_han();
            let fu = self.calc_fu();
            let val = self.score_val(self.me == self.house, self.win.is_none(), han, fu, 0);
            return val;
        }
    }

    pub fn ron(&self, val: i32) -> Vec<i32> {
        let mut res = vec![0; 4];
        res[self.win.unwrap()] = val;
        res
    }

    pub fn house_tsumo(&self, val: i32) -> Vec<i32> {
        let mut res = vec![val; 4];
        res[self.me] = 0;
        res
    }

    pub fn child_tsumo(&self, val: (i32, i32)) -> Vec<i32> {
        let mut res = vec![val.0; 4];
        res[self.me] = 0;
        res[self.house] = val.1;
        res
    }

    pub fn score_val(
        &self,
        house: bool,
        tsumo: bool,
        han: usize,
        fu: usize,
        yakuman: i32,
    ) -> Vec<i32> {
        if yakuman > 0 {
            if house && tsumo {
                return self.house_tsumo(16000 * yakuman);
            } else if house && !tsumo {
                return self.ron(48000 * yakuman);
            } else if !house && tsumo {
                return self.child_tsumo((8000 * yakuman, 16000 * yakuman));
            } else {
                return self.ron(32000 * yakuman);
            }
        }

        if han >= 5 || han == 4 && fu >= 40 || han == 3 && fu >= 70 {
            // 満貫
            if house && tsumo {
                return self.house_tsumo(4000);
            } else if house && !tsumo {
                return self.ron(12000);
            } else if !house && tsumo {
                return self.child_tsumo((2000, 4000));
            } else {
                return self.ron(8000);
            }
        }

        if 6 <= han && han <= 7 {
            // 跳満
            if house && tsumo {
                return self.house_tsumo(6000);
            } else if house && !tsumo {
                return self.ron(18000);
            } else if !house && tsumo {
                return self.child_tsumo((3000, 6000));
            } else {
                return self.ron(12000);
            }
        }

        if 8 <= han && han <= 10 {
            // 倍満
            if house && tsumo {
                return self.house_tsumo(8000);
            } else if house && !tsumo {
                return self.ron(24000);
            } else if !house && tsumo {
                return self.child_tsumo((4000, 8000));
            } else {
                return self.ron(16000);
            }
        }

        if 11 <= han && han <= 12 {
            // 三倍満
            if house && tsumo {
                return self.house_tsumo(12000);
            } else if house && !tsumo {
                return self.ron(36000);
            } else if !house && tsumo {
                return self.child_tsumo((6000, 12000));
            } else {
                return self.ron(24000);
            }
        }

        if 13 <= han {
            // 数え役満
            if house && tsumo {
                return self.house_tsumo(16000);
            } else if house && !tsumo {
                return self.ron(48000);
            } else if !house && tsumo {
                return self.child_tsumo((8000, 16000));
            } else {
                return self.ron(32000);
            }
        }

        if house && tsumo {
            // 親のツモ
            let score_table = HashMap::from([
                (20, [None, Some(700), Some(1300), Some(2600)]),
                (25, [None, None, Some(1600), Some(3200)]),
                (30, [Some(500), Some(1000), Some(2000), Some(3900)]),
                (40, [Some(700), Some(1300), Some(2600), None]),
                (50, [Some(800), Some(1600), Some(3200), None]),
                (60, [Some(1000), Some(2000), Some(3900), None]),
                (70, [Some(1200), Some(2300), None, None]),
                (80, [Some(1300), Some(2600), None, None]),
                (90, [Some(1500), Some(2900), None, None]),
                (100, [Some(1600), Some(3200), None, None]),
                (110, [Some(1800), Some(3600), None, None]),
            ]);
            let val = score_table.get(&fu).unwrap()[han - 1].unwrap();
            return self.house_tsumo(val);
        } else if house && !tsumo {
            // 親のロン
            let score_table = HashMap::from([
                (25, [None, Some(2400), Some(4800), Some(9600)]),
                (30, [Some(1500), Some(2900), Some(5800), Some(11600)]),
                (40, [Some(2000), Some(3900), Some(7700), None]),
                (50, [Some(2400), Some(4800), Some(9600), None]),
                (60, [Some(2900), Some(5800), Some(11600), None]),
                (70, [Some(3400), Some(6800), None, None]),
                (80, [Some(3900), Some(7700), None, None]),
                (90, [Some(4400), Some(8700), None, None]),
                (100, [Some(4800), Some(9600), None, None]),
                (110, [Some(5300), Some(10600), None, None]),
            ]);
            let val = score_table.get(&fu).unwrap()[han - 1].unwrap();
            return self.ron(val);
        } else if !house && tsumo {
            // 子のツモ
            let score_table = HashMap::from([
                (
                    20,
                    [
                        None,
                        Some((400, 700)),
                        Some((700, 1300)),
                        Some((1300, 2600)),
                    ],
                ),
                (25, [None, None, Some((800, 1600)), Some((1600, 3200))]),
                (
                    30,
                    [
                        Some((300, 500)),
                        Some((500, 1000)),
                        Some((1000, 2000)),
                        Some((2000, 3900)),
                    ],
                ),
                (
                    40,
                    [
                        Some((400, 700)),
                        Some((700, 1300)),
                        Some((1300, 2600)),
                        None,
                    ],
                ),
                (
                    50,
                    [
                        Some((400, 800)),
                        Some((800, 1600)),
                        Some((1600, 3200)),
                        None,
                    ],
                ),
                (
                    60,
                    [
                        Some((500, 1000)),
                        Some((1000, 2000)),
                        Some((2000, 3900)),
                        None,
                    ],
                ),
                (70, [Some((600, 1200)), Some((1200, 2300)), None, None]),
                (80, [Some((700, 1300)), Some((1300, 2600)), None, None]),
                (90, [Some((800, 1500)), Some((1500, 2900)), None, None]),
                (100, [Some((800, 1600)), Some((1600, 3200)), None, None]),
                (110, [Some((900, 1800)), Some((1800, 3600)), None, None]),
            ]);
            let val = score_table.get(&fu).unwrap()[han - 1].unwrap();
            return self.child_tsumo(val);
        } else {
            // 子のロン
            let score_table = HashMap::from([
                (25, [None, Some(1600), Some(3200), Some(6400)]),
                (30, [Some(1000), Some(2000), Some(3900), Some(7700)]),
                (40, [Some(1300), Some(2600), Some(5200), None]),
                (50, [Some(1600), Some(3200), Some(6400), None]),
                (60, [Some(2000), Some(3900), Some(7700), None]),
                (70, [Some(2300), Some(4500), None, None]),
                (80, [Some(2600), Some(5200), None, None]),
                (90, [Some(2900), Some(5800), None, None]),
                (100, [Some(3200), Some(6400), None, None]),
                (110, [Some(3600), Some(7100), None, None]),
            ]);
            let val = score_table.get(&fu).unwrap()[han - 1].unwrap();
            return self.ron(val);
        }
    }
}
