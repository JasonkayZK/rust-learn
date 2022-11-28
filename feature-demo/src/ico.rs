use crate::bmp::process_bmp;
use crate::png::process_png;

#[cfg(all(feature = "png", feature = "bmp"))]
#[cfg(feature = "ico")]
pub fn process_ico() {
    println!("Before process_ico: ");
    process_bmp();
    process_png();
    println!("Processing ico")
}
