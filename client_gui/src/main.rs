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

struct InputField {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    text: String,
    color: (u8, u8, u8),
    pressed: bool,
    is_password: bool,
}

trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

impl<'a> Button<'a> {
    fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        color: (u8, u8, u8),
        texture: Option<Texture<'a>>,
    ) -> Self {
        Button {
            x,
            y,
            w,
            h,
            color,
            texture,
        }
    }
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

impl InputField {
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
            let display_text = if self.is_password {
                "*".repeat(self.text.len())
            } else {
                self.text.clone()
            };

            let surface = font
                .render(&display_text)
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

#[derive(PartialEq)]
enum View {
    Login,
    Register,
    MainScreen,
}

fn load_texture_or_panic<'a>(
    path: &str,
    creator: &'a TextureCreator<WindowContext>,
) -> Texture<'a> {
    creator
        .load_texture(path)
        .unwrap_or_else(|_| panic!("Failed to load texture: {path}"))
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

fn point_in_input_field(x: i32, y: i32, field: &InputField) -> bool {
    x >= field.x && x <= field.x + field.w as i32 && y >= field.y && y <= field.y + field.h as i32
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

    let font = ttf_context
        .load_font("src/assets/font.ttf", 24)
        .expect("Failed to load font");

    let button_login = Button::new(
        860,
        620,
        200,
        100,
        (204, 41, 54),
        Some(load_texture_or_panic(
            "src/assets/button_login.png",
            &texture_creator,
        )),
    );
    let button_register = Button::new(
        860,
        620,
        200,
        100,
        (204, 41, 54),
        Some(load_texture_or_panic(
            "src/assets/button_register.png",
            &texture_creator,
        )),
    );
    let button_change_to_register = Button::new(
        860,
        820,
        200,
        100,
        (8, 65, 92),
        Some(load_texture_or_panic(
            "src/assets/button_change_to_register.png",
            &texture_creator,
        )),
    );

    let mut email_login_field = InputField {
        x: 860,
        y: 220,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
        is_password: false,
    };
    let mut password_login_field = InputField {
        x: 860,
        y: 420,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
        is_password: true,
    };
    let mut email_register_field = InputField {
        x: 860,
        y: 220,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
        is_password: false,
    };
    let mut password_register_field = InputField {
        x: 860,
        y: 420,
        w: 200,
        h: 100,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
        is_password: true,
    };

    let mut current_view = View::Login;

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
                    if mouse_btn == MouseButton::Left {
                        match current_view {
                            View::Login => {
                                if point_in_button(x, y, &button_login) {
                                    println!("Button login clicked");
                                    let email = email_login_field.text.clone();
                                    let password = password_login_field.text.clone();
                                    std::thread::spawn(move || {
                                        let rt = tokio::runtime::Runtime::new().unwrap();
                                        match rt.block_on(send_to_server::send_login_data(
                                            email, password,
                                        )) {
                                            Ok(_) => println!("Login successful"),
                                            Err(e) => eprintln!("Login error: {}", e),
                                        }
                                    });
                                    email_login_field.text.clear();
                                    password_login_field.text.clear();
                                    current_view = View::MainScreen;
                                } else if point_in_button(x, y, &button_change_to_register) {
                                    current_view = View::Register;
                                } else if point_in_input_field(x, y, &email_login_field) {
                                    email_login_field.pressed = true;
                                    password_login_field.pressed = false;
                                } else if point_in_input_field(x, y, &password_login_field) {
                                    password_login_field.pressed = true;
                                    email_login_field.pressed = false;
                                }
                            }
                            View::Register => {
                                if point_in_button(x, y, &button_register) {
                                    println!("Button register clicked");
                                    let email = email_register_field.text.clone();
                                    let password = password_register_field.text.clone();
                                    std::thread::spawn(move || {
                                        let rt = tokio::runtime::Runtime::new().unwrap();
                                        match rt.block_on(send_to_server::send_register_data(
                                            email, password,
                                        )) {
                                            Ok(_) => println!("Register successful"),
                                            Err(e) => eprintln!("Register error: {}", e),
                                        }
                                    });
                                    email_register_field.text.clear();
                                    password_register_field.text.clear();
                                    current_view = View::Login;
                                } else if point_in_input_field(x, y, &email_register_field) {
                                    email_register_field.pressed = true;
                                    password_register_field.pressed = false;
                                } else if point_in_input_field(x, y, &password_register_field) {
                                    password_register_field.pressed = true;
                                    email_register_field.pressed = false;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::TextInput { text, .. } => match current_view {
                    View::Login => {
                        if email_login_field.pressed {
                            email_login_field.text.push_str(&text);
                        }
                        if password_login_field.pressed {
                            password_login_field.text.push_str(&text);
                        }
                    }
                    View::Register => {
                        if email_register_field.pressed {
                            email_register_field.text.push_str(&text);
                        }
                        if password_register_field.pressed {
                            password_register_field.text.push_str(&text);
                        }
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => match current_view {
                    View::Login => {
                        if email_login_field.pressed {
                            email_login_field.text.pop();
                        }
                        if password_login_field.pressed {
                            password_login_field.text.pop();
                        }
                    }
                    View::Register => {
                        if email_register_field.pressed {
                            email_register_field.text.pop();
                        }
                        if password_register_field.pressed {
                            password_register_field.text.pop();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        bg_color(&mut canvas, vec![8, 65, 92]);

        match current_view {
            View::Login => {
                button_login.draw(&mut canvas);
                button_change_to_register.draw(&mut canvas);
                email_login_field.draw_with_text(&mut canvas, &font, &texture_creator);
                password_login_field.draw_with_text(&mut canvas, &font, &texture_creator);
            }
            View::Register => {
                button_register.draw(&mut canvas);
                email_register_field.draw_with_text(&mut canvas, &font, &texture_creator);
                password_register_field.draw_with_text(&mut canvas, &font, &texture_creator);
            }
            _ => {}
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
