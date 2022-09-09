use captcha_a::{CaptchaBuilder, Font};

fn main() {
    let fonts = vec![
        Font::try_from_bytes(include_bytes!("../fonts/captcha0.ttf")).unwrap(),
        Font::try_from_bytes(include_bytes!("../fonts/captcha1.ttf")).unwrap(),
        Font::try_from_bytes(include_bytes!("../fonts/captcha2.ttf")).unwrap(),
        Font::try_from_bytes(include_bytes!("../fonts/captcha3.ttf")).unwrap(),
        Font::try_from_bytes(include_bytes!("../fonts/captcha4.ttf")).unwrap(),
        Font::try_from_bytes(include_bytes!("../fonts/captcha5.ttf")).unwrap(),
    ];
    let captcha = CaptchaBuilder {
        //custom attribute
        width: 120,
        height: 40,
        length: 4,
        fonts: &fonts,
        //default attribute
        ..Default::default()
    };
    for i in 0..6 {
        let save_path = format!("image_{}.png", i);
        let phrase = captcha.save(&save_path).unwrap();
        println!("[{}]phrase={}", i, phrase);
    }
}
