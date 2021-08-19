use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

fn main() {
    let mut image = image::open("assets/image/bg1.jpg").unwrap();
    let font = Vec::from(include_bytes!("../assets/fonts/DelaGothicOne-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 300.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    let text = "すごい副業";
    draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), 50, 0, scale, &font, text);

    image.save("tmp/test.jpg").unwrap();
}
