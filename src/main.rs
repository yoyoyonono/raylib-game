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

struct Platform {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct CollisionInfo {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
        y: (SCREEN_HEIGHT - 40) as f32,
        dx: 0.0,
        dy: 0.0,
    };

    let mut platforms = Vec::new();
    platforms.push(Platform::new(
        0.0,
        SCREEN_HEIGHT as f32 - 40.0,
        SCREEN_WIDTH as f32,
        40.0,
    ));
    platforms.push(Platform::new(
        (SCREEN_WIDTH * 2 / 3) as f32,
        SCREEN_HEIGHT as f32 - 100.0,
        (SCREEN_WIDTH / 3) as f32,
        40.0,
    ));

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
                // check collision

                let player_collision_info = player.get_collision_info();

                for platform in platforms.iter() {
                    let platform_collision_info = platform.get_collision_info();

                    if is_colliding(&player_collision_info, &platform_collision_info) {
                        player.dy = 0.0;
                        player.dx = 0.0;
                    }
                }

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

                player.do_physics();
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
                platforms.iter().for_each(|p| p.draw(&mut d));
            }
        }
    }
}

impl Player {
    fn do_physics(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        self.dy += 0.1;
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

    fn stop(&mut self) {
        self.dx = 0.0;
    }

    fn get_collision_info(&self) -> CollisionInfo {
        CollisionInfo {
            x: self.x - 20.0,
            y: self.y - 40.0,
            width: 40.0,
            height: 40.0,
        }
    }
}

impl Platform {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Platform {
        Platform {
            x,
            y,
            width,
            height,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.x as i32,
            self.y as i32,
            self.width as i32,
            self.height as i32,
            Color::GRAY,
        );
    }

    fn get_collision_info(&self) -> CollisionInfo {
        CollisionInfo {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

fn is_colliding(a: &CollisionInfo, b: &CollisionInfo) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}
