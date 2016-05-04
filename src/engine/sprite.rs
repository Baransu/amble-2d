extern crate opengl as gl;

use std::mem;
use std::ptr;

use self::gl::types::*;
use texture:: { Texture };
use shader:: { Shader };

pub struct Sprite {
    pub texture: Texture,
    pub shader: Shader,
    vbo: u32,
    vao: u32,
}

impl Drop for Sprite {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        };
    }
}

impl Sprite {
    pub fn new(texture_path: &str, shader: Shader) -> Sprite {

        let texture = Texture::new(texture_path);
        let width = texture.width as f32;
        let height = texture.height as f32;
        let data: [f32; 24] = [
            -width/2.0,  height/2.0,  0.0, 1.0,
            -width/2.0, -height/2.0,  0.0, 0.0,
             width/2.0, -height/2.0,  1.0, 0.0,

            -width/2.0,  height/2.0,  0.0, 1.0,
             width/2.0, -height/2.0,  1.0, 0.0,
             width/2.0,  height/2.0,  1.0, 1.0
        ];

        println!("sprite {:?} {:?}x{:?}", texture_path, width, height);

        let mut vbo = 0;
        let mut vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&data[0]), gl::STATIC_DRAW);

            // pos
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 4 * mem::size_of::<f32>() as i32, ptr::null());
            // uvs
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, 4 * mem::size_of::<f32>() as i32, mem::transmute(2 * mem::size_of::<f32>()));

            gl::BindVertexArray(0);
        }

        Sprite {
            texture: texture,
            shader: shader,
            vbo: vbo,
            vao: vao,
        }
    }

    pub fn draw(&self) {
        unsafe {

            self.texture.bind(gl::TEXTURE0);
            self.shader.set_uniform_1i("texture0", 0);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

}
