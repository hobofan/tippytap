## Tippytap

Tippytap is a Rust library to help you write tooltips for [Tip](https://github.com/tanin47/tip), the "programmable tooltip that can be used with any Mac OS app".

## Installation

Via [cargo-edit](https://github.com/killercup/cargo-edit):

```
cargo add tippytap
```

## Example program

`main.rs`

```rust
use tippytap::prelude::*;

pub fn sci_hub_tooltip(input: &TipInput) -> TipUrlLine {
    TipUrlLine {
        label: "SciHub".to_owned(),
        value: format!("https://sci-hub.tw/{}", input.args[0]),
    }
}

fn main() {
    let input = TipInput::from_args();

    let output = vec![
        TipTextLine {
            value: format!("Input {}", input),
        }
        .into(),
        sci_hub_tooltip(&input).into(),
    ];
    print_tooltips(&output);
}
```
