#[derive(PartialEq, Eq, Hash)]
pub enum Yaku {
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
    pub fn han(&self) -> usize {
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

    pub fn yakuman_cnt(&self) -> usize {
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
