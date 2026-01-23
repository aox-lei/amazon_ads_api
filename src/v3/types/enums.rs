use serde::{Serialize,Deserialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Serialize,Deserialize, Debug, AsRefStr, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum MatchType {
    Broad,
    Exact,
    Phrase,
}

#[derive(Serialize, Debug, AsRefStr, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum Locale {
    ArEg, // ar_EG
    DeDe, // de_DE
    EnAe, // en_AE
    EnAu, // en_AU
    EnCa, // en_CA
    EnGb, // en_GB
    EnIn, // en_IN
    EnSa, // en_SA
    EnSg, // en_SG
    EnUs, // en_US
    EsEs, // es_ES
    EsMx, // es_MX
    FrFr, // fr_FR
    ItIt, // it_IT
    JaJp, // ja_JP
    NlNl, // nl_NL
    PlPl, // pl_PL
    PtBr, // pt_BR
    SvSe, // sv_SE
    TrTr, // tr_TR
    ZhCn, // zh_CN
}

#[derive(Serialize, Debug, AsRefStr, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum SortDimension {
    Clicks,
    Conversions,
    Default,
}
