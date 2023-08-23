use image::DynamicImage;

pub struct Render {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Render {
    pub fn image(&mut self, img: &DynamicImage) {
        let img_buf = img.to_rgba8();
        for (x, y, pixel) in img_buf.enumerate_pixels() {
            let rgba = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
            // バウンドチェックを行い、バッファの範囲内にある場合のみ画像をコピーします
            if x < self.width as u32 && y < self.height as u32 {
                if let Some(pixel_to_update) = self.buffer.get_mut((y as usize) * self.width + x as usize) {
                    *pixel_to_update = rgba;
                }
            }
        }
    }

    pub fn image_at(&mut self, img: &DynamicImage, x: f32, y: f32) {
        let img_buf = img.to_rgba8();
    
        // 画像のサイズを取得
        let img_width = img.width() as usize;
        let img_height = img.height() as usize;
    
        for (img_x, img_y, pixel) in img_buf.enumerate_pixels() {
            // ワールド座標系でのピクセルの位置を計算
            let world_x = x as usize + img_x as usize;
            let world_y = y as usize + img_y as usize;
    
            let rgba = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
    
            // バウンドチェックを行い、バッファの範囲内にある場合のみ画像をコピーします
            if world_x < self.width && world_y < self.height {
                if let Some(pixel_to_update) = self.buffer.get_mut(world_y * self.width + world_x) {
                    *pixel_to_update = rgba;
                }
            }
        }
    }
    
    pub fn color(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }
}
