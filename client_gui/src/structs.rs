use crate::helpers;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
pub struct Border {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub border_thick: u32,
    pub body_color: (u8, u8, u8),
    pub border_color: (u8, u8, u8),
}
pub struct Subburgerbuttons {
    pub bg_color: (u8, u8, u8),
    pub font_color: (u8, u8, u8),
    pub text: String,
}
pub struct Menuburger {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub _texture: sdl2::image::Sdl2ImageContext,
    pub fields: Vec<Subburgerbuttons>,
}
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub color_bg: (u8, u8, u8),
    pub color_fg: (u8, u8, u8),
    pub text: String,
}

pub struct Text {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub text: String,
    pub color: (u8, u8, u8),
}

#[derive(Clone)]
pub struct InputField {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub text: String,
    pub color: (u8, u8, u8),
    pub pressed: bool,
    pub is_password: bool,
}
impl Border {
    pub fn new(
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
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
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
    pub fn new(bg_color: (u8, u8, u8), font_color: (u8, u8, u8), text: String) -> Self {
        Subburgerbuttons {
            bg_color: bg_color,
            font_color: font_color,
            text: text,
        }
    }
}
impl Menuburger {
    pub fn new(x: i32, y: i32, w: u32, h: u32, texture: sdl2::image::Sdl2ImageContext) -> Self {
        Menuburger {
            x: (x),
            y: (y),
            w: (w),
            h: (h),
            _texture: (texture),
            fields: Vec::new(),
        }
    }
    pub fn draw_with_texture(&self, canvas: &mut Canvas<Window>, texture: &Texture) {
        let target = Rect::new(self.x, self.y, self.w, self.h);
        canvas.copy(texture, None, Some(target)).unwrap();
    }
    pub fn populate(&mut self, subfields: Vec<Vec<Subburgerbuttons>>) {
        self.fields.extend(subfields.into_iter().flatten());
    }
    pub fn draw_all(
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
    pub fn new(
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

    pub fn draw_with_text(
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
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: (u8, u8, u8), text: String) -> Self {
        Text {
            x,
            y,
            w,
            h,
            color,
            text,
        }
    }

    pub fn draw_with_text(
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
    pub fn draw_with_text(
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
            let (font, fitted_size) = helpers::find_fitting_font(
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
