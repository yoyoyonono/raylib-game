use raylib::prelude::*;

enum GameScreen {
    Logo,
    Title,
    Gameplay,
}

struct Player {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}
static SCREEN_WIDTH: i32 = 1280;
static SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    let mut current_screen = GameScreen::Logo;

    let mut frame_counter = 0;

    let mut player = Player {
        x: (SCREEN_WIDTH / 2) as f32,
        y: SCREEN_HEIGHT as f32,
        dx: 0.0,
        dy: 0.0,
    };

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
                let mut left_or_right = 0;
                if rl.is_key_down(KeyboardKey::KEY_A) {
                    left_or_right -= 1;
                }
                if rl.is_key_down(KeyboardKey::KEY_D) {
                    left_or_right += 1
                }
                if rl.is_key_pressed(KeyboardKey::KEY_W) {
                    player.jump();
                }

                if left_or_right < 0 {
                    player.left();
                } else if left_or_right > 0 {
                    player.right();                    
                } else {
                    player.stop();
                }

                player.update();
            }
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        match current_screen {
            GameScreen::Logo => {
                d.draw_text(
                    "Logo",
                    SCREEN_WIDTH / 2,
                    SCREEN_HEIGHT / 2,
                    40,
                    Color::LIGHTGRAY,
                );
                d.draw_text(
                    "Copyright notice",
                    SCREEN_WIDTH / 2,
                    SCREEN_HEIGHT * 3 / 4,
                    20,
                    Color::GRAY,
                );
            }
            GameScreen::Title => {
                d.draw_text("TITLE SCREEN", 20, 20, 40, Color::LIGHTGRAY);
                d.draw_text("press enter to start", 130, 220, 20, Color::GRAY);
            }
            GameScreen::Gameplay => {
                d.draw_text("GAMEPLAY SCREEN", 20, 20, 40, Color::MAROON);
                player.draw(&mut d);
            }
        }
    }
}

impl Player {
    fn update(&mut self) {
        self.dy += 0.1;

        self.x += self.dx;
        self.y += self.dy;

        if self.y >= SCREEN_HEIGHT as f32 {
            self.y = SCREEN_HEIGHT as f32;
            self.dy = 0.0;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x as i32 - 20, self.y as i32 - 40, 40, 40, Color::RED);
    }

    fn jump(&mut self) {
        self.dy = -5.0;
    }

    fn left(&mut self) {
        self.dx = -5.0;
    }

    fn right(&mut self) {
        self.dx = 5.0;
    }

    fn stop (&mut self) {
        self.dx = 0.0;
    }
}
