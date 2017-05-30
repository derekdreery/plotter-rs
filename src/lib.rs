#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate itertools;

mod util;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            pos: [0.0; 2],
            color: [0.0; 3],
        }
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];


#[derive(Debug, Clone)]
pub enum Error {
    ArgumentError(String)
}

const TRIANGLE: [Vertex; 4] = [
    Vertex { pos: [ -1.0, -1.0 ], color: WHITE },
    Vertex { pos: [ -1.0,  1.0 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  1.0, -1.0 ], color: WHITE },
    Vertex { pos: [  1.0,  1.0 ], color: WHITE }
];

const TRIANGLE_IDX: &'static [u16] = &[
    0, 1, 2,
    1, 2, 3,
];

/// Plot a line graph and block until the window is colsed
pub fn plot_linegraph<A>(points: A) -> Result<(), Error>
    where A: AsRef<[(f32, f32)]>
{
    for pair in points.as_ref().iter() {
        println!("{:?}", pair);
    }
    println!("{:?}", util::triangulate_lines(points.as_ref(), 0.01));

    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Line graph".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        pipe::new()
    ).unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, TRIANGLE_IDX);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color
    };

    let mut running = true;
    while running {
        // fetch events
        events_loop.poll_events(|glutin::Event::WindowEvent{window_id: _, event}| {
            match event {
                glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) |
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(_width, _height) => {
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                },
                _ => {},
            }
        });

        // draw a frame
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
