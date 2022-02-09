use pixels::{Error, Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::cpu;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub fn run_main_loop() -> Result<(), Error>{
    // Initialize CPU
    let mut cpu = cpu::init();
    // Load ROM from file
    //cpu.load_rom("C:/test/test-rom.ch8".to_string());
    cpu.load_rom("C:/test/c8_test.c8".to_string());


    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("CHIP-8 Emulator :D")
        .build(&event_loop)
        .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            for _ in 1..2{
                cpu.fetch_instruction();
            }
            draw(pixels.get_frame(), cpu.frame_buffer);
            if pixels
                .render()
                .map_err(|e| eprint!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Request a redraw
            window.request_redraw();
        }
    });
}
fn draw(frame: &mut [u8], frame_buffer: [u8; 64*32]){
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let mut val = frame_buffer[i];
        if val > 0{
            val = 0xff;
        }
        let rgba = [val, val, val, 0xff];

        pixel.copy_from_slice(&rgba);
    }
}