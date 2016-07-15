extern crate stb_truetype;

use stb_truetype::FontInfo;
use std::borrow::Cow;
fn main() {
    let file = &include_bytes!("Gudea-Regular.ttf")[..];
    let font = FontInfo::new(Cow::Borrowed(file), 0).unwrap();

    for info in font.get_font_name_strings() {
        let (name, pl_en, la, na) = info;
        let name8 = ::std::str::from_utf8(name);
        println!("{}, {:?}, {:?}, {:?}", name8.ok().unwrap_or("(not UTF-8)"), pl_en, la, na);
    }
}
