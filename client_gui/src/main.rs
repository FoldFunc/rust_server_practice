mod send_to_server;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;

// ------------------ UI COMPONENTS ------------------

struct Button {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color_bg: (u8, u8, u8),
    color_fg: (u8, u8, u8),
    text: String,
}

struct Text {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    text: String,
    color: (u8, u8, u8),
}
#[derive(Clone)]
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

// ------------------ ENUMS ------------------

#[derive(PartialEq)]
enum View {
    Login,
    Register,
    MainScreen,
}

// ------------------ HELPERS ------------------

fn point_in_button(x: i32, y: i32, button: &Button) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}

fn point_in_input_field(x: i32, y: i32, field: &InputField) -> bool {
    x >= field.x && x <= field.x + field.w as i32 && y >= field.y && y <= field.y + field.h as i32
}

fn bg_color(canvas: &mut Canvas<Window>, colors: Vec<u8>) {
    canvas.set_draw_color(Color::RGB(colors[0], colors[1], colors[2]));
    canvas.clear();
}

// ------------------ IMPLS ------------------

impl Button {
    fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        color_bg: (u8, u8, u8),
        color_fg: (u8, u8, u8),
        text: String,
    ) -> Self {
        Button {
            x,
            y,
            w,
            h,
            color_bg,
            color_fg,
            text,
        }
    }

    fn draw_with_text(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        canvas.set_draw_color(Color::RGB(
            self.color_bg.0,
            self.color_bg.1,
            self.color_bg.2,
        ));
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        canvas.fill_rect(rect).unwrap();

        let surface = font
            .render(&self.text)
            .blended(Color::RGB(
                self.color_fg.0,
                self.color_fg.1,
                self.color_fg.2,
            ))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let text_rect = Rect::new(
            self.x + ((self.w - surface.width()) / 2) as i32,
            self.y + ((self.h - surface.height()) / 2) as i32,
            surface.width(),
            surface.height(),
        );
        canvas.copy(&texture, None, Some(text_rect)).unwrap();
    }
}

impl Text {
    fn new(x: i32, y: i32, w: u32, h: u32, color: (u8, u8, u8), text: String) -> Self {
        Text {
            x,
            y,
            w,
            h,
            color,
            text,
        }
    }

    fn draw_with_text(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        canvas.set_draw_color(Color::RGB(8, 65, 92));
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.w, self.h))
            .unwrap();

        let surface = font
            .render(&self.text)
            .blended(Color::RGB(self.color.0, self.color.1, self.color.2))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let rect = Rect::new(
            self.x + ((self.w - surface.width()) / 2) as i32,
            self.y + ((self.h - surface.height()) / 2) as i32,
            surface.width(),
            surface.height(),
        );
        canvas.copy(&texture, None, Some(rect)).unwrap();
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
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.w, self.h))
            .unwrap();

        let display_text = if self.is_password {
            "*".repeat(self.text.len())
        } else {
            self.text.clone()
        };
        if !self.text.trim().is_empty() {
            let surface = font
                .render(&display_text)
                .blended(Color::RGB(255, 255, 255))
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            canvas
                .copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        self.x + 5,
                        self.y + 10,
                        surface.width(),
                        surface.height(),
                    )),
                )
                .unwrap();
        } else {
        }
    }
}

// ------------------ MAIN ------------------

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    sdl2::image::init(InitFlag::PNG).unwrap();

    let window = video_subsystem
        .window("Stock market game", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let font = ttf_context.load_font("src/assets/font.ttf", 24).unwrap();

    let mut current_view = View::Login;
    let mut login_result_rx: Option<Receiver<bool>> = None;
    let mut register_result_rx: Option<Receiver<bool>> = None;

    // UI Components
    let button_login = Button::new(
        860,
        620,
        200,
        100,
        (204, 41, 54),
        (255, 255, 255),
        "Login".to_string(),
    );
    let button_register = Button::new(
        860,
        620,
        200,
        100,
        (204, 41, 54),
        (255, 255, 255),
        "Register".to_string(),
    );
    let button_change_to_register = Button::new(
        860,
        820,
        200,
        100,
        (8, 65, 92),
        (255, 255, 255),
        "No account?".to_string(),
    );
    let button_change_to_login = Button::new(
        860,
        820,
        200,
        100,
        (8, 65, 92),
        (255, 255, 255),
        "Has account?".to_string(),
    );
    let welcome_text = Text::new(
        810,
        100,
        300,
        100,
        (255, 255, 255),
        "Welcome to stock game".to_string(),
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
    let mut email_register_field = email_login_field.clone();
    let mut password_register_field = password_login_field.clone();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonUp {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => match current_view {
                    View::Login => {
                        if point_in_button(x, y, &button_login) {
                            let email = email_login_field.text.clone();
                            let password = password_login_field.text.clone();
                            let (tx, rx) = channel();
                            login_result_rx = Some(rx);
                            thread::spawn(move || {
                                let rt = tokio::runtime::Runtime::new().unwrap();
                                let result =
                                    rt.block_on(send_to_server::send_login_data(email, password));
                                let _ = tx.send(result.is_ok());
                            });
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
                            let email = email_register_field.text.clone();
                            let password = password_register_field.text.clone();
                            let (tx, rx) = channel();
                            register_result_rx = Some(rx);
                            thread::spawn(move || {
                                let rt = tokio::runtime::Runtime::new().unwrap();
                                let result = rt
                                    .block_on(send_to_server::send_register_data(email, password));
                                let _ = tx.send(result.is_ok());
                            });
                        } else if point_in_button(x, y, &button_change_to_login) {
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
                },
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

        // Check login or register result
        if let Some(rx) = &login_result_rx {
            if let Ok(success) = rx.try_recv() {
                if success {
                    current_view = View::MainScreen;
                } else {
                    println!("Login failed");
                }
                login_result_rx = None;
            }
        }
        if let Some(rx) = &register_result_rx {
            if let Ok(success) = rx.try_recv() {
                if success {
                    current_view = View::MainScreen;
                } else {
                    println!("Register failed");
                }
                register_result_rx = None;
            }
        }

        // DRAWING
        bg_color(&mut canvas, vec![8, 65, 92]);
        match current_view {
            View::Login => {
                button_login.draw_with_text(&mut canvas, &font, &texture_creator);
                button_change_to_register.draw_with_text(&mut canvas, &font, &texture_creator);
                email_login_field.draw_with_text(&mut canvas, &font, &texture_creator);
                password_login_field.draw_with_text(&mut canvas, &font, &texture_creator);
            }
            View::Register => {
                button_register.draw_with_text(&mut canvas, &font, &texture_creator);
                button_change_to_login.draw_with_text(&mut canvas, &font, &texture_creator);
                email_register_field.draw_with_text(&mut canvas, &font, &texture_creator);
                password_register_field.draw_with_text(&mut canvas, &font, &texture_creator);
            }
            View::MainScreen => {
                welcome_text.draw_with_text(&mut canvas, &font, &texture_creator);
            }
        }

        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}
