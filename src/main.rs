use rusttype::{Font, Scale, point};
use image::{Rgba, GenericImageView, GenericImage};

fn main() {
    let mut img = image::open("assets/image/bg1.jpg").unwrap();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    // img.save("tmp/test.png").unwrap();
    //
    // let thumb = img.thumbnail(200, 200);
    // thumb.save("tmp/thumb.jpg").unwrap();

    let font_data = include_bytes!("../assets/fonts/DelaGothicOne-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let text = "こんにちは";
    let color = (150, 0, 0); // dark red
    let start = point(20.0, 50.0);
    let size = 32.0;
    let scale = Scale {x: size, y: size};

    for glyph in font.layout(text, scale, start) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            // glyph.draw(|x, y, v|
            //     image.put_pixel(
            //     // Offset the position by the glyph bounding box
            //     x + bounding_box.min.x as u32,
            //     y + bounding_box.min.y as u32,
            //     // Turn the coverage into an alpha value
            //     Rgba {data: [color.0, color.1, color.2, (v * 255.0) as u8]}
            // )
            // );

            glyph.draw(|x, y, v|
                // println!("{}", x)
                img.put_pixel(
                x + bounding_box.min.x as u32,
                y + bounding_box.min.y as u32,
               Rgba { 0: [color.0, color.1, color.2, 255] }
                )
            );
        }
    }

    img.save("tmp/test.jpg").unwrap();
}
