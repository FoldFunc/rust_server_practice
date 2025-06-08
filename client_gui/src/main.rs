mod send_to_server;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::time::Duration;

struct Button<'a> {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: (u8, u8, u8),
    texture: Option<Texture<'a>>,
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

impl<'a> Drawable for Button<'a> {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        if let Some(texture) = &self.texture {
            let _ = canvas.copy(texture, None, rect);
        } else {
            canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
            let _ = canvas.fill_rect(rect);
        }
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
    let sdl_context = sdl2::init().expect("SDL init failed");
    let video_subsystem = sdl_context.video().expect("SDL video failed");
    let ttf_context = sdl2::ttf::init().expect("TTF init failed");
    sdl2::image::init(InitFlag::PNG).expect("Image init failed");

    let window = video_subsystem
        .window("Stock market game", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let font_path = "src/assets/font.ttf";
    let font = ttf_context
        .load_font(font_path, 24)
        .expect("Failed to load font");

    let texture_login_button = texture_creator
        .load_texture("src/assets/button_login.png")
        .expect("Failed to load button texture");
    let texture_register_button = texture_creator
        .load_texture("src/assets/button_register.png")
        .expect("Failed to load button texture");
    let texture_change_to_register_button = texture_creator
        .load_texture("src/assets/button_change_to_register.png")
        .expect("Failed to load button texture");

    let button_login = Button {
        x: 860,
        y: 620,
        w: 200,
        h: 100,
        color: (204, 41, 54),
        texture: Some(texture_login_button),
    };
    let button_register = Button {
        x: 860,
        y: 620,
        w: 200,
        h: 100,
        color: (204, 41, 54),
        texture: Some(texture_register_button),
    };
    let button_change_to_register = Button {
        x: 860,
        y: 820,
        w: 200,
        h: 100,
        color: (8, 65, 92),
        texture: Some(texture_change_to_register_button),
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

    let mut password_login_fieald = Inputfield {
        x: 860,
        y: 220,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
    };
    let mut email_register_fieald = Inputfield {
        x: 860,
        y: 420,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
    };

    let mut password_regsiter_fieald = Inputfield {
        x: 860,
        y: 220,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
    };
    let mut current_view = "login".to_string();
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
                    if mouse_btn == MouseButton::Left
                        && point_in_button(x, y, &button_login)
                        && current_view == "login".to_string()
                    {
                        println!("Button clicked");
                        println!("Information to send email: {}", email_login_fieald.text);
                        println!(
                            "Information to send password: {}",
                            password_login_fieald.text
                        );
                        let email = &email_login_fieald.text;
                        let password = &password_login_fieald.text;
                        let _ = send_to_server::send_login_data(
                            email.to_string(),
                            password.to_string(),
                        );
                        email_login_fieald.text = "".to_string();
                        password_login_fieald.text = "".to_string();
                    } else if mouse_btn == MouseButton::Left
                        && point_in_button(x, y, &button_register)
                        && current_view == "register".to_string()
                    {
                        println!("Register button clicked");
                        println!("Information to send email {}", email_register_fieald.text);
                        println!(
                            "Information to send password {}",
                            password_regsiter_fieald.text
                        );
                        let email = email_register_fieald.text;
                        let password = password_regsiter_fieald.text;
                        let _ = send_to_server::send_register_data(
                            password.to_string(),
                            email.to_string(),
                        );
                        email_register_fieald.text = "".to_string();
                        password_regsiter_fieald.text = "".to_string();
                    } else if mouse_btn == MouseButton::Left
                        && point_in_input_field(x, y, &email_login_fieald)
                        && current_view == "login".to_string()
                    {
                        email_login_fieald.pressed = !email_login_fieald.pressed;
                        password_login_fieald.pressed = false;
                    } else if mouse_btn == MouseButton::Left
                        && point_in_input_field(x, y, &password_login_fieald)
                        && current_view == "login".to_string()
                    {
                        password_login_fieald.pressed = !password_login_fieald.pressed;
                        email_login_fieald.pressed = false;
                    } else if mouse_btn == MouseButton::Left
                        && point_in_button(x, y, &button_change_to_register)
                        && current_view == "login".to_string()
                    {
                        println!("Changing to register view");
                        current_view = "register".to_string();
                    } else if mouse_btn == MouseButton::Left
                        && point_in_input_field(x, y, &email_register_fieald)
                        && current_view == "register".to_string()
                    {
                        email_register_fieald.pressed = !email_register_fieald.pressed;
                        password_regsiter_fieald.pressed = false;
                    } else if mouse_btn == MouseButton::Left
                        && point_in_input_field(x, y, &password_regsiter_fieald)
                        && current_view == "register".to_string()
                    {
                        password_regsiter_fieald.pressed = !password_regsiter_fieald.pressed;
                        email_register_fieald.pressed = false;
                    }
                }
                Event::TextInput { text, .. } => {
                    if email_login_fieald.pressed && current_view == "login".to_string() {
                        email_login_fieald.text.push_str(&text);
                    } else if password_login_fieald.pressed && current_view == "login".to_string() {
                        password_login_fieald.text.push_str(&text);
                    } else if email_register_fieald.pressed
                        && current_view == "register".to_string()
                    {
                        email_register_fieald.text.push_str(&text);
                    } else if password_regsiter_fieald.pressed
                        && current_view == "register".to_string()
                    {
                        password_regsiter_fieald.text.push_str(&text);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if email_login_fieald.pressed && current_view == "login".to_string() {
                        email_login_fieald.text.pop();
                    } else if password_login_fieald.pressed && current_view == "login".to_string() {
                        password_login_fieald.text.pop();
                    } else if email_register_fieald.pressed
                        && current_view == "register".to_string()
                    {
                        email_register_fieald.text.pop();
                    } else if password_regsiter_fieald.pressed
                        && current_view == "register".to_string()
                    {
                        password_regsiter_fieald.text.pop();
                    }
                }
                _ => {}
            }
        }

        let _ = bg_color(&mut canvas, vec![8, 65, 92]);
        if current_view == "login".to_string() {
            button_login.draw(&mut canvas);
            button_change_to_register.draw(&mut canvas);
            email_login_fieald.draw_with_text(&mut canvas, &font, &texture_creator);
            password_login_fieald.draw_with_text(&mut canvas, &font, &texture_creator);
        } else if current_view == "register".to_string() {
            email_register_fieald.draw_with_text(&mut canvas, &font, &texture_creator);
            password_regsiter_fieald.draw_with_text(&mut canvas, &font, &texture_creator);
            button_register.draw(&mut canvas);
        }
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
