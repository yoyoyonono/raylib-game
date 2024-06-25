use raylib::prelude::*;

enum GameScreen {
    Logo,
    Title,
    Gameplay,
    Ending
}

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    let mut current_screen = GameScreen::Logo;

    let mut frame_counter = 0;

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
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    current_screen = GameScreen::Ending;
                }
            }
            GameScreen::Ending => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    current_screen = GameScreen::Title;
                }
            }
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);

        match current_screen {
            GameScreen::Logo => {
                d.draw_text("LOGO SCREEN", 20, 20, 40, Color::LIGHTGRAY);
                d.draw_text("WAIT for 2 SECONDS...", 290, 220, 20, Color::GRAY);
            }
            GameScreen::Title => {
                d.draw_rectangle(0, 0, screen_width, screen_height, Color::GREEN);
                d.draw_text("TITLE SCREEN", 20, 20, 40, Color::DARKGREEN);
                d.draw_text("PRESS ENTER to JUMP to GAMEPLAY SCREEN", 130, 220, 20, Color::DARKGREEN);
            }
            GameScreen::Gameplay => {
                d.draw_rectangle(0, 0, screen_width, screen_height, Color::PURPLE);
                d.draw_text("GAMEPLAY SCREEN", 20, 20, 40, Color::MAROON);
                d.draw_text("PRESS ENTER to JUMP to ENDINGS SCREEN", 130, 220, 20, Color::MAROON);
            }
            GameScreen::Ending => {
                d.draw_rectangle(0, 0, screen_width, screen_height, Color::BLUE);
                d.draw_text("ENDING SCREEN", 20, 20, 40, Color::DARKBLUE);
                d.draw_text("PRESS ENTER to RETURN to TITLE SCREEN", 120, 220, 20, Color::DARKBLUE);
            }
        }
    }
}
