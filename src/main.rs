extern crate glutin;
extern crate time;
extern crate image;
extern crate rand;

// local
extern crate math;
extern crate engine;
extern crate opengl as gl;

use gl::types::*;

use std::mem;
use std::ptr;
// use std::str;
// use std::cmp;

use glutin::*;

use rand::Rng;

// use cgmath::*;

// use std::ffi::CString;
//
// use std::fs::File;


// local
use engine::shader::Shader;
use engine::texture::Texture;

use math::mat4::Mat4;
use math::vec4::Vec4;
use math::vec3::Vec3;
use math::vec2::Vec2;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

static QUAD_VERTICES: [f32; 24] = [
    // Positions   // TexCoords
    -1.0,  1.0,  0.0, 1.0,
    -1.0, -1.0,  0.0, 0.0,
     1.0, -1.0,  1.0, 0.0,

    -1.0,  1.0,  0.0, 1.0,
     1.0, -1.0,  1.0, 0.0,
     1.0,  1.0,  1.0, 1.0
];

fn main() {

    let window = WindowBuilder::new()
        .with_title("rust-3d".to_string())
        // .with_fullscreen(get_primary_monitor())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        // .with_gl(GlRequest::Specific(Api::OpenGl, (3 as u8, 3 as u8)))
        // .with_multisampling(16)
        .with_vsync()
        .build()
        .unwrap();

    // Make the context current before calling gl::load_with
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| {
        window.get_proc_address(symbol) as *const _
    });

    // input stuff
    let mut pressed_keys: [bool; 1024] = [false; 1024];

    let texture = Texture::new("res/rust_logo.png", 4.0);
    let shader = Shader::new("res/simpleShader.vert", "res/simpleShader.frag");

    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        // gl::Enable(gl::CULL_FACE);
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::FRONT_AND_BACK);
        gl::Enable(gl::DEPTH_TEST);

        // create vao and vbo form simple quad
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (QUAD_VERTICES.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&QUAD_VERTICES[0]), gl::STATIC_DRAW);

        // pos
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 4 * mem::size_of::<f32>() as i32, ptr::null());
        // uvs
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, 4 * mem::size_of::<f32>() as i32, mem::transmute(2 * mem::size_of::<f32>()));

        gl::BindVertexArray(0);

    }

    let mut time = 0.0;
    'running: loop {

        time += 0.16;
        let ts = time::get_time();
        // println!("{:?}", ts.sec as f64);
        let angle: f64 = ts.sec as f64 + ts.nsec as f64/1000000000.0;
        // println!("{:?}", time);

        // opengl stuff
        unsafe {

            gl::ClearColor(44.0/255.0, 44.0/255.0, 44.0/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection = Mat4::from_ortho(-WIDTH/2.0, WIDTH/2.0, -HEIGHT/2.0, HEIGHT/2.0, 0.1, 100.0);
            let model = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));
            // let model = Mat4::new(1.0);
            println!("{:?}", model);

            shader.bind();
            shader.set_uniform_matrix4fv("projection", projection);
            shader.set_uniform_matrix4fv("model", model);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);

        }

        window.swap_buffers().unwrap();

        for event in window.poll_events() {
            match event {
                Event::Closed => break'running,
                Event::KeyboardInput(ElementState::Pressed, _, Some(x)) => {
                    pressed_keys[x as usize] = true;
                },
                Event::KeyboardInput(ElementState::Released, _, Some(x)) => {
                    pressed_keys[x as usize] = false;
                },
                Event::MouseMoved((x, y)) => { },
                _ => (),
            }
        }
    }

    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }

}
