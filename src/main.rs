use raylib::prelude::*;

enum GameScreen {
    Logo,
    Title,
    Gameplay
}

struct Player {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32
}

fn main() {
    let screen_width = 1280;
    let screen_height = 720;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    let mut current_screen = GameScreen::Logo;

    let mut frame_counter = 0;

    let mut player = Player {x: (screen_width/2) as f32, y: screen_height as f32, dx: 0.0, dy: 0.0};

    while !rl.window_should_close() {
        // Update
        match current_screen {
            GameScreen::Logo => {
                frame_counter += 1;

                if frame_counter > 120 {
                    current_screen = GameScreen::Title;
                }
            }
            GameScreen::Title => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    current_screen = GameScreen::Gameplay;
                }
            }
            GameScreen::Gameplay => {
                player.dx = 0.0;
                player.dy += 0.1;
                if player.y >= screen_height as f32 {
                    player.dy = 0.0;
                }
                if rl.is_key_down(KeyboardKey::KEY_A) {
                    player.dx -= 5.0;
                }
                if rl.is_key_down(KeyboardKey::KEY_D) {
                    player.dx += 5.0;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_W) {
                    player.dy = -5.0;
                }

                player.x += player.dx;
                player.y += player.dy;

                if player.y >= screen_height as f32 {
                    player.y = screen_height as f32;
                }
            }
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        match current_screen {
            GameScreen::Logo => {
                d.draw_text("Logo", screen_width/2, screen_height/2, 40, Color::LIGHTGRAY);
                d.draw_text("Copyright notice", screen_width/2, screen_height*3/4, 20, Color::GRAY);
            }
            GameScreen::Title => {
                d.draw_text("TITLE SCREEN", 20, 20, 40, Color::LIGHTGRAY);
                d.draw_text("press enter to start", 130, 220, 20, Color::GRAY);
            }
            GameScreen::Gameplay => {
                d.draw_text("GAMEPLAY SCREEN", 20, 20, 40, Color::MAROON);
                draw_player(&mut d, &player);
            }
        }
    }
}

fn draw_player(d: &mut RaylibDrawHandle, player: &Player) {
    d.draw_rectangle(player.x as i32 - 20, player.y as i32 - 40, 40, 40, Color::RED);
}