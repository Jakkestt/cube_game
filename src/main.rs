extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx_graphics;
extern crate gfx;

mod object;

use piston_window::*;
use object::Object;

pub enum Event {
    Input(Input),
    Loop(Loop),
}

pub enum Input {
}

pub struct Game {
    width: f64,
    height: f64,
    rotation: f64,
    player: Object,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}



impl Game {
    fn new() -> Self {
        Game {
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false,
        }
    }
    fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let tank_sprite = assets.join("E-100_Base.png");
        let tank_sprite: G2dTexture = Texture::from_path(&mut w.factory,
                                                         &tank_sprite,
                                                         Flip::None,
                                                         &TextureSettings::new())
                .expect("couldn't find tank_sprite");

        self.player.hull.set_sprite(tank_sprite.clone());
        self.player.turret.set_sprite(turret_sprite.clone());
        self.player2.hull.set_sprite(tank_sprite);
        self.player2.turret.set_sprite(turret_sprite);
        self.player2.mov_to(Vec2::new(200.0, 0.0));

    }

    fn on_update(&mut self, upd: &UpdateArgs) {
        if self.up_d {
            //self.player.mov(0.0, -150.0 * upd.dt);
            self.player.fwd(150.0 * upd.dt);
        }
        if self.down_d {
            //self.player.mov(0.0, 150.0 * upd.dt);
            self.player.fwd(-150.0 * upd.dt);
        }
        if self.left_d {
            //self.player.mov(-150.0 * upd.dt, 0.0);
            self.player.rot(-1.0 * upd.dt);
        }
        if self.right_d {
            //self.player.mov(150.0 * upd.dt, 0.0);
            self.player.rot(1.0 * upd.dt);
        }
        self.player.update(upd.dt);
        for bul in &mut self.bullets {
            if self.player.collides(&bul) {
                self.player.is_destroyed = true;
                self.player.hull.set_sprite(self.hull_destroyed.clone().unwrap());
                self.player.turret.set_sprite(self.turret_destroyed.clone().unwrap());
                bul.to_be_removed = true;
            }
            if self.player2.collides(&bul) {
                self.player2.is_destroyed = true;
                self.player2.hull.set_sprite(self.hull_destroyed.clone().unwrap());
                self.player2.turret.set_sprite(self.turret_destroyed.clone().unwrap());
                bul.to_be_removed = true;
            }
            bul.update(upd.dt);
        }
        self.bullets.retain(|ref bul| bul.to_be_removed == false);

    }
    fn on_mouse_mov(&mut self, coord: [f64; 2]) {
        let (x, y) = (coord[0], coord[1]);
        self.player.point_tur_to(x - self.scx, y - self.scy);
    }
    fn on_draw(&mut self, c: &Context, g: &mut G2d, ren: &RenderArgs) {
        clear([0.8, 0.8, 0.8, 1.0], g);
        self.scx = (ren.width / 2) as f64;
        self.scy = (ren.height / 2) as f64;
        let center = c.transform.trans(self.scx, self.scy);
        self.player.render(center, g);
        self.player2.render(center, g);
        for bul in &mut self.bullets {
            bul.render(center, g);
        }
    }
    fn on_input(&mut self, button_args: &ButtonArgs) {
        if let Button::Mouse(MouseButton::Left) = button_args.button {
            self.bullets.push(self.player.fire(self.bullet.clone().unwrap()));
        }
        match button_args.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Right => self.right_d = true,
                        Key::Left => self.left_d = true,
                        Key::Down => self.down_d = true,
                        Key::Up => self.up_d = true,
                        _ => {}
                    }
                }
            }
            ButtonState::Release => {

                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Right => self.right_d = false,
                        Key::Left => self.left_d = false,
                        Key::Down => self.down_d = false,
                        Key::Up => self.up_d = false,
                       /*  Key::Space => {
                            self.bullets
                                .push(self.player.fire(self.bullet.clone().unwrap()));
                        } */
                        _ => {}
                    }
                }
            }
        }
    }
}


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Cube", [1280, 720])
        .fullscreen(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut game = Game::new();
        game.on_load(&mut window);
        while let Some(e) = window.next() {
            if let Some(uargs) = e.update_args() {
                game.on_update(&uargs);
            }
            if let Some(render_args) = e.render_args() {
                window.draw_2d(&e, |c, g| game.on_draw(&c, g, &render_args));
            }
            if let Some(input) = e.button_args() {
                game.on_input(&input);
            }

            if let Some(mouse) = e.mouse_cursor_args() {
                game.on_mouse_mov(mouse);
            }
        }
    }
