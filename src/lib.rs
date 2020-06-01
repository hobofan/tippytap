pub mod prelude {
    pub use crate::{print_tooltips, TipExecuteLine, TipInput, TipLine, TipTextLine, TipUrlLine};
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum TipLine {
    Text(TipTextLine),
    Url(TipUrlLine),
    Execute(TipExecuteLine),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TipExecuteLine {
    pub label: String,
    pub args: Vec<String>,
}

impl Into<TipLine> for TipExecuteLine {
    fn into(self) -> TipLine {
        TipLine::Execute(self)
    }
}

#[derive(Debug, Clone)]
pub struct TipInput {
    pub args: Vec<String>,
    pub bundle_identifier: Option<String>,
}

impl TipInput {
    pub fn from_args() -> Self {
        let mut main_args = Vec::new();
        let mut bundle_identifier = None;

        let mut previous_arg_was_bundleidentifier = false;
        let mut args = std::env::args();
        args.next().unwrap();
        for arg in args {
            if arg == "--bundle-identifier" {
                previous_arg_was_bundleidentifier = true;
                continue;
            }
            if previous_arg_was_bundleidentifier {
                previous_arg_was_bundleidentifier = false;
                bundle_identifier = Some(arg);
                continue;
            }

            main_args.push(arg);
        }

        TipInput {
            args: main_args,
            bundle_identifier,
        }
    }
}

pub fn print_tooltips(tips: &[TipLine]) {
    let output = serde_json::to_string(&tips).unwrap();
    print!("{}", output);
}
