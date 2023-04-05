use prac_macro_derive::parse_attr;

#[parse_attr("hello", "world")]
fn ahh() {}

fn main() {
    ahh();
}
