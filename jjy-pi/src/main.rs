use std::thread;

use chrono::{NaiveDateTime, Utc};
use embedded_graphics::{
    draw_target::DrawTarget as _,
    geometry::Point,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    primitives::{Circle, Primitive, PrimitiveStyle},
    text::Text,
    Drawable as _,
};
use jjy_core::{get_next_second, signal::Signal};
use rppal::{
    gpio::{Gpio, OutputPin},
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig as _},
    prelude::SPIInterface,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    Ssd1306,
};

fn main() -> ! {
    let tz = chrono_tz::Asia::Tokyo;
    let gpio = Gpio::new().expect("failed to initialize GPIO");
    let mut pin = gpio.get(4).expect("failed to get GPIO 4").into_output();
    pin.set_low();

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 200_000_000, Mode::Mode0)
        .expect("failed to initialize SPI");

    let dc = gpio.get(9).expect("failed to get GPIO 9").into_output();
    let cs = gpio.get(8).expect("failed to get GPIO 8").into_output();

    let interface = SPIInterface::new(spi, dc, cs);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().expect("failed to initialize display");
    display.clear(BinaryColor::Off).ok();

    loop {
        let now = Utc::now().with_timezone(&tz).naive_local();
        let next_second = get_next_second(now);
        let signal = Signal::from(next_second);
        thread::sleep((next_second - now).to_std().unwrap());
        pin.set_high();

        display.clear(BinaryColor::Off).ok();
        draw_date(&mut display, &next_second, &signal);
        draw_bit(&mut display, true);

        // NOTE: 仕様上5ms以内の誤差であれば許容される（そして描画はだいたい1ms程度）が、念のため正確な残り時間を再計算する
        let now = Utc::now().with_timezone(&tz).naive_local();
        let duration = (next_second + signal.signal_value.to_duration() - now)
            .to_std()
            .unwrap();
        thread::sleep(duration);
        pin.set_low();

        draw_bit(&mut display, false);
    }
}

fn draw_date(
    display: &mut Ssd1306<
        SPIInterface<Spi, OutputPin, OutputPin>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
    date: &NaiveDateTime,
    signal: &Signal,
) {
    let char_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    Text::new(
        &date.format("%Y-%m-%d").to_string(),
        Point::new(14, 18),
        char_style,
    )
    .draw(display)
    .ok();
    Text::new(
        &date.format("%H:%M:%S").to_string(),
        Point::new(24, 38),
        char_style,
    )
    .draw(display)
    .ok();
    Text::new(
        &format!("{:>4}:{}", signal.signal_type, signal.signal_value),
        Point::new(24, 58),
        char_style,
    )
    .draw(display)
    .ok();

    display.flush().ok();
}

fn draw_bit(
    display: &mut Ssd1306<
        SPIInterface<Spi, OutputPin, OutputPin>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
    on: bool,
) {
    Circle::new(Point::new(120, 52), 8)
        .into_styled(PrimitiveStyle::with_fill(if on {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }))
        .draw(display)
        .ok();
    display.flush().ok();
}
