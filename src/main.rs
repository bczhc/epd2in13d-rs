use epd2in13d_rs::dev;
use epd2in13d_rs::epd2in13d::Epd2in13d;
use epd2in13d_rs::ffi::{EPD_2IN13D_HEIGHT, EPD_2IN13D_WIDTH};

fn main() {
    dev::module_init();

    let epd2in13d = Epd2in13d;
    epd2in13d.init();
    epd2in13d.clear();
    dev::delay_ms(500.0);

    let mut image_buffer = vec![0_u8; EPD_2IN13D_WIDTH / 8 * EPD_2IN13D_HEIGHT];
    let row_bytes = EPD_2IN13D_WIDTH / 8;
    for i in 0..EPD_2IN13D_HEIGHT {
        let byte_filled = match i % 2 {
            0 => 0b01010101_u8,
            1 => 0b10101010_u8,
            _ => unreachable!(),
        };
        image_buffer[(row_bytes * i)..(row_bytes * i + row_bytes)].fill(byte_filled);
    }

    epd2in13d.display(&image_buffer);
    epd2in13d.sleep();
}
