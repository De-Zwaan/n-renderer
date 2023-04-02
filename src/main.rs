use std::f64::consts::PI;

// Crates for window managment
use pixels::{Error, PixelsBuilder, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

// Actual rendering code
use simple_graphics::{orbital::create_orbital_v2, pos::RotationPlane, shapes::*};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

const SCALE: f64 = 200.0;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    // Initialise the window
    let window = WindowBuilder::new()
        .with_title("Spinny Spinny")
        // .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Create a surface texture to render to
    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        &window,
    );

    // Create a pixelarray
    let mut pixels: pixels::Pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture).build()?;

    // let mut t: u64 = 0;

    // let mut shape = create_3_cube(1.0);
    // let mut shape = create_4_cube(1.0);
    // let mut shape = create_3_sphere(1000);
    // let mut shape = create_4_sphere(3200, 1.8);
    // let mut shape = create_orbital(1000, 1.0, 2.0, 0.1);
    let mut shape: Object = create_orbital_v2(50, 0.1, 5.0, 0.15, (2, 1, 0));
    // let mut shape = create_torus(100, 1.8);
    // let mut shape = empty();

    // shape.rotate(RotationPlane::get_rot_mat_4d(RotationPlane::YZ, PI / 2.0));
    shape.scale(1.0);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // println!("Window closed");
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                // println!("Window resized");
                pixels.resize_buffer(new_size.width, new_size.height);
                pixels.resize_surface(new_size.width, new_size.height);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // t += 1;

                let screen = pixels.get_frame();

                // Create an empty pixelbuffer to render to
                screen.chunks_exact_mut(4).for_each(|p| {
                    p.copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
                });

                // Transform the object
                shape.rotate(RotationPlane::get_rot_mat_4d(RotationPlane::YZ, PI / 512.0));

                // Draw the object
                shape.draw(
                    screen,
                    window.inner_size(),
                    SCALE,
                    simple_graphics::projection::Projection::Perspective,
                );

                // Display the result on the screen
                if pixels
                    .render()
                    .map_err(|e| println!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    control_flow.set_exit();
                };
            }
            _ => (),
        }
    })
}
