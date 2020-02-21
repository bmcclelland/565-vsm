use rusttype::{
    Font,
    Scale,
    VMetrics,
    PositionedGlyph,
    point,
};
use image::{
    Rgba,
    RgbaImage,
};
use glium::{
    texture::RawImage2d,
    texture::CompressedSrgbTexture2d,
    Display,
};
use super::font_placer::Placer;
use std::collections::HashMap;
use nalgebra_glm::Vec2;

pub struct GlyphString<'a> {
    pub glyphs: Vec<PositionedGlyph<'a>>,
    pub dims: (u32, u32),
}

pub struct UnfinishedTexMap {
    font: Font<'static>,
    scale: Scale,
    h_pad: u32,
    v_metrics: VMetrics, 
    strings: HashMap<&'static str, GlyphString<'static>>,
    dims: (u32, u32), 
}

impl UnfinishedTexMap {
    pub fn new(dims: (u32, u32), scale: u32, h_pad: u32) -> Self {
        let scale = Scale::uniform(scale as f32);
        let font_data = include_bytes!("../../data/LiberationSans-Bold.ttf");
        let font = Font::from_bytes(font_data as &[u8])
            .expect("TexMap: error constructing font");
        let v_metrics = font.v_metrics(scale);

        Self {
            font,
            scale,
            h_pad,
            v_metrics,
            strings: HashMap::new(),
            dims,
        }
    }

    pub fn add_string(mut self, text: &'static str) -> Self {
        let glyphs: Vec<_> = self.font
            .layout(text, self.scale, point(self.h_pad as f32, self.v_metrics.ascent))
            .collect();
        let height = (self.v_metrics.ascent - self.v_metrics.descent).ceil() as u32;
        let width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };
        let dims = (self.h_pad * 2 + width, height);
        self.strings.insert(text, GlyphString { glyphs, dims });
        self
    }

    fn make_image(&self) -> (RgbaImage, HashMap<&'static str, TexMeta>) {
        let black = Rgba([0,0,0,255]);
        let mut image = RgbaImage::new(self.dims.0, self.dims.1);
        for (_, _, p) in image.enumerate_pixels_mut() {
            *p = black;
        }

        let mut unused: Vec<(&'static str, u32, u32)> = Vec::new();
        for (t,g) in self.strings.iter() {
            unused.push( (t,g.dims.0,g.dims.1) );
        }

        let mut placer = Placer::new(self.dims, unused);
        placer.run();
    
        let mut image = RgbaImage::new(self.dims.0, self.dims.1);
        for (_, _, p) in image.enumerate_pixels_mut() {
            *p = Rgba([0,0,0,255]);
        }

        let mut map = HashMap::new();

        let color = (1.0, 1.0, 0.0);

        for (id, x, y, w, h) in placer.results() {
            let s = &self.strings[id];
            for g in s.glyphs.iter() {
                if let Some(bounding_box) = g.pixel_bounding_box() {
                    g.draw(|gx,gy,a| {
                        let r = (color.0 * a * 255.0) as u8;
                        let g = (color.1 * a * 255.0) as u8;
                        let b = (color.2 * a * 255.0) as u8;
                        let a = 255;
                        image.put_pixel(
                            x + gx + bounding_box.min.x as u32,
                            y + gy + bounding_box.min.y as u32,
                            Rgba([r,g,b,a]),
                        )
                    });
                }
            }

            map.insert(id, TexMeta { x, y, w, h });
        }

        (image, map)
    }
    
    pub fn finish(self, display: &Display) -> TexMap {
        let (image, map) = self.make_image();
        let image_dims = image.dimensions();
        let raw_image = RawImage2d::from_raw_rgba(image.into_raw(), image_dims);
        let gl_image = CompressedSrgbTexture2d::new(display, raw_image).unwrap();

        TexMap {
            gl_image,
//            font: self.font,
//            scale: self.scale,
            dims: self.dims,
            map,
        }
    }
}

pub struct TexMeta {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct TexMap {
//    font: Font<'static>,
//    scale: Scale,
    dims: (u32, u32), 
    map: HashMap<&'static str, TexMeta>,
    gl_image: CompressedSrgbTexture2d,
}

impl TexMap {
    pub fn dims(&self) -> (u32, u32) {
        self.dims
    }

    pub fn gl_image(&self) -> &CompressedSrgbTexture2d {
        &self.gl_image
    }

    pub fn get(&self, key: &'static str) -> Option<&TexMeta> {
        self.map.get(key)
    }

    pub fn get_attrs(&self, key: &'static str) -> Option<(Vec2, Vec2)> {
        let t = self.get(key)?;
       
        let tx = t.x as f32;
        let ty = t.y as f32;
        let tw = t.w as f32;
        let th = t.h as f32;
        let w = self.dims.0 as f32;
        let h = self.dims.1 as f32;

        let tpos   = Vec2::new(tx / w, 1.0 - ty / h - th / h);
        let tscale = Vec2::new(tw / w, th / h);
        Some( (tpos, tscale) )
    }
}
