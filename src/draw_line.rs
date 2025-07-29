use std::io;

pub fn draw_line(from: (u16, u16), to: (u16, u16)) -> io::Result<Vec<(u16, u16)>> {
    if from.0 == to.0 && from.1 == to.1 {
        return Ok(vec![from]);
    }

    let mut pixels: Vec<(u16, u16)> = Vec::new();

    let x0 = from.0 as i32;
    let y0 = from.1 as i32;
    let x1 = to.0 as i32;
    let y1 = to.1 as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        pixels.push((x as u16, y as u16));

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

    Ok(pixels)
}
