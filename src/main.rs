extern crate piston_window;

use piston_window::*;

pub struct Game {
    rotation: f64,
    x: f64,
    y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}



impl Game {
    fn new() -> Game {
        Game { rotation : 0.0, x : 0.0, y : 0.0, up_d: false, down_d: false, left_d: false, right_d: false }
    }
    fn on_draw(&mut self, e: Input, ren: RenderArgs, w: &mut PistonWindow) {
        //const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        //const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let size = 100.0;

        w.draw_2d(&e, |c, g| {
            clear(WHITE, g);
            let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, size);
            rectangle(BLUE, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 0.0 * upd.dt;
        if self.y >= 0.0 {
            self.y += (10.0) * 9.0 * upd.dt;
        }
        if 100.0 / 2.0 + self.y <= 0.0 {
            self.y *= (0.0) * upd.dt;
        }
        if self.up_d {
            self.y += (-100.0) * upd.dt;
        }
        if self.down_d {
            self.y += (100.0) * upd.dt;
        }
        if self.left_d {
            self.x += (-100.0) * upd.dt;
        }
        if self.right_d {
            self.x += (100.0) * upd.dt;
        }
    }
    fn on_press(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn on_release(&mut self, inp: Input) {
        match inp {
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
 }

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Cube", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    while let Some(e) = window.next() {
       match e {
           Input::Render(r) => {
               game.on_draw(e, r, &mut window);
           }
           Input::Update(u) => {
               game.on_update(u);
           }
           Input::Press(_) => {
               game.on_press(e);
           }
           Input::Release(_) => {
               game.on_release(e);
           }
           _ => {}
       }
   }
}
