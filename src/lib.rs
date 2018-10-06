mod pixel;

use pixel::Pixel;
use std::slice;

#[no_mangle]
pub fn alloc_pixels(size: usize) -> i32 {
    let mut vec = Vec::<Pixel>::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    ptr as i32
}

#[no_mangle]
pub fn paint_red(ptr: i32, width: usize, height: usize, r: u8, g: u8, b: u8, sens: f32, invt: bool) {
    let pixels = unsafe { slice::from_raw_parts_mut(ptr as *mut Pixel, width * height) };

    // pixels.into_par_iter().for_each(|pixel| {
    for pixel in pixels.into_iter() {
        //  比較する色
        let diff = pixel.diff([r, g, b]);

        if !invt {
            if diff < sens {
                pixel.set_alpha(0);
            }
        } else {
            if diff > sens {
                pixel.set_alpha(0);
            }
        }
    }
}
