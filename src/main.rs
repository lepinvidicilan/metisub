use glium::{buffer::Mapping, uniform, GlObject, Surface, Vertex};

use std::{
    f32::consts::PI,
    fs::{self},
    io::Read,
    path::Path,
    time::Instant,
    vec,
};

mod objects;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop failed building");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("I'm a pear")
        .with_inner_size(1920, 10)
        .build(&event_loop);

    let shape = vec![
        objects::Vertex::new([1.0, 0.0, 0.0], [0.0, 0.0]),
        objects::Vertex::new([0.0, 1.0, 0.0], [0.0, 0.0]),
        objects::Vertex::new([-1.0, 1.0, 0.0], [0.0, 0.0]),
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    println!("{}", &vertex_buffer.get_id());
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let basic_frag_shader_path = Path::new("./shaders/fragment_shaders/basic.frag");
    let basic_vert_shader_path = Path::new("./shaders/vertex_shaders/basic.vert");

    let mut basic_frag_shader = Default::default();
    let mut basic_vert_shader = Default::default();

    //find files.
    let mut f = match fs::File::open(basic_vert_shader_path) {
        Ok(f) => f,
        Err(why) => {
            panic!("can't open vertex shader: {why}")
        }
    };
    f.read_to_string(&mut basic_vert_shader)
        .expect("Can't read string");

    let mut f = match fs::File::open(basic_frag_shader_path) {
        Ok(f) => f,
        Err(why) => {
            panic!("Can't open fragment shader : {why}")
        }
    };
    f.read_to_string(&mut basic_frag_shader)
        .expect("Can't read string");
    let program =
        glium::Program::from_source(&display, &basic_vert_shader, &basic_frag_shader, None)
            .unwrap();

    let start: Instant = Instant::now();

    let mut sound_played = 0;

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let uniforms = uniform! {

                        u_time : start.elapsed().as_secs_f32(),
                    };

                    if start.elapsed().as_secs_f32() > PI + 1.0 {
                        if sound_played == 0 {
                            sound_played = 1;
                            /* for i in vertex_buffer.map().len() {
                                vertex_buffer.map() =
                                    reverse_vertex_buffer(vertex_buffer.borrow().map());
                            }*/
                        }
                    }

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap()
                }
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                    let uniforms = uniform! {
                        u_resolution: [window_size.width as f32, window_size.height as f32]
                    };
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap()
                }
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
fn reverse_vertex_buffer(list: Mapping<'_, [Vertex]>) -> Mapping<'_, [Vertex]> {
    let mut a = vec![];
    let mut b = list;
    println!("{}", b.len());
    if b.len() != 0 {
        for i in 0..b.len() {
            a.push(b[i]);
        }
        for i in 0..b.len() {
            b[i] = a[b.len() - 1 - i];
        }
    }
    return b;
}
