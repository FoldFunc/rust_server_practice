use crate::structs::Button;
use crate::structs::InputField;
use crate::structs::Menuburger;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;

pub fn point_in_button(x: i32, y: i32, button: &Button) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}
pub fn point_in_burger_menu(x: i32, y: i32, button: &Menuburger) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}

pub fn point_in_input_field(x: i32, y: i32, field: &InputField) -> bool {
    x >= field.x && x <= field.x + field.w as i32 && y >= field.y && y <= field.y + field.h as i32
}

pub fn bg_color(canvas: &mut Canvas<Window>, colors: Vec<u8>) {
    canvas.set_draw_color(Color::RGB(colors[0], colors[1], colors[2]));
    canvas.clear();
}

pub fn find_fitting_font<'a>(
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
