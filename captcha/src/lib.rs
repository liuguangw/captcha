//! a captcha library for rust
mod captcha;
mod captcha_builder;
pub(crate) mod color;
pub use captcha::Captcha;
pub use captcha_builder::CaptchaBuilder;
pub use image::ImageError;
pub use image::ImageResult;
pub use image::Rgba;
pub use rusttype::Font;
