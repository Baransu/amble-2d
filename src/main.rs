extern crate glutin;
extern crate time;
extern crate image;
extern crate rand;

// local
extern crate math;
extern crate engine;
extern crate opengl as gl;

use gl::types::*;

// use std::mem;
// use std::ptr;
// use std::str;
// use std::cmp;

use glutin::*;

// use rand::Rng;

// use cgmath::*;

// use std::ffi::CString;
//
// use std::fs::File;


// local
use engine::shader::Shader;
use engine::sprite::Sprite;
use engine::entity::Entity;
use engine::camera:: { Camera2D };

use engine::transform:: { Transform2D };

use math::vec2::Vec2;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

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

    let mut camera = Camera2D::new(Vec2::new(0.0, 0.0), 0.0, WIDTH, HEIGHT);
    let mut entity = Entity::new(
        Transform2D::new(Vec2::new(0.0, 0.0), 0.0, Vec2::new(1.0, 1.0)),
        Sprite::new("res/rust_logo.png", Shader::new("res/sprite_shader.vert", "res/sprite_shader.frag"))
    );

    unsafe {
        // gl::Enable(gl::CULL_FACE);
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::FRONT_AND_BACK);
        gl::Enable(gl::DEPTH_TEST);

    }

    let mut time = 0.0;
    'running: loop {

        time += 0.16;
        let ts = time::get_time();
        let angle: f64 = ts.sec as f64 + ts.nsec as f64/1000000000.0;

        // opengl stuff
        unsafe {

            gl::ClearColor(44.0/255.0, 44.0/255.0, 44.0/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            entity.transform.rotation = time;

            entity.draw(&mut camera);

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
                // Event::MouseMoved((x, y)) => { },
                _ => (),
            }
        }
    }
}
