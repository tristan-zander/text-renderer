use rusttype::{Font, gpu_cache::{CacheBuilder, Cache}};

use glium::{
    texture::{ RawImage2d, Texture2d},
};


struct TextRenderer<'a> {
    font: Font<'a>,
    cache: Cache<'a>,
    tex: Texture2d
}

impl <'a> TextRenderer<'a> {

}