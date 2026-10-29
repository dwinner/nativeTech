use wasm_bindgen::prelude::*;
use ::image::{imageops, ImageBuffer, Rgba};

#[wasm_bindgen]
pub fn wasm_resize(
    data: &[u8],
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32
) -> Vec<u8> {
    let src_img: ImageBuffer<Rgba<u8>, &[u8]> =
        ImageBuffer::from_raw(src_width, src_height, data)
            .expect("Неверные размеры исходного изображения");

    let filter = imageops::FilterType::Lanczos3;

    let resized = imageops::resize(
        &src_img,
        dst_width,
        dst_height,
        filter,
    );

    resized.into_raw()
}

#[wasm_bindgen]
pub fn wasm_gaussian_blur(
    data: &[u8],
    width: u32,
    height: u32,
    sigma: f32,
) -> Vec<u8> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, data.to_vec())
            .expect("Invalid image dimensions");

    let blurred = imageops::blur(&img, sigma);

    blurred.into_raw()
}

#[wasm_bindgen]
pub fn wasm_gaussian_blur2(
    data: Vec<u8>,
    width: usize,
    height: usize,
    sigma: f32,
) -> Vec<u8> {
    // Canvas отдает плоский массив RGBA элементов, а fastblur::gaussian_blur ожидает Vec<[u8; 3]>,
    // поэтому вариант с mem::transmute не сработал.
    // В итоге делаем честно новый вектор без альфа канала.
    let mut data: Vec<[u8; 3]> = data.chunks(4).fold(vec![], |mut acc, chunk| {
        acc.push([chunk[0], chunk[1], chunk[2]]);
        acc
    });

    fastblur::gaussian_blur(&mut data, width, height, sigma);

    // Преобразовываем обратно в плоский массив RGBA элементов
    let data: Vec<u8> = data.iter().fold(vec![], |mut acc, el| {
        acc.extend_from_slice(el);
        acc.push(255);
        acc
    });

    data
}
