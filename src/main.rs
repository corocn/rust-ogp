// use rusttype::{Font, Scale, point};
use image::{Rgba, GenericImageView, GenericImage, DynamicImage};
use imageproc::drawing::{draw_text_mut, Canvas};

// use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
// use fontdue::{Font, FontSettings};

use rusttype::{Font, Scale};

fn main() {
    let mut image = image::open("assets/image/bg1.jpg").unwrap();

    // println!("dimensions {:?}", image.dimensions());
    // println!("{:?}", image.color());

    // image.save("tmp/test.png").unwrap();
    //
    // let thumb = image.thumbnail(200, 200);
    // thumb.save("tmp/thumb.jpg").unwrap();

    // let font = Vec::from(include_bytes!("../assets/fonts/Roboto-Regular.ttf") as &[u8]);
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
