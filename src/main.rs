extern crate image;
use image::{open};
mod blur;
use blur::gaussian::gaussian_blur;

fn main() {
    let img = open("./t.png").unwrap();
    gaussian_blur(img, 5);
}
