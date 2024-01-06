#![allow(unused)]
fn main() {}

// field `b` => [Rename(LitStr { token: "serde rename for b" })]
// field `c` => [Skip]
// field `d` => [Rename(LitStr { token: "rust_xlsxwriter rename for d" }), NumFormat(LitStr { token: "$0.00" })]
// field `e` => [Rename(LitStr { token: "xlsxwriter rename for e" }), NumFormat(LitStr { token: "$0.00" })]
// field `f` => [Skip]
// field `g` => [FormatObj(Expr::MethodCall { ... })]
#[derive(_impl::ExcelSerialize, serde::Serialize)]
pub struct A {
    #[serde(rename = "serde rename for b")]
    b: (),
    #[serde(skip)]
    c: (),

    #[rust_xlsxwriter(rename = "rust_xlsxwriter rename for d", num_format = "$0.00")]
    d: (),

    #[xlsxwriter(rename = "xlsxwriter rename for e", num_format = "$0.00")]
    e: (),
    #[xlsxwriter(skip)]
    f: (),

    #[xlsxwriter(value_format = Format::new().set_bold())]
    g1: (),
}

pub struct Format;
impl Format {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Format {
        Format
    }
    pub fn set_bold(self) -> Format {
        self
    }
}

// error: `not_exist` is not supported by ExcelSerialize derive macro
//   --> src/main.rs:40:18
//    |
// 40 |     #[xlsxwriter(not_exist)]
//    |                  ^^^^^^^^^
// #[derive(_impl::ExcelSerialize, serde::Serialize)]
// pub struct B {
//     #[xlsxwriter(not_exist)]
//     f: (),
// }
