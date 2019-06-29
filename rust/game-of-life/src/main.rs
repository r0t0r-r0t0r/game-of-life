extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Graphics;
use graphics::math;
use rand::Rng;

type IntColor = [u8; 4];
type FloatColor = [f32; 4];

const BLACK: IntColor = [0x23, 0x23, 0x23, 0xff];
const YELLOW: IntColor = [0xdb, 0xd5, 0x80, 0xff];
const GRAY: IntColor = [0x84, 0x84, 0x84, 0xff];

fn color_int_to_float(c: &IntColor) -> FloatColor {
    fn conv(n: u8) -> f32 {
        n as f32 / 255.0
    }

    [conv(c[0]), conv(c[1]), conv(c[2]), conv(c[3])]
}

struct Settings {
    pixel_width: u32,
    pixel_height: u32,

    bg_color: FloatColor,
    fg_color: FloatColor,
    grid_color: FloatColor,

    x_pixel_count: u32,
    y_pixel_count: u32,

    grid: bool,
}

#[derive(Clone)]
struct Buf {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Buf {
    fn new(w: usize, h: usize) -> Buf {
        Buf {
            data: vec![false; w * h],
            width: w,
            height: h,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.data[y * self.width + x] = value;
    }
}

struct Field {
    curr_buf: Buf,
    next_buf: Buf,
    width: usize,
    height: usize,
}

impl Field {
    fn new(size: [usize; 2], initial_buf: Buf) -> Field {
        let next_buf = initial_buf.clone();
        Field {
            curr_buf: initial_buf,
            next_buf,
            width: size[0],
            height: size[1],
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.curr_buf.get(x, y)
    }

    fn get_live_neighbour_count(&self, x: usize, y: usize) -> usize {
        let prev_x = if x > 0 { x - 1 } else { self.width - 1 };
        let next_x = if x < (self.width - 1) { x + 1 } else { 0 };
        let prev_y = if y > 0 { y - 1 } else { self.height - 1 };
        let next_y = if y < (self.height - 1) { y + 1 } else { 0 };

        fn bool_to_num(b: bool) -> usize {
            if b { 1 } else { 0 }
        }

        let buf = &self.curr_buf;

        let result =
            bool_to_num(buf.get(x, prev_y)) +
                bool_to_num(buf.get(next_x, prev_y)) +
                bool_to_num(buf.get(next_x, y)) +
                bool_to_num(buf.get(next_x, next_y)) +
                bool_to_num(buf.get(x, next_y)) +
                bool_to_num(buf.get(prev_x, next_y)) +
                bool_to_num(buf.get(prev_x, y)) +
                bool_to_num(buf.get(prev_x, prev_y));

        result
    }

    fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let is_dead = !self.curr_buf.get(x, y);
                let neighbour_count = self.get_live_neighbour_count(x, y);

                let mut next_alive = false;

                if is_dead {
                    if neighbour_count == 3 {
                        next_alive = true;
                    }
                } else {
                    if neighbour_count < 2 {
                        next_alive = false;
                    } else if neighbour_count == 2 || neighbour_count == 3 {
                        next_alive = true;
                    } else {
                        next_alive = false;
                    }
                }

                self.next_buf.set(x, y, next_alive);
            }
        }
        std::mem::swap(&mut self.curr_buf, &mut self.next_buf);
    }
}

struct BufBuilder {
    buf: Buf,
    width: usize,
    height: usize,
}

impl BufBuilder {
    pub fn new(w: usize, h: usize) -> BufBuilder {
        BufBuilder {
            buf: Buf::new(w, h),
            width: w,
            height: h,
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, value: bool) {
        for cy in y..(y + height) {
            for cx in x..(x + width) {
                self.buf.set(cx, cy, value);
            }
        }
    }

    pub fn place_glider(&mut self, x: usize, y: usize) {
        self.fill_rect(x, y, 3, 3, false);

        self.buf.set(x + 2, y, true);
        self.buf.set(x, y + 1, true);
        self.buf.set(x + 2, y + 1, true);
        self.buf.set(x + 1, y + 2, true);
        self.buf.set(x + 2, y + 2, true);
    }

    pub fn place_pentadecathlon(&mut self, x: usize, y: usize) {
        self.fill_rect(x, y, 3, 8, true);

        self.buf.set(x + 1, y + 1, false);
        self.buf.set(x + 1, y + 6, false);
    }

    pub fn place_acorn(&mut self, x: usize, y: usize) {
        self.fill_rect(x, y, 7, 3, false);

        self.buf.set(x + 1, y, true);
        self.buf.set(x + 3, y + 1, true);
        self.buf.set(x, y + 2, true);
        self.buf.set(x + 1, y + 2, true);
        self.buf.set(x + 4, y + 2, true);
        self.buf.set(x + 5, y + 2, true);
        self.buf.set(x + 6, y + 2, true);
    }

    pub fn fill_random(&mut self) {
        let mut rng = rand::thread_rng();

        for cy in 0..self.height {
            for cx in 0..self.width {
                let value: f64 = rng.gen();
                self.buf.set(cx, cy, value > 0.8);
            }
        }
    }

    pub fn clear(&mut self) {
        self.fill_rect(0, 0, self.width, self.height, false);
    }

    fn build(&self) -> Buf {
        self.buf.clone()
    }
}

pub struct App {
    gl: GlGraphics,

    settings: Settings,
    field: Field,

    t_buf: f64,
}

fn calc_grid_size(count: [u32; 2], step: [u32; 2]) -> [u32; 2] {
    let [x_count, y_count] = count;
    let [x_step, y_step] = step;

    let width = x_count * x_step;
    let height = y_count * y_step;

    [width, height]
}

fn draw_grid<G>(color: FloatColor, count: [u32; 2], step: [u32; 2], transform: math::Matrix2d, g: &mut G)
    where G: Graphics {
    use graphics::*;

    let [x_count, y_count] = count;
    let [x_step, y_step] = step;

    let size = calc_grid_size(count, step);
    let width = size[0] as f64;
    let height = size[1] as f64;

    let radius = 0.5;

    for i in 0..(y_count + 1) {
        let y = (i * y_step) as f64;
        line(color, radius, [0.0, y, width, y], transform, g);
    }

    for i in 0..(x_count + 1) {
        let x = (i * x_step) as f64;
        line(color, radius, [x, 0.0, x, height], transform, g);
    }
}

fn draw_field<G>(
    field: &Field,
    settings: &Settings,
    transform: math::Matrix2d,
    g: &mut G,
) where G: Graphics {
    use graphics::*;

    for y in 0..field.height {
        for x in 0..field.width {
            if field.get(x, y) {
                rectangle(
                    settings.fg_color,
                    [0.0, 0.0, settings.pixel_width as f64, settings.pixel_height as f64],
                    transform.trans((x * settings.pixel_width as usize) as f64, (y * settings.pixel_height as usize) as f64),
                    g,
                );
            }
        }
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        let settings = &self.settings;
        let field = &self.field;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(settings.bg_color, gl);

            let grid_size = calc_grid_size(
                [settings.x_pixel_count, settings.y_pixel_count],
                [settings.pixel_width, settings.pixel_height],
            );

            let field_transform = c.transform.trans(x - grid_size[0] as f64 / 2.0, y - grid_size[1] as f64 / 2.0);

            draw_field(
                field,
                settings,
                field_transform,
                gl,
            );

            if settings.grid {
                draw_grid(
                    settings.grid_color,
                    [settings.x_pixel_count, settings.y_pixel_count],
                    [settings.pixel_width, settings.pixel_height],
                    field_transform,
                    gl,
                )
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.t_buf = self.t_buf + args.dt;

        let period = 0.016 * 5.0;
        if self.t_buf > period {
            self.field.step();

            self.t_buf = self.t_buf - period;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let settings = Settings {
        pixel_width: 10,
        pixel_height: 10,

        bg_color: color_int_to_float(&BLACK),
        fg_color: color_int_to_float(&YELLOW),
        grid_color: color_int_to_float(&GRAY),

        x_pixel_count: 80,
        y_pixel_count: 80,

        grid: true,
    };

    let size = calc_grid_size([settings.x_pixel_count, settings.y_pixel_count], [settings.pixel_width, settings.pixel_height]);

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Game of Life",
        [size[0] + 1, size[1] + 1],
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let width = settings.x_pixel_count as usize;
    let height = settings.y_pixel_count as usize;

    let mut builder = BufBuilder::new(
        width,
        height,
    );

    builder.fill_random();

    let field = Field::new([width, height], builder.build());

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        settings,
        field,
        t_buf: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
