use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Erstellt eine Event-Schleife
    let event_loop = EventLoop::new();
    
    // Erstellt ein Fenster
    let window = WindowBuilder::new()
        .with_title("Mein 3D-Fenster")
        .build(&event_loop)
        .unwrap();
    
    // Event-Schleife für das Fenster
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });
}
