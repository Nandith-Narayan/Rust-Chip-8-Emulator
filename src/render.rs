use pixels::{Error, Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::cpu;
use crate::cpu::KeyState;


const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub fn run_main_loop() -> Result<(), Error>{
    // Initialize CPU
    let mut cpu = cpu::init();
    // Load ROM from file
    cpu.load_rom("C:/Users/nandi/Downloads/Pong (alt).ch8".to_string());
    //cpu.load_rom("C:/test/Particle Demo [zeroZshadow, 2008].ch8".to_string());
    //cpu.load_rom("C:/test/Maze [David Winter, 199x].ch8".to_string());
    //cpu.load_rom("C:/test/c8_test.c8".to_string());


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


        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key1){
                cpu.key_states[1] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key2){
                cpu.key_states[2] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key3){
                cpu.key_states[3] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key4){
                cpu.key_states[0xC] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Q){
                cpu.key_states[0x4] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::W){
                cpu.key_states[0x5] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::E){
                cpu.key_states[0x6] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::R){
                cpu.key_states[0xD] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::A){
                cpu.key_states[0x7] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::S){
                cpu.key_states[0x8] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::D){
                cpu.key_states[0x9] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::F){
                cpu.key_states[0xE] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Z){
                cpu.key_states[0xA] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::X){
                cpu.key_states[0x0] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::C){
                cpu.key_states[0xB] = KeyState::Pressed;
                return;
            }
            if input.key_pressed(VirtualKeyCode::V){
                cpu.key_states[0xF] = KeyState::Pressed;
                return;
            }

            if input.key_released(VirtualKeyCode::Key1){
                cpu.key_states[1] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::Key2){
                cpu.key_states[2] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::Key3){
                cpu.key_states[3] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::Key4){
                cpu.key_states[0xC] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::Q){
                cpu.key_states[0x4] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::W){
                cpu.key_states[0x5] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::E){
                cpu.key_states[0x6] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::R){
                cpu.key_states[0xD] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::A){
                cpu.key_states[0x7] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::S){
                cpu.key_states[0x8] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::D){
                cpu.key_states[0x9] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::F){
                cpu.key_states[0xE] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::Z){
                cpu.key_states[0xA] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::X){
                cpu.key_states[0x0] = KeyState::NotPressed;

                return;
            }
            if input.key_released(VirtualKeyCode::C){
                cpu.key_states[0xB] = KeyState::NotPressed;
                return;
            }
            if input.key_released(VirtualKeyCode::V){
                cpu.key_states[0xF] = KeyState::NotPressed;
                return;
            }
            
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                window.request_redraw();
            }
            return;

        }
        if let Event::RedrawRequested(_) = event {
            for _ in 1..10{
                cpu.fetch_instruction();
            }
            if cpu.delay_timer > 0{
                cpu.delay_timer -= 1;
            }
            if cpu.key_states[0] == KeyState::Pressed {
                println!("YES1");
            }
            draw(pixels.get_frame(), cpu.frame_buffer);
            if pixels
                .render()
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

        }
        if let Event::RedrawEventsCleared = event{
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