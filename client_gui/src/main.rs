mod helpers;
mod send_to_server;
mod structs;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;
use structs::Border;
use structs::Button;
use structs::InputField;
use structs::Menuburger;
use structs::Subburgerbuttons;
use structs::Text;
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
    burger_menu.populate(vec![vec![
        Subburgerbuttons::new((255, 255, 255), "Add portfolio".to_string()),
        Subburgerbuttons::new((255, 255, 255), "Remove portfolio".to_string()),
    ]]);
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
                        if helpers::point_in_button(x, y, &button_login) {
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
                        } else if helpers::point_in_button(x, y, &button_change_to_register) {
                            current_view = View::Register;
                        } else if helpers::point_in_input_field(x, y, &email_login_field) {
                            email_login_field.pressed = true;
                            password_login_field.pressed = false;
                        } else if helpers::point_in_input_field(x, y, &password_login_field) {
                            password_login_field.pressed = true;
                            email_login_field.pressed = false;
                        }
                    }
                    View::Register => {
                        if helpers::point_in_button(x, y, &button_register) {
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
                        } else if helpers::point_in_button(x, y, &button_change_to_login) {
                            current_view = View::Login;
                        } else if helpers::point_in_input_field(x, y, &email_register_field) {
                            email_register_field.pressed = true;
                            password_register_field.pressed = false;
                        } else if helpers::point_in_input_field(x, y, &password_register_field) {
                            password_register_field.pressed = true;
                            email_register_field.pressed = false;
                        }
                    }
                    View::MainScreen => {
                        if helpers::point_in_button(x, y, &button_logout) {
                            println!("Goodbye");
                            current_view = View::Login;
                        }
                        if helpers::point_in_burger_menu(x, y, &burger_menu) {
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
        helpers::bg_color(&mut canvas, vec![8, 65, 92]);
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
