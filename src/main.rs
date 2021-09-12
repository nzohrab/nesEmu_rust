mod bit_helpers;
mod bits;
extern crate minifb;
extern crate time;
mod bus;
mod chip;
mod instructions;
mod ppu;
mod cartridge;
mod mappers;
use stick::{Event, Hub, Pad};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn main() {
    let mut cart = cartridge::Cartridge::new("roms/dk.nes".into()).unwrap();
    let mut cpu = chip::Chip::new();
    let mut ppu = ppu::PPU::new();


    let mut data_bus: bus::Bus = bus::Bus::new(&mut cart, &mut ppu, &mut cpu);
    ppu.cart = &mut cart;
    cpu.data_bus = &mut data_bus;

    data_bus.reset();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    //window.limit_update_rate(Some(std::time::Duration::from_micros(2666)));


    while true{
        let now = time::Instant::now();

        while !ppu.frame_complete{
            data_bus.clock();
        }
        let elapsed = now.elapsed();
        println!("FPS: {}", 1.0/elapsed.as_seconds_f64());

        //println!("{}", cpu.clock_count);

        data_bus.controllers[0] = 0x00;
        data_bus.controllers[0] |= if window.is_key_down(Key::X) {0x80} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::Z) {0x40} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::A) {0x20} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::S) {0x10} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::Up) {0x08} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::Down) {0x04} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::Left) {0x02} else {0x00};
        data_bus.controllers[0] |= if window.is_key_down(Key::Right) {0x01} else {0x00}; 
   
        //println!("{:010b}",data_bus.controllers[0]);
        window.update_with_buffer(&ppu.scr_buf, WIDTH, HEIGHT).unwrap();
        ppu.frame_complete = false;
    }
}