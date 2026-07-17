// Aqui son las funciones para dibujar las lineas usando la ecuación de la recta, DDA y BRESENHAM

use crate::framebuffer::Framebuffer;

/// Con la ecuación de la recta 
// y= mx + b
pub fn draw_line_equation(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32,) {

    let m = (y1 - y0) as f64 / (x1 - x0) as f64;
    let b = y0 as f64 - m * x0 as f64;

    let x_start = x0.min(x1);
    let x_end = x0.max(x1);

    for x in x_start..=x_end {

        let y = (m * x as f64 + b).round() as i32;

        fb.point(x, y);

    }

}

// DDA 
pub fn draw_line_dda(fb: &mut Framebuffer,x0: i32,y0: i32,x1: i32,y1: i32,) {

    let dx = x1 - x0;
    let dy = y1 - y0;

    let steps = dx.abs().max(dy.abs());

    let x_inc = dx as f64 / steps as f64;
    let y_inc = dy as f64 / steps as f64;

    let mut x = x0 as f64;
    let mut y = y0 as f64;

    for _ in 0..=steps {

        fb.point(
            x.round() as i32,
            y.round() as i32,
        );

        x += x_inc;
        y += y_inc;

    }

}


// BRESENHAM (TODOS LOS OCTANTES)
pub fn draw_line_bresenham(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32,) {

    let mut x = x0;
    let mut y = y0;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    loop {

        fb.point(x, y);

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {

            err -= dy;
            x += sx;

        }

        if e2 < dx {

            err += dx;
            y += sy;

        }

    }

}
