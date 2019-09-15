extern crate gl;

use winit::{Event, WindowEvent};

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new().with_title("Fractals").build(&events_loop).unwrap();
    println!("Hello, world!");

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => running = false,
                _ => ()
            };
        });
    }
}
