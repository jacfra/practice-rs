mod parse_attr;

use parse_attr::parse_attr_core;

#[proc_macro_attribute]
pub fn parse_attr(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_attr_core(attr.into(), input.into()).into()
}
