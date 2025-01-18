use raylib::prelude::*;

fn main() {
    const SCREEN_WIDTH: i32 = 640;
    const SCREEN_HEIGHT: i32 = 480;

    let mut player = Rectangle::new(10.0, 280.0, 40.0, 40.0);

    let mut camera = Camera2D {
        target: Vector2::new(player.x + 20.0, player.y + 20.0),
        offset: Vector2::new(player.x, player.y),
        rotation: 0.0,
        zoom: 1.0,
    };

    let (mut rl, thread) = raylib
        ::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        player.y += 1.0;
        if player.y >= (SCREEN_HEIGHT as f32) {
            player.y = 280.0;
        }
        camera.target = Vector2::new(player.x, player.y);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("text 0", 12, 52, 20, Color::BLACK);

        let mut d2 = d.begin_mode2D(camera);

        d2.draw_rectangle_rec(player, Color::RED);

        d2.draw_line(
            camera.target.x as i32,
            -SCREEN_HEIGHT * 10,
            camera.target.x as i32,
            (SCREEN_HEIGHT as i32) * 10,
            Color::GREEN
        );
        d2.draw_line(
            -(SCREEN_WIDTH as i32) * 10,
            camera.target.y as i32,
            (SCREEN_WIDTH as i32) * 10,
            camera.target.y as i32,
            Color::GREEN
        );

        d2.draw_text("text 1", 12, 52, 20, Color::BLACK);

        // let player_y: String = player.y.to_string();
        // println!("{player_y}");
    }
}
