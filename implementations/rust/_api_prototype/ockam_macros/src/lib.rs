extern crate proc_macro;

use self::proc_macro::{Delimiter, TokenStream, TokenTree};
use std::str::FromStr;

#[proc_macro_attribute]
pub fn node(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = item.into_iter().peekable();

    let mut parse = true;

    let mut func_params = String::new();
    let mut func_body = String::new();

    while parse {
        parse = match input.next() {
            Some(TokenTree::Group(t)) => match t.delimiter() {
                Delimiter::Parenthesis => {
                    func_params = t.stream().to_string();
                    true
                }
                Delimiter::Brace => {
                    func_body = t.stream().to_string();
                    false
                }
                _ => false,
            },
            _ => true,
        }
    }

    let f = &format!(
        "fn main({}) -> () {{
          {}
         }}",
        func_params, func_body
    );
    TokenStream::from_str(f).unwrap()
}
