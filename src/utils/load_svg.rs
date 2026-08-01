use gpui::*;
use image::{Frame, RgbaImage};
use resvg::{
    tiny_skia,
    usvg::{self, Tree},
};
use std::{fs, path::Path, sync::Arc};

pub fn load_svg(path: &Path, target_pixels: u32) -> Option<Arc<RenderImage>> {
    let svg_data = fs::read(path).ok()?;
    let opt = usvg::Options::default();
    let tree = Tree::from_data(&svg_data, &opt).ok()?;

    let mut pixmap = tiny_skia::Pixmap::new(target_pixels, target_pixels)?;

    let bbox = tree.size();
    let scale_x = target_pixels as f32 / bbox.width();
    let scale_y = target_pixels as f32 / bbox.height();
    let scale = scale_x.min(scale_y);

    let transform = tiny_skia::Transform::from_scale(scale, scale);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let mut bgra_bytes = Vec::with_capacity(pixmap.data().len());
    for pixel in pixmap.pixels() {
        let c = pixel.demultiply();

        bgra_bytes.push(c.blue());
        bgra_bytes.push(c.green());
        bgra_bytes.push(c.red());
        bgra_bytes.push(c.alpha());
    }

    let bgra_image = RgbaImage::from_raw(target_pixels, target_pixels, bgra_bytes)?;
    let frame = Frame::new(bgra_image);

    Some(Arc::new(RenderImage::new(vec![frame])))
}
