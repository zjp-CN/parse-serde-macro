#![allow(unused)]
fn main() {}

// field `b` => [Ok([Rename(LitStr { token: "serde rename for b" })])]
// field `c` => [Ok([Skip])]
// field `d` => [Ok([Rename(LitStr { token: "rust_xlsxwriter rename for d" }), NumFormat(LitStr { token: "$0.00" })])]
// field `e` => [Ok([Rename(LitStr { token: "xlsxwriter rename for d" }), NumFormat(LitStr { token: "$0.00" })])]
// field `f` => [Ok([Skip])]
#[derive(_impl::ExcelSerialize, serde::Serialize)]
pub struct A {
    #[serde(rename = "serde rename for b")]
    b: (),
    #[serde(skip)]
    c: (),

    #[rust_xlsxwriter(rename = "rust_xlsxwriter rename for d", num_format = "$0.00")]
    d: (),

    #[xlsxwriter(rename = "xlsxwriter rename for d", num_format = "$0.00")]
    e: (),

    #[xlsxwriter(skip)]
    f: (),
}
