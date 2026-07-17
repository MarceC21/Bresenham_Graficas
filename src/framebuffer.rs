// Para crear un framebuffer y dibujar en él

use raylib::prelude::*;

// Framebuffer
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub image: Image,

    background_color: Color,
    current_color: Color,
}


// Implementación de Framebuffer
impl Framebuffer {

    // Crear framebuffer
    pub fn new(width: i32, height: i32) -> Self {

        Self { width, height, image: Image::gen_image_color(width, height, Color::BLACK),
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    // Cosas extras para cambiar el color de fondo y el color actual
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }


    pub fn clear(&mut self) {

        self.image = Image::gen_image_color(
            self.width,
            self.height,
            self.background_color,
        );

    }

    // Dibujar un punto usando el color actual
    pub fn point(&mut self, x: i32, y: i32) {

        if x >= 0 &&
           x < self.width &&
           y >= 0 &&
           y < self.height {

            self.image.draw_pixel(x, y, self.current_color);

        }

    }

    // Guardar la imagen
    pub fn render_to_file(&self, filename: &str) {

        self.image.export_image(filename);

    }

}