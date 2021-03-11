use glium::{
    backend::Facade,
    texture::{MipmapsOption, RawImage2d, Texture2d, UncompressedFloatFormat, ClientFormat},
};
use rusttype::{
    gpu_cache::{Cache, CacheBuilder},
    Font,
};
use std::borrow::Cow;

struct TextRenderer<'a> {
    font: Font<'a>,
    cache: Cache<'a>,
    tex: Texture2d,
}

impl<'a> TextRenderer<'a> {
    fn new<F: Facade>(display: F) -> Self {
        const SIZE: u32 = 4 * 1024;

        let font = Font::try_from_bytes(include_bytes!("../res/LinLibertine_R.ttf")).unwrap();
        let cache = CacheBuilder::default()
            .dimensions(SIZE, SIZE)
            .multithread(true)
            .build();
        let tex = Texture2d::with_format(
            &display,
            RawImage2d {
                data: Cow::Owned(vec![128u8; (SIZE * SIZE) as usize]),
                width: SIZE,
                height: SIZE,
                format: ClientFormat::U8,
            },
            UncompressedFloatFormat::U8,
            MipmapsOption::NoMipmap,
        )
        .unwrap();

        TextRenderer { font, cache, tex }
    }
}
