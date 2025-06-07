use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::time::Duration;
struct Button {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: (u8, u8, u8),
}
struct Inputfield {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    text: String,
    color: (u8, u8, u8),
    pressed: bool,
}
trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
impl Drawable for Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        canvas.fill_rect(rect).unwrap();
    }
}
impl Inputfield {
    fn draw_with_text(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        canvas.fill_rect(rect).unwrap();
        if !self.text.is_empty() {
            let surface = font
                .render(&self.text)
                .blended(Color::RGB(255, 255, 255))
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let text_rect = Rect::new(self.x + 5, self.y + 10, surface.width(), surface.height());
            canvas.copy(&texture, None, Some(text_rect)).unwrap();
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let window = video_subsystem
        .window("Stock market game", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let font_path = "src/font.ttf";
    let font = ttf_context.load_font(font_path, 24).unwrap();

    let button_login = Button {
        x: 860,
        y: 720,
        w: 200,
        h: 100,
        color: (204, 41, 54),
    };
    let mut email_login_fieald = Inputfield {
        x: 860,
        y: 420,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == MouseButton::Left && point_in_button(x, y, &button_login) {
                        println!("Button clicked");
                        // let _ = send_login_data();
                    } else if mouse_btn == MouseButton::Left
                        && point_in_input_field(x, y, &email_login_fieald)
                    {
                        email_login_fieald.pressed = !email_login_fieald.pressed;
                    }
                }
                Event::TextInput { text, .. } => {
                    if email_login_fieald.pressed {
                        email_login_fieald.text.push_str(&text);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if email_login_fieald.pressed {
                        email_login_fieald.text.pop();
                    }
                }
                _ => {}
            }
        }
        let colors = vec![8 as u8, 65 as u8, 92 as u8];
        let _ = bg_color(&mut canvas, colors);
        button_login.draw(&mut canvas);
        email_login_fieald.draw_with_text(&mut canvas, &font, &texture_creator);
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
fn bg_color(canvas: &mut Canvas<Window>, colors: Vec<u8>) {
    canvas.set_draw_color(Color::RGB(colors[0], colors[1], colors[2]));
    canvas.clear();
}

fn point_in_button(x: i32, y: i32, button: &Button) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}
fn point_in_input_field(x: i32, y: i32, bar: &Inputfield) -> bool {
    x >= bar.x && x <= bar.x + bar.w as i32 && y >= bar.y && y <= bar.y + bar.h as i32
}
