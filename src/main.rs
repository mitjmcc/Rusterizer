#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;

// mod vertex;
// use crate::vertex::Vertex;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

const SQUARE: [Vertex; 4] = [
    Vertex {
        pos: [0.5, -0.5, 0.0],
        color: WHITE,
    },
    Vertex {
        pos: [-0.5, -0.5, 0.0],
        color: WHITE,
    },
    Vertex {
        pos: [-0.5, 0.5, 0.0],
        color: WHITE,
    },
    Vertex {
        pos: [0.5, 0.5, 0.0],
        color: WHITE,
    },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

pub fn main() {
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Rusterizer".to_string())
        .with_dimensions(800, 800)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/rect_150.glslv"
            )),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/rect_150.glslf"
            )),
            pipe::new(),
        )
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, INDICES);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    let mut running = true;
    while running {
        events_loop.poll_events(
            |glutin::Event::WindowEvent {
                 window_id: _,
                 event,
             }| {
                use glutin::WindowEvent::*;
                match event {
                    KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) | Closed => {
                        running = false
                    }
                    Resized(_, _) => {
                        gfx_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    }
                    _ => (),
                }
            },
        );

        encoder.clear(&data.out, BLACK);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
