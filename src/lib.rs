pub mod prelude {
    pub use crate::{print_tooltips, TipLine, TipTextLine, TipUrlLine};
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum TipLine {
    Text(TipTextLine),
    Url(TipUrlLine),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TipTextLine {
    pub value: String,
}

impl Into<TipLine> for TipTextLine {
    fn into(self) -> TipLine {
        TipLine::Text(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TipUrlLine {
    pub label: String,
    pub value: String,
}

impl Into<TipLine> for TipUrlLine {
    fn into(self) -> TipLine {
        TipLine::Url(self)
    }
}

pub fn print_tooltips(tips: &[TipLine]) {
    let output = serde_json::to_string(&tips).unwrap();
    print!("{}", output);
}
