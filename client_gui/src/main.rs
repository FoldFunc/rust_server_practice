mod send_to_server;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;
// ------------------ UI COMPONENTS ------------------
struct Border {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    border_thick: u32,
    body_color: (u8, u8, u8),
    border_color: (u8, u8, u8),
}
struct Subburgerbuttons {
    bg_color: (u8, u8, u8),
    font_color: (u8, u8, u8),
    text: String,
}
struct Menuburger {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    _texture: sdl2::image::Sdl2ImageContext,
    fields: Vec<Subburgerbuttons>,
}
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
#[derive(PartialEq)]
enum Showburgermenu {
    Show,
    Noshow,
}
// ------------------ HELPERS ------------------

fn point_in_button(x: i32, y: i32, button: &Button) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}
fn point_in_burger_menu(x: i32, y: i32, button: &Menuburger) -> bool {
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

// ------------------ FONT SIZE HELPER ------------------

fn find_fitting_font<'a>(
    ttf_context: &'a Sdl2TtfContext,
    font_path: &str,
    display_text: &str,
    mut font_size: u16,
    field_w: u32,
    field_h: u32,
) -> (Font<'a, 'static>, u16) {
    let min_font_size = 8;
    loop {
        let font = ttf_context.load_font(font_path, font_size).unwrap();
        let surface = font
            .render(display_text)
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        if surface.width() <= field_w && surface.height() <= field_h {
            return (font, font_size);
        }
        if font_size <= min_font_size {
            return (font, font_size);
        }
        font_size -= 1;
    }
}

// ------------------ IMPLS ------------------
impl Border {
    fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        border_thic: u32,
        body_color: (u8, u8, u8),
        border_color: (u8, u8, u8),
    ) -> Self {
        Border {
            x: x,
            y: y,
            w: (w),
            h: (h),
            border_thick: (border_thic),
            body_color: (body_color),
            border_color: (border_color),
        }
    }
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // 1. Border (outer rect)
        let outer_rect = Rect::new(self.x, self.y, self.w, self.h);
        canvas.set_draw_color(Color::RGB(
            self.border_color.0,
            self.border_color.1,
            self.border_color.2,
        ));
        let _ = canvas.fill_rect(outer_rect);

        // 2. Body (inner rect)
        let inner_rect = Rect::new(
            self.x + self.border_thick as i32,
            self.y + self.border_thick as i32,
            self.w - 2 * self.border_thick,
            self.h - 2 * self.border_thick,
        );
        canvas.set_draw_color(Color::RGB(
            self.body_color.0,
            self.body_color.1,
            self.body_color.2,
        ));
        let _ = canvas.fill_rect(inner_rect);
    }
}
impl Subburgerbuttons {
    fn new(bg_color: (u8, u8, u8), font_color: (u8, u8, u8), text: String) -> Self {
        Subburgerbuttons {
            bg_color: bg_color,
            font_color: font_color,
            text: text,
        }
    }
}
impl Menuburger {
    fn new(x: i32, y: i32, w: u32, h: u32, texture: sdl2::image::Sdl2ImageContext) -> Self {
        Menuburger {
            x: (x),
            y: (y),
            w: (w),
            h: (h),
            _texture: (texture),
            fields: Vec::new(),
        }
    }
    fn draw_with_texture(&self, canvas: &mut Canvas<Window>, texture: &Texture) {
        let target = Rect::new(self.x, self.y, self.w, self.h);
        canvas.copy(texture, None, Some(target)).unwrap();
    }
    fn populate(&mut self, subfields: Vec<Vec<Subburgerbuttons>>) {
        self.fields.extend(subfields.into_iter().flatten());
    }
    fn draw_all(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let mut y_offset = self.y + self.h as i32; // Start drawing below the burger button
        for field in &self.fields {
            let field_height = 50;
            let rect = Rect::new(self.x, y_offset, self.w, field_height);
            canvas.set_draw_color(Color::RGB(
                field.bg_color.0,
                field.bg_color.1,
                field.bg_color.2,
            ));
            canvas.fill_rect(rect).unwrap();

            let surface = font
                .render(&field.text)
                .blended(Color::RGB(
                    field.font_color.0,
                    field.font_color.1,
                    field.font_color.2,
                ))
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let text_rect = Rect::new(
                self.x + ((self.w - surface.width()) / 2) as i32,
                y_offset + ((field_height - surface.height()) / 2) as i32,
                surface.width(),
                surface.height(),
            );
            canvas.copy(&texture, None, Some(text_rect)).unwrap();

            y_offset += field_height as i32;
        }
    }
}
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
        canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.w, self.h))
            .unwrap();

        let surface = font
            .render(&self.text)
            .blended(Color::RGB(255, 255, 255))
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

impl InputField {
    fn draw_with_text(
        &self,
        canvas: &mut Canvas<Window>,
        ttf_context: &Sdl2TtfContext,
        font_path: &str,
        font_size: &mut u16,
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
            let (font, fitted_size) = find_fitting_font(
                ttf_context,
                font_path,
                &display_text,
                *font_size,
                self.w,
                self.h,
            );
            *font_size = fitted_size;
            let surface = font
                .render(&display_text)
                .blended(Color::RGB(255, 255, 255))
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
}

// ------------------ MAIN ------------------

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    sdl2::image::init(InitFlag::PNG).unwrap();
    let winwidth = 1920;
    let winheight = 1080;
    let window = video_subsystem
        .window("Stock market game", winwidth, winheight)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut font_size = 20;
    let font_path = "src/assets/font.ttf";
    let font = ttf_context.load_font(font_path, font_size).unwrap();
    let mut current_burger_view = Showburgermenu::Noshow;
    let mut current_view = View::Login;
    let mut login_result_rx: Option<Receiver<bool>> = None;
    let mut register_result_rx: Option<Receiver<bool>> = None;
    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let texture = texture_creator
        .load_texture("src/assets/burger_menu.png")
        .unwrap();
    // UI Components
    let mut burger_menu = Menuburger::new(0, 0, 128, 128, image_context);
    burger_menu.populate(vec![vec![Subburgerbuttons::new(
        (8, 65, 92),
        (255, 255, 255),
        "Add portfolio".to_string(),
    )]]);
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
    let button_logout = Button::new(
        1720,
        0,
        200,
        100,
        (8, 65, 92),
        (255, 255, 255),
        "Logout ]->".to_string(),
    );
    let welcome_text = Text::new(
        810,
        0,
        300,
        100,
        (8, 65, 92),
        "Welcome to stock game".to_string(),
    );
    let welcome_text_login_screen =
        Text::new(810, 0, 300, 100, (8, 65, 92), "Login to play".to_string());
    let welcome_text_register_screen = Text::new(
        810,
        0,
        300,
        100,
        (8, 65, 92),
        "Register new account".to_string(),
    );

    let mut email_login_field = InputField {
        x: 860,
        y: 420,
        w: 200,
        h: 50,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
        is_password: false,
    };
    let mut password_login_field = InputField {
        x: 860,
        y: 520,
        w: 200,
        h: 50,
        text: String::new(),
        color: (204, 41, 54),
        pressed: false,
        is_password: true,
    };
    let border_on_login = Border::new(840, 400, 240, 350, 10, (8, 65, 92), (255, 255, 255));
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
                            email_login_field.text = "".to_string();
                            password_login_field.text = "".to_string();
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
                            email_register_field.text = "".to_string();
                            password_register_field.text = "".to_string();
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
                    View::MainScreen => {
                        if point_in_button(x, y, &button_logout) {
                            println!("Goodbye");
                            current_view = View::Login;
                        }
                        if point_in_burger_menu(x, y, &burger_menu) {
                            if current_burger_view == Showburgermenu::Show {
                                current_burger_view = Showburgermenu::Noshow;
                            } else {
                                current_burger_view = Showburgermenu::Show;
                            }
                        }
                    }
                },
                Event::TextInput { text, .. } => match current_view {
                    View::Login => {
                        if email_login_field.pressed {
                            email_login_field.text.push_str(&text);
                        } else if password_login_field.pressed {
                            password_login_field.text.push_str(&text);
                        }
                    }
                    View::Register => {
                        if email_register_field.pressed {
                            email_register_field.text.push_str(&text);
                        } else if password_register_field.pressed {
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
                        } else if password_login_field.pressed {
                            password_login_field.text.pop();
                        }
                    }
                    View::Register => {
                        if email_register_field.pressed {
                            email_register_field.text.pop();
                        } else if password_register_field.pressed {
                            password_register_field.text.pop();
                        }
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => match current_view {
                    View::Login => {
                        if email_login_field.pressed {
                            email_login_field.pressed = false;
                            password_login_field.pressed = true;
                        } else if password_login_field.pressed {
                            password_login_field.pressed = false;
                            email_login_field.pressed = true;
                        } else {
                            email_login_field.pressed = true;
                            password_login_field.pressed = false;
                        }
                    }
                    View::Register => {
                        if email_register_field.pressed {
                            email_register_field.pressed = false;
                            password_register_field.pressed = true;
                        } else if password_register_field.pressed {
                            password_register_field.pressed = false;
                            email_register_field.pressed = true;
                        } else {
                            email_register_field.pressed = true;
                            password_register_field.pressed = false;
                        }
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => match current_view {
                    View::Login => {
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
                    }
                    View::Register => {
                        let email = email_register_field.text.clone();
                        let password = password_register_field.text.clone();
                        let (tx, rx) = channel();
                        register_result_rx = Some(rx);
                        thread::spawn(move || {
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            let result =
                                rt.block_on(send_to_server::send_register_data(email, password));
                            let _ = tx.send(result.is_ok());
                        });
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
                border_on_login.draw(&mut canvas);
                button_login.draw_with_text(&mut canvas, &font, &texture_creator);
                button_change_to_register.draw_with_text(&mut canvas, &font, &texture_creator);
                email_login_field.draw_with_text(
                    &mut canvas,
                    &ttf_context,
                    font_path,
                    &mut font_size,
                    &texture_creator,
                );
                password_login_field.draw_with_text(
                    &mut canvas,
                    &ttf_context,
                    font_path,
                    &mut font_size,
                    &texture_creator,
                );
                welcome_text_login_screen.draw_with_text(&mut canvas, &font, &texture_creator);
            }
            View::Register => {
                border_on_login.draw(&mut canvas);
                button_register.draw_with_text(&mut canvas, &font, &texture_creator);
                button_change_to_login.draw_with_text(&mut canvas, &font, &texture_creator);
                email_register_field.draw_with_text(
                    &mut canvas,
                    &ttf_context,
                    font_path,
                    &mut font_size,
                    &texture_creator,
                );
                welcome_text_register_screen.draw_with_text(&mut canvas, &font, &texture_creator);
                password_register_field.draw_with_text(
                    &mut canvas,
                    &ttf_context,
                    font_path,
                    &mut font_size,
                    &texture_creator,
                );
            }
            View::MainScreen => {
                welcome_text.draw_with_text(&mut canvas, &font, &texture_creator);
                button_logout.draw_with_text(&mut canvas, &font, &texture_creator);
                burger_menu.draw_with_texture(&mut canvas, &texture);
                if let Showburgermenu::Show = current_burger_view {
                    burger_menu.draw_all(&mut canvas, &font, &texture_creator);
                }
            }
        }

        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}
