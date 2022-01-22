use clap::{ArgEnum, Parser};
use strum::Display;

/// Google Knowledge Graf API CLI Tool
#[derive(Parser)]
pub struct Args {
    /// 検索するクエリ文字列
    pub query: String,

    /// 最大表示件数
    #[clap(long, short = 'n', default_value_t = 5)]
    pub limit: u32,

    /// 検索に使用する言語
    #[clap(long, short, arg_enum, default_value_t = Lang::Ja)]
    pub lang: Lang,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

#[derive(Copy, Clone, ArgEnum, Display)]
pub enum Lang {
    #[strum(serialize = "ja")]
    Ja,
    #[strum(serialize = "en")]
    En,
}
