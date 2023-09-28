use crate::engine::core::entity::player::Player;


pub fn update_camera_follow_player(
    player: &mut Player,
    _frame: &mut glium::Frame,
    player_position: [f32; 2],
) {
    // Get the current window size
    let (window_width, window_height) = player.display.get_framebuffer_dimensions();

    // Calculate the aspect ratio
    let aspect_ratio = window_width as f32 / window_height as f32;

    // Calculate the desired square size in window coordinates
    let desired_square_size = player.sprite_size / window_width as f32;

    // Calculate the position to center the camera on the player
    let camera_position = na::Point2::new(
        player_position[0] - (window_width as f32 * 0.5 * desired_square_size),
        player_position[1] - (window_height as f32 * 0.5 * desired_square_size / aspect_ratio),
    );

    // Create a camera transformation matrix to center on the player
    let _camera_matrix = na::Matrix4::new_translation(&na::Vector3::new(
        -camera_position.x,
        -camera_position.y,
        0.0,
    ));

    // Apply the camera matrix to the rendering of the player
    //player.draw_sprite(frame, camera_matrix);

}
