use image::GenericImageView;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("assets/bg1.jpg").unwrap();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    img.save("tmp/test.png").unwrap();

    let thumb = img.thumbnail(200, 200);
    thumb.save("tmp/thumb.jpg").unwrap();

    let font_data = include_bytes!("assets/fonts/DelaGothicOne-Regular.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap();
}
