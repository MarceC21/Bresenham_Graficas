// Main

// Se usa mod para llamar a los otros archivos
mod framebuffer;
mod line;

// Se usan use para llamar a las funciones de los otros archivos
use framebuffer::Framebuffer;
use line::{draw_line_equation, draw_line_dda, draw_line_bresenham};

use raylib::prelude::*;


fn main() {

    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::BLACK);

    framebuffer.set_current_color(Color::WHITE);
    
    //------------------------------------------------------------------------
    // Para la primera imagen para ver la diferencia entre las tres

    // Línea con la Ecuación de la recta
    //framebuffer.set_current_color(Color::WHITE);
    //draw_line_equation( &mut framebuffer, 50, 50, 250, 250,);

    // Línea con DDA
    //Para que sean de diferente color
    //framebuffer.set_current_color(Color::RED);
    //draw_line_dda(&mut framebuffer,300,50,500,250,);

    // Línea 3 - Bresenham
    //framebuffer.set_current_color(Color::GREEN);

    //draw_line_bresenham(&mut framebuffer,550,50,750,250,);

    //------------------------------------------------------------------------

    // Para la segunda imagen para solo lineas con Bresenham

    // Intento de una estrella
    // Centro de la estrella (para refencia)
    let cx = 400;
    let cy = 300;

    draw_line_bresenham(&mut framebuffer, cx, cy, 400, 100);
    draw_line_bresenham(&mut framebuffer, cx, cy, 600, 180);
    draw_line_bresenham(&mut framebuffer, cx, cy, 650, 400);
    draw_line_bresenham(&mut framebuffer, cx, cy, 400, 500);
    draw_line_bresenham(&mut framebuffer, cx, cy, 150, 400);
    draw_line_bresenham(&mut framebuffer, cx, cy, 200, 180);

    framebuffer.render_to_file("estrella.png");

}