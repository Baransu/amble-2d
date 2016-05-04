
use transform::Transform2D;
use sprite::Sprite;
use camera::{ Camera2D };

pub struct Entity {
    pub transform: Transform2D,
    sprite: Sprite,
}

impl Entity {
    pub fn new(transform: Transform2D, sprite: Sprite) -> Entity {
        Entity {
            transform: transform,
            sprite: sprite,
        }
    }

    pub fn draw(&self, camera: &mut Camera2D) {

        self.sprite.shader.bind();
        self.sprite.shader.set_uniform_matrix4fv("projection", camera.get_projection_marix());
        self.sprite.shader.set_uniform_matrix4fv("model", self.transform.get_model_matrix());
        self.sprite.shader.set_uniform_matrix4fv("view", camera.get_look_at_matrix());

        self.sprite.draw();
    }
}
