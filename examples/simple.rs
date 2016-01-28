extern crate stb_truetype;

use stb_truetype::FontInfo;

fn main() {
    let file = include_bytes!("Gudea-Regular.ttf");
    let font = FontInfo::new(file, 0).unwrap();
    let vmetrics = font.get_v_metrics();
    println!("{:?}", vmetrics);
    let c = 'é';
    let cp = c as u32;
    let g = font.find_glyph_index(cp);
    println!("{:?} -> {:?}", cp, g);
    let r = font.get_glyph_box(g);
    println!("{:?}", r);
    let shape = font.get_glyph_shape(g);
    println!("{:#?}", shape);
    let hmetrics = font.get_glyph_h_metrics(g);
    println!("{:?}", hmetrics);
    let advance = font.get_codepoint_kern_advance('f' as u32, 'f' as u32);
    println!("{:?}", advance);
    let scale = font.scale_for_pixel_height(20.0);
    println!("{:?}", scale);
}
