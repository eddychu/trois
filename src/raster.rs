use crate::framebuffer::FrameBuffer;
use crate::vector2::Vector2;
use crate::vector3::Vector3;
pub fn draw_line(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: Vector3,
    framebuffer: &mut FrameBuffer,
) {
    let ipart = |x: f32| x.floor() as u32;
    let round = |x: f32| x.round() as u32;
    let fpart = |x: f32| x - ipart(x) as f32;
    let rfpart = |x: f32| 1.0 - fpart(x);
    let diff_y = (y1 - y0).abs();
    let diff_x = (x1 - x0).abs();
    let steep = diff_y > diff_x;
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    if steep {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
    }
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut gradient = 0.0f32;
    if dx == 0.0 {
        gradient = 1.0;
    } else {
        gradient = dy / dx;
    }
    let mut x_end = round(x0);
    let mut y_end = y0 + gradient * (x_end as f32 - x0);
    let mut x_gap = rfpart(x0 + 0.5);
    let x_pxl1 = x_end;
    let y_pxl1 = ipart(y_end);
    if steep {
        let cur_color = color * x_gap * rfpart(y_end);
        framebuffer.set_color(y_pxl1, x_pxl1, cur_color.to_u32());
        let cur_color = color * x_gap * fpart(y_end);
        framebuffer.set_color(y_pxl1 + 1, x_pxl1, cur_color.to_u32());
    } else {
        let cur_color = color * x_gap * rfpart(y_end);
        framebuffer.set_color(x_pxl1, y_pxl1, cur_color.to_u32());
        let cur_color = color * x_gap * fpart(y_end);
        framebuffer.set_color(x_pxl1, y_pxl1 + 1, cur_color.to_u32());
    }

    let mut intery = y_end + gradient;
    x_end = round(x1);
    y_end = y1 + gradient * (x_end as f32 - x1);
    x_gap = fpart(x1 + 0.5);
    let x_pxl2 = x_end;
    let y_pxl2 = ipart(y_end);
    if steep {
        let cur_color = color * x_gap * rfpart(y_end);
        framebuffer.set_color(y_pxl2, x_pxl2, cur_color.to_u32());
        let cur_color = color * x_gap * fpart(y_end);
        framebuffer.set_color(y_pxl2 + 1, x_pxl2, cur_color.to_u32());
    } else {
        let cur_color = color * x_gap * rfpart(y_end);
        framebuffer.set_color(x_pxl2, y_pxl2, cur_color.to_u32());
        let cur_color = color * x_gap * fpart(y_end);
        framebuffer.set_color(x_pxl2, y_pxl2 + 1, cur_color.to_u32());
    }

    if steep {
        for x in (x_pxl1 + 1)..x_pxl2 {
            let cur_color = color * rfpart(intery);
            framebuffer.set_color(ipart(intery), x, cur_color.to_u32());
            let cur_color = color * fpart(intery);
            framebuffer.set_color(ipart(intery) + 1, x, cur_color.to_u32());

            intery += gradient;
        }
    } else {
        for x in (x_pxl1 + 1)..x_pxl2 {
            let cur_color = color * rfpart(intery);
            framebuffer.set_color(x, ipart(intery), cur_color.to_u32());
            let cur_color = color * fpart(intery);
            framebuffer.set_color(x, ipart(intery) + 1, cur_color.to_u32());
            intery += gradient;
        }
    }
}

// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
pub fn draw_filled_triangle(vertices: Vec<Vector2>, color: Vector3, framebuffer: &mut FrameBuffer) {
    
}
