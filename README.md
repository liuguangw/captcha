# captcha-a
a captcha library for rust

![image_0](captcha_examples/images/image_0.png) | ![image_1](captcha_examples/images/image_1.png) | ![image_2](captcha_examples/images/image_2.png)
--- | --- | ---
![image_3](captcha_examples/images/image_3.png) | ![image_4](captcha_examples/images/image_4.png) | ![image_5](captcha_examples/images/image_5.png)

## code example

```rust
use captcha_a::{CaptchaBuilder, Font};
fn main() {
    let captcha = CaptchaBuilder {
        //custom attribute
        width: 120,
        height: 40,
        length: 4,
        fonts: vec![
            Font::try_from_bytes(include_bytes!("../fonts/captcha0.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../fonts/captcha1.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../fonts/captcha2.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../fonts/captcha3.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../fonts/captcha4.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../fonts/captcha5.ttf")).unwrap(),
        ],
        //default attribute
        ..Default::default()
    };
    for i in 0..6 {
        let save_path = format!("image_{}.png", i);
        let phrase = captcha.save(&save_path).unwrap();
        println!("[{}]phrase={}", i, phrase);
    }
}
```

