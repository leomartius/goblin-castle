pub fn compute_fov(visible: &mut [bool], width: i32, height: i32, player_x: i32, player_y: i32) {
    visible.fill(false);

    for i in player_x - 2..=player_x + 2 {
        for j in player_y - 2..=player_y + 2 {
            if i >= 0 && i < width && j >= 0 && j < height {
                visible[j as usize * width as usize + i as usize] = true;
            }
        }
    }
}
