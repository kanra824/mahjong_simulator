use std::{collections::HashMap, collections::HashSet};

#[allow(dead_code)]
struct ScoreTable {
    House: bool, // 親
    Tsumo: bool, // ツモかどうか
    Fu: u8,
    Han: u8,
}

impl ScoreTable {
    fn calc_score() {}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
enum Yuan {
    Haku,
    Hatsu,
    Chun,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
enum Tile {
    S(usize, bool),
    M(usize, bool),
    P(usize, bool),
    W(Wind),
    Y(Yuan),
}

impl Tile {
    fn is_char(&self) -> bool {
        match self {
            Tile::W(_) | Tile::Y(_) => true,
            _ => false,
        }
    }

    fn is_19(&self) -> bool {
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

    fn is_char_19(&self) -> bool {
        self.is_char() || self.is_19()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Hand {
    tiles: Vec<Tile>,
    // 鳴いた牌
    called: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
impl Hand {
    fn new() -> Self {
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

    fn sort(&mut self) {
        self.tiles.sort();
    }

    fn enumerate_result_shape(
        &self,
        last: Tile,
        me: usize,
        house: usize,
        win: Option<usize>,
    ) -> Vec<ResultShape> {
        // calc_wait()
        //

        unimplemented!();
    }

    fn calc_score(&self) {}
}

#[derive(PartialEq, Eq)]
enum Wait {
    Head,  // 単騎
    One,   // カンチャン、ペンチャン
    Two,   // 両面
    Other, // シャボ、その他複合
}

#[derive(PartialEq, Eq, Hash)]
enum Yaku {
    // https://majandofu.com/mahjong-hands-simple
    // ドラ
    Dora(usize), // ドラ

    // 一翻
    Reach,   // 立直
    Ippatsu, // 一発
    Tsumo,   // 門前清自摸和
    Haku,    // 白
    Hatsu,   // 發
    Chun,    // 中
    East,    // 東
    South,   // 南
    West,    // 西
    North,   // 北
    Tanyao,  // 断么九
    Pinfu,   // 平和
    Ipeko,   // 一盃口
    Haitei,  // 海底摸月
    Houtei,  // 河底撈魚
    Rinshan, // 嶺上開花
    Chankan, // 槍槓

    // 二翻
    DoubleReach,          // ダブルリーチ
    SansyokuDoujun(bool), // 三色同順
    SansyokuDoko,         // 三色同刻
    Sanankou,             // 三暗刻
    IkkiTsukan(bool),     // 一気通貫
    Chitoitsu,            // 七対子
    Toitoiho,             // 対々和
    HonChan(bool),        // 混全帯幺九
    Sankantsu,            // 三槓子
    Syousangen,           // 小三元
    Honroutou,            // 混老頭
    DoubleEast,           // ダブ東
    DoubleSouth,          // ダブ南

    // 三翻
    Ryanpeko,      // 二盃口
    Junchan(bool), // 純全帯幺九
    Honitsu(bool), // 混一色

    // 六翻
    Chinitsu(bool), // 清一色

    // 役満
    SuAnko,       // 四暗刻
    Daisangen,    // 大三元
    KokushiMusou, // 国士無双
    Ryuiisou,     // 緑一色
    Tsuiisou,     // 字一色
    Chinroutou,   // 清老頭
    Suukantsu,    // 四槓子
    Syousuushi,   // 小四喜
    ChurenPoutou, // 九蓮宝燈
    Chihou,       // 地和
    Tenhou,       // 天和

    // ダブル役満
    SuankoTanki,        // 四暗刻単騎
    KokushiMusou13,     // 国士無双十三面待ち
    JunseiChurenPoutou, // 純正九蓮宝燈
    Daisuushi,          // 大四喜
}

impl Yaku {
    fn han(&self) -> usize {
        match *self {
            Self::Dora(val) => val,
            Self::Reach
            | Self::Ippatsu
            | Self::Tsumo
            | Self::Haku
            | Self::Hatsu
            | Self::Chun
            | Self::East
            | Self::South
            | Self::West
            | Self::North
            | Self::Tanyao
            | Self::Pinfu
            | Self::Ipeko
            | Self::Haitei
            | Self::Houtei
            | Self::Rinshan
            | Self::Chankan
            | Self::SansyokuDoujun(true)
            | Self::IkkiTsukan(true)
            | Self::HonChan(true) => 1,
            Self::DoubleReach
            | Self::SansyokuDoujun(false)
            | Self::SansyokuDoko
            | Self::Sanankou
            | Self::IkkiTsukan(false)
            | Self::Chitoitsu
            | Self::Toitoiho
            | Self::HonChan(false)
            | Self::Sankantsu
            | Self::Syousangen
            | Self::Honroutou
            | Self::DoubleEast
            | Self::DoubleSouth
            | Self::Junchan(true)
            | Self::Honitsu(true) => 2,
            Self::Ryanpeko | Self::Junchan(false) | Self::Honitsu(false) => 3,
            Self::Chinitsu(true) => 5,
            Self::Chinitsu(false) => 6,
            _ => 13,
        }
    }

    fn yakuman_cnt(&self) -> usize {
        match *self {
            Self::SuAnko
            | Self::Daisangen
            | Self::KokushiMusou
            | Self::Ryuiisou
            | Self::Tsuiisou
            | Self::Chinroutou
            | Self::Suukantsu
            | Self::Syousuushi
            | Self::ChurenPoutou
            | Self::Chihou
            | Self::Tenhou => 1,
            Self::SuankoTanki
            | Self::KokushiMusou13
            | Self::JunseiChurenPoutou
            | Self::Daisuushi => 2,
            _ => 0,
        }
    }
}

enum Fuuro {
    Chi,
    Pon,
    Minkan,
    Ankan,
    Kakan,
    None, // 鳴いていない,
}

#[derive(PartialEq, Eq, Hash)]
enum OtherInfo {
    Yakuman,
    Chitoitsu,
    Kokushimusou,
    Churenpoutou,
    Rinshan,
    Ippatsu,
    Menzen,
    Haitei,
    Houtei,
    Chankan,
    Reach,
    DoubleReach,
    Chihou,
    Tenhou,
}

struct ResultShape {
    wait: Wait,
    last: Tile,
    head: Tile,
    body: Vec<(Vec<Tile>, Fuuro)>, // TODO: 登録時にバリデーションする. 順子の場合ソートも行う
    yaku: HashSet<Yaku>,           // calc_han() で役が登録される
    win: Option<usize>,
    dora_cnt: usize,
    field_wind: Wind,
    wind: Wind,
    me: usize,
    house: usize,
    other: HashSet<OtherInfo>,
}

impl ResultShape {
    fn new(
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

    fn is_shuntsu(&self, body: &Vec<Tile>) -> bool {
        if body[0].is_char() {
            return false;
        }
        if body[0] == body[1] {
            return false;
        }

        return true;
    }

    fn validate(&self) {
        unimplemented!();
    }

    // ドラ
    fn check_dora(&mut self) {
        if self.dora_cnt > 0 {
            self.yaku.insert(Yaku::Dora(self.dora_cnt));
        }
    }

    // 一翻

    fn check_reach(&mut self) {
        if self.other.contains(&OtherInfo::Reach) {
            self.yaku.insert(Yaku::Reach);
        }
    }

    fn check_ippatsu(&mut self) {
        if self.other.contains(&OtherInfo::Ippatsu) {
            self.yaku.insert(Yaku::Ippatsu);
        }
    }

    fn check_tsumo(&mut self) {
        if self.win.is_none() && self.other.contains(&OtherInfo::Menzen) {
            self.yaku.insert(Yaku::Tsumo);
        }
    }

    fn check_yakuhai(&mut self) {
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                continue;
            }
            match mentsu[0] {
                Tile::W(Wind::East) => {
                    let field_is_east = self.field_wind == Wind::East;
                    let mine_is_east = self.wind == Wind::East;
                    if field_is_east && mine_is_east {
                        self.yaku.insert(Yaku::DoubleEast);
                    } else if field_is_east || mine_is_east {
                        self.yaku.insert(Yaku::East);
                    }
                }
                Tile::W(Wind::South) => {
                    let field_is_south = self.field_wind == Wind::South;
                    let mine_is_south = self.wind == Wind::South;
                    if field_is_south && mine_is_south {
                        self.yaku.insert(Yaku::DoubleSouth);
                    } else if field_is_south || mine_is_south {
                        self.yaku.insert(Yaku::South);
                    }
                }
                Tile::W(Wind::West) => {
                    let mine_is_west = self.wind == Wind::West;
                    if mine_is_west {
                        self.yaku.insert(Yaku::West);
                    }
                }
                Tile::W(Wind::North) => {
                    let mine_is_north = self.wind == Wind::North;
                    if mine_is_north {
                        self.yaku.insert(Yaku::North);
                    }
                }
                Tile::Y(Yuan::Haku) => {
                    self.yaku.insert(Yaku::Haku);
                }
                Tile::Y(Yuan::Hatsu) => {
                    self.yaku.insert(Yaku::Hatsu);
                }
                Tile::Y(Yuan::Chun) => {
                    self.yaku.insert(Yaku::Chun);
                }
                _ => (),
            }
        }
    }

    fn check_tanyao(&mut self) {
        if self.head.is_char_19() {
            return;
        }

        let tanyao = self
            .body
            .iter()
            .all(|(mentsu, _)| mentsu.iter().all(|tile| !tile.is_char_19()));

        if tanyao {
            self.yaku.insert(Yaku::Tanyao);
        }
    }

    fn check_pinfu(&mut self) {
        if self.wait != Wait::Two {
            return;
        }
        if !self.other.contains(&OtherInfo::Menzen) {
            return;
        }

        let shuntsu = self.body.iter().all(|(mentsu, _)| self.is_shuntsu(mentsu));
        if !shuntsu {
            return;
        }

        if self.head.is_char() {
            return;
        }

        self.yaku.insert(Yaku::Pinfu);
    }

    fn check_ipeko(&mut self) {
        if !self.other.contains(&OtherInfo::Menzen) {
            return;
        }
        let mut ipeko = false;
        for i in 0..self.body.len() - 1 {
            let (mentsu1, _) = &self.body[i];
            if !self.is_shuntsu(mentsu1) {
                continue;
            }
            for j in i + 1..self.body.len() {
                let (mentsu2, _) = &self.body[j];
                if !self.is_shuntsu(mentsu2) {
                    continue;
                }
                if mentsu1 == mentsu2 {
                    ipeko = true;
                }
            }
        }

        if ipeko {
            self.yaku.insert(Yaku::Ipeko);
        }
    }

    fn check_haitei(&mut self) {
        if self.other.contains(&OtherInfo::Haitei) {
            self.yaku.insert(Yaku::Haitei);
        }
    }

    fn check_houtei(&mut self) {
        if self.other.contains(&OtherInfo::Houtei) {
            self.yaku.insert(Yaku::Houtei);
        }
    }

    fn check_rinshan(&mut self) {
        if self.other.contains(&OtherInfo::Rinshan) {
            self.yaku.insert(Yaku::Rinshan);
        }
    }

    fn check_chankan(&mut self) {
        if self.other.contains(&OtherInfo::Chankan) {
            self.yaku.insert(Yaku::Chankan);
        }
    }

    // 二翻
    fn check_double_reach(&mut self) {
        if self.other.contains(&OtherInfo::DoubleReach) {
            self.yaku.insert(Yaku::DoubleReach);
        }
    }

    fn check_sansyoku_dojun(&mut self) {
        let mut sansyoku_dojun = false;
        for i in 0..self.body.len() - 2 {
            let (mentsu1, _) = &self.body[i];
            if !self.is_shuntsu(mentsu1) {
                continue;
            }
            for j in i + 1..self.body.len() - 1 {
                let (mentsu2, _) = &self.body[j];
                if !self.is_shuntsu(mentsu2) {
                    continue;
                }
                for k in j + 1..self.body.len() {
                    let (mentsu3, _) = &self.body[k];
                    if !self.is_shuntsu(mentsu3) {
                        continue;
                    }
                    if mentsu1[0] == mentsu2[0] && mentsu2[0] == mentsu3[0] {
                        sansyoku_dojun = true;
                    }
                }
            }
        }

        if sansyoku_dojun {
            self.yaku.insert(Yaku::SansyokuDoujun(
                self.other.contains(&OtherInfo::Menzen),
            ));
        }
    }

    fn check_sansyoku_doko(&mut self) {
        let mut sansyoku_doko = false;
        for i in 0..self.body.len() - 2 {
            let (mentsu1, _) = &self.body[i];
            if self.is_shuntsu(mentsu1) {
                continue;
            }
            for j in i + 1..self.body.len() - 1 {
                let (mentsu2, _) = &self.body[j];
                if self.is_shuntsu(mentsu2) {
                    continue;
                }
                for k in j + 1..self.body.len() {
                    let (mentsu3, _) = &self.body[k];
                    if self.is_shuntsu(mentsu3) {
                        continue;
                    }
                    if mentsu1[0] == mentsu2[0] && mentsu2[0] == mentsu3[0] {
                        sansyoku_doko = true;
                    }
                }
            }
        }

        if sansyoku_doko {
            self.yaku.insert(Yaku::SansyokuDoko);
        }
    }

    fn check_sananko(&mut self) {
        let mut cnt_anko = 0;
        for (mentsu, fuuro) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
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
            self.yaku.insert(Yaku::Sanankou);
        }
    }

    fn check_ikkitsukan(&mut self) {
        let mut check_m = vec![false; 3];
        let mut check_s = vec![false; 3];
        let mut check_p = vec![false; 3];
        for (mentsu, _) in self.body.iter() {
            if !self.is_shuntsu(mentsu) {
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
            self.yaku
                .insert(Yaku::IkkiTsukan(self.other.contains(&OtherInfo::Menzen)));
        }
    }

    fn check_chitoitsu(&mut self) {
        if self.other.contains(&OtherInfo::Chitoitsu) {
            self.yaku.insert(Yaku::Chitoitsu);
        }
    }

    fn check_toitoiho(&mut self) {
        let mut toitoiho = true;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                toitoiho = false;
                break;
            }
        }

        if toitoiho {
            self.yaku.insert(Yaku::Toitoiho);
        }
    }

    fn check_honchan(&mut self) {
        let mut honchan = true;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
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
            self.yaku
                .insert(Yaku::HonChan(self.other.contains(&OtherInfo::Menzen)));
        }
    }

    fn check_sankantsu(&mut self) {
        let mut cnt_kan = 0;
        for (mentsu, fuuro) in self.body.iter() {
            match fuuro {
                Fuuro::Minkan | Fuuro::Ankan => {
                    cnt_kan += 1;
                }
                _ => (),
            }
        }
        if cnt_kan == 3 {
            self.yaku.insert(Yaku::Sankantsu);
        }
    }

    fn check_shosangen(&mut self) {
        let mut cnt_yuan = 0;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                continue;
            }
            match mentsu[0] {
                Tile::Y(_) => {
                    cnt_yuan += 1;
                }
                _ => (),
            }
        }
        match self.head {
            Tile::Y(_) if cnt_yuan == 2 => {
                self.yaku.insert(Yaku::Syousangen);
            }
            _ => (),
        }
    }

    fn check_honroutou(&mut self) {
        let mut honroutou = true;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) || !mentsu[0].is_char_19() {
                honroutou = false;
                break;
            }
        }
        if honroutou {
            self.yaku.insert(Yaku::Honroutou);
        }
    }

    // 三翻

    fn check_ryanpeko(&mut self) {
        let mut cnt_peko = 0;
        for i in 0..self.body.len() - 1 {
            let (mentsu1, _) = &self.body[i];
            if !self.is_shuntsu(mentsu1) {
                continue;
            }
            for j in i + 1..self.body.len() {
                let (mentsu2, _) = &self.body[j];
                if !self.is_shuntsu(mentsu2) {
                    continue;
                }
                if mentsu1 == mentsu2 {
                    cnt_peko += 1;
                }
            }
        }

        if self.other.contains(&OtherInfo::Menzen) && cnt_peko == 2 {
            self.yaku.remove(&Yaku::Ipeko);
            self.yaku.insert(Yaku::Ryanpeko);
        }
    }

    fn check_junchan(&mut self) {
        let mut junchan = true;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
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
            self.yaku
                .remove(&Yaku::HonChan(self.other.contains(&OtherInfo::Menzen)));
            self.yaku
                .insert(Yaku::Junchan(self.other.contains(&OtherInfo::Menzen)));
        }
    }

    fn check_honitsu(&mut self) {
        let mut check_m = true;
        let mut check_s = true;
        let mut check_p = true;
        for (mentsu, _) in self.body.iter() {
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
            self.yaku
                .insert(Yaku::Honitsu(self.other.contains(&OtherInfo::Menzen)));
        }
    }

    fn check_chinitsu(&mut self) {
        let mut check_m = true;
        let mut check_s = true;
        let mut check_p = true;
        for (mentsu, _) in self.body.iter() {
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
            self.yaku
                .remove(&Yaku::Honitsu(self.other.contains(&OtherInfo::Menzen)));
            self.yaku
                .insert(Yaku::Chinitsu(self.other.contains(&OtherInfo::Menzen)));
        }
    }

    fn check_suanko(&mut self) {
        let mut cnt_anko = 0;
        for (mentsu, fuuro) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
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
            self.yaku.insert(Yaku::SuAnko);
        }
    }

    fn check_daisangen(&mut self) {
        let mut cnt_yuan = 0;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
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
            self.yaku.insert(Yaku::Daisangen);
        }
    }

    fn check_kokushimusou(&mut self) {
        if self.other.contains(&OtherInfo::Kokushimusou) {
            self.yaku.insert(Yaku::KokushiMusou);
        }
    }

    fn check_ryuiisou(&mut self) {
        let mut ryuiisou = true;
        for (mentsu, _) in self.body.iter() {
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

        match self.head {
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
            self.yaku.insert(Yaku::Ryuiisou);
        }
    }

    fn check_tsuiisou(&mut self) {
        let mut tsuiisou = true;
        for (mentsu, _) in self.body.iter() {
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

        match self.head {
            Tile::Y(_) | Tile::W(_) => (),
            _ => {
                tsuiisou = false;
            }
        }

        if tsuiisou {
            self.yaku.insert(Yaku::Tsuiisou);
        }
    }

    fn check_chinroto(&mut self) {
        let mut chinroto = true;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                chinroto = false;
                break;
            }
            chinroto = chinroto && mentsu[0].is_19();
        }

        if chinroto {
            self.yaku.insert(Yaku::Chinroutou);
        }
    }

    fn check_sukantsu(&mut self) {
        let mut cnt_kan = 0;
        for (mentsu, fuuro) in self.body.iter() {
            match fuuro {
                Fuuro::Minkan | Fuuro::Ankan => {
                    cnt_kan += 1;
                }
                _ => (),
            }
        }
        if cnt_kan == 4 {
            self.yaku.insert(Yaku::Suukantsu);
        }
    }

    fn check_syousuushi(&mut self) {
        let mut cnt_wind = 0;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                continue;
            }
            match mentsu[0] {
                Tile::W(_) => {
                    cnt_wind += 1;
                }
                _ => (),
            }
        }
        match self.head {
            Tile::W(_) => {
                cnt_wind += 1;
            }
            _ => (),
        }
        if cnt_wind == 4 {
            self.yaku.insert(Yaku::Syousuushi);
        }
    }

    fn check_churen_poutou(&mut self) {
        if self.other.contains(&OtherInfo::Churenpoutou) {
            self.yaku.insert(Yaku::ChurenPoutou);
        }
    }

    fn check_chihou(&mut self) {
        if self.other.contains(&OtherInfo::Chihou) {
            self.yaku.insert(Yaku::Chihou);
        }
    }

    fn check_tenho(&mut self) {
        if self.other.contains(&OtherInfo::Tenhou) {
            self.yaku.insert(Yaku::Tenhou);
        }
    }

    fn check_suanko_tanki(&mut self) {
        let mut cnt_anko = 0;
        for (mentsu, fuuro) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
                continue;
            }
            match fuuro {
                Fuuro::None | Fuuro::Ankan => {
                    cnt_anko += 1;
                }
                _ => (),
            }
        }
        if cnt_anko == 4 && self.wait == Wait::One {
            self.yaku.remove(&Yaku::SuAnko);
            self.yaku.insert(Yaku::SuankoTanki);
        }
    }

    fn check_kokushimusou_13(&mut self) {
        if self.head == self.last {
            self.yaku.remove(&Yaku::KokushiMusou);
            self.yaku.insert(Yaku::KokushiMusou13);
        }
    }

    fn junsei_churenpoutou(&mut self) {
        if !self.other.contains(&OtherInfo::Churenpoutou) {
            return;
        }
        let mut cnt = vec![0; 9];
        for (mentsu, _) in self.body.iter() {
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
            self.yaku.remove(&Yaku::ChurenPoutou);
            self.yaku.insert(Yaku::JunseiChurenPoutou);
        }
    }

    fn check_daisuushi(&mut self) {
        let mut cnt_wind = 0;
        for (mentsu, _) in self.body.iter() {
            if self.is_shuntsu(mentsu) {
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
            self.yaku.remove(&Yaku::Syousuushi);
            self.yaku.insert(Yaku::Daisuushi);
        }
    }

    fn check_yakuman(&mut self) {
        // 役満

        // ダブル役満
    }

    fn check_1han(&mut self) {
        self.check_reach();
        self.check_ippatsu();
        self.check_tsumo();
        self.check_yakuhai();
        self.check_tanyao();
        self.check_pinfu();
        self.check_ipeko();
        self.check_haitei();
        self.check_houtei();
        self.check_rinshan();
        self.check_chankan();
    }

    fn check_2han(&mut self) {
        self.check_double_reach();
        self.check_sansyoku_dojun();
        self.check_sansyoku_doko();
        self.check_sananko();
        self.check_ikkitsukan();
        self.check_chitoitsu();
        self.check_toitoiho();
        self.check_honchan();
        self.check_sankantsu();
        self.check_shosangen();
        self.check_honroutou();
    }

    fn check_3han(&mut self) {
        self.check_ryanpeko();
        self.check_junchan();
        self.check_honitsu();
    }

    fn check_6han(&mut self) {
        self.check_chinitsu();
    }

    fn upd_yaku(&mut self) {
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

    fn calc_yakuman(&self) -> usize {
        let mut res = 0;
        for yaku in self.yaku.iter() {
            res += yaku.yakuman_cnt();
        }
        res
    }

    fn calc_han(&self) -> usize {
        let mut res = 0;
        for yaku in self.yaku.iter() {
            res += yaku.han();
        }
        res
    }

    fn calc_fu(&self) -> usize {
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

    fn calc_score(&self) -> Vec<i32> {
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

    fn ron(&self, val: i32) -> Vec<i32> {
        let mut res = vec![0; 4];
        res[self.win.unwrap()] = val;
        res
    }

    fn house_tsumo(&self, val: i32) -> Vec<i32> {
        let mut res = vec![val; 4];
        res[self.me] = 0;
        res
    }

    fn child_tsumo(&self, val: (i32, i32)) -> Vec<i32> {
        let mut res = vec![val.0; 4];
        res[self.me] = 0;
        res[self.house] = val.1;
        res
    }

    fn score_val(&self, house: bool, tsumo: bool, han: usize, fu: usize, yakuman: i32) -> Vec<i32> {
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

fn main() {
    let mut hand = Hand::new();
    hand.sort();
    println!("{:?}", hand);
}
