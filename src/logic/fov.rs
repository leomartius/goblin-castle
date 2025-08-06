/// line-of-sight FOV algorithm
pub fn compute_fov<F>(
    visible: &mut [bool],
    width: i32,
    height: i32,
    is_transparent: F,
    player_x: i32,
    player_y: i32,
) where
    F: Fn(i32, i32) -> bool,
{
    let radius = 8;

    visible.fill(false);

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            // skip if outside the circle radius
            if dx * dx + dy * dy > radius * radius {
                continue;
            }

            let i = player_x + dx;
            let j = player_y + dy;
            if i < 0 || i >= width || j < 0 || j >= height {
                continue;
            }

            for (x, y) in line(player_x, player_y, i, j) {
                visible[y as usize * width as usize + x as usize] = true;
                if !is_transparent(x, y) {
                    break;
                }
            }
        }
    }
}

/// Bresenham's line algorithm
fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    points
}
