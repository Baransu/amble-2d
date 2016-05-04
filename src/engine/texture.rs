extern crate opengl as gl;
extern crate image;

// use std::str;
use std::mem;
// use std::ptr;

// use self::gl::types::*;

#[derive(Clone)]
pub struct Texture {
    texture_id: u32,
    pub width: u32,
    pub height: u32,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &mut self.texture_id) } ;
    }
}

impl Texture {
    pub fn new(texture_path: &str) -> Texture {

        let mut texture_id = 0;

        let mut width: u32 = 0;
        let mut height: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // texture wrapping
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // texture filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let texture_data = image::open(texture_path).expect("Opening image for texture failed");
            let texture_data = texture_data.to_rgba();

            height = texture_data.width();
            width = texture_data.height();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                height as i32,
                width as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                mem::transmute(&texture_data.into_raw()[0])
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);

        }

        Texture {
            texture_id: texture_id,
            width: width,
            height: height
        }
    }

    pub fn bind(&self, location: u32) {
        unsafe {
            gl::ActiveTexture(location);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}
