mod as_bytes;
mod keyboard_state;
mod mouse_state;
mod window_state;

use as_bytes::*;

use gl_typed as gl;
use std::convert::TryInto;

use bvh::vector::*;

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

pub type Triangle = [u32; 3];

#[derive(Debug)]
pub struct Scene {
    pub pos_in_obj_buffer: Vec<[f32; 3]>,
    pub triangle_buffer: Vec<[u32; 3]>,
    pub mesh_descriptions: Vec<MeshDescription>,
}

const RGBA_PALETTE: [[f32; 4]; 32] = [
    [0.329, 0.588, 0.208, 1.000],
    [0.337, 0.475, 0.910, 1.000],
    [0.875, 0.729, 0.180, 1.000],
    [0.522, 0.459, 0.859, 1.000],
    [0.659, 0.792, 0.314, 1.000],
    [0.867, 0.443, 0.820, 1.000],
    [0.424, 0.859, 0.522, 1.000],
    [0.757, 0.310, 0.635, 1.000],
    [0.357, 0.827, 0.592, 1.000],
    [0.863, 0.294, 0.545, 1.000],
    [0.294, 0.816, 0.678, 1.000],
    [0.867, 0.329, 0.294, 1.000],
    [0.231, 0.902, 0.918, 1.000],
    [0.843, 0.365, 0.396, 1.000],
    [0.306, 0.584, 0.325, 1.000],
    [0.663, 0.443, 0.808, 1.000],
    [0.808, 0.698, 0.243, 1.000],
    [0.294, 0.604, 0.894, 1.000],
    [0.824, 0.533, 0.149, 1.000],
    [0.439, 0.510, 0.812, 1.000],
    [0.745, 0.761, 0.361, 1.000],
    [0.733, 0.510, 0.804, 1.000],
    [0.675, 0.808, 0.478, 1.000],
    [0.824, 0.486, 0.741, 1.000],
    [0.518, 0.506, 0.176, 1.000],
    [0.812, 0.431, 0.612, 1.000],
    [0.835, 0.659, 0.322, 1.000],
    [0.835, 0.373, 0.486, 1.000],
    [0.784, 0.588, 0.341, 1.000],
    [0.824, 0.455, 0.357, 1.000],
    [0.757, 0.435, 0.165, 1.000],
    [0.835, 0.451, 0.286, 1.000],
];

impl Scene {
    pub fn from_meshes(meshes: &[Mesh]) -> Self {
        let mut pos_in_obj_buffer = Vec::new();
        let mut triangle_buffer = Vec::new();

        let mesh_descriptions = meshes
            .iter()
            .map(|mesh| MeshDescription {
                vertex_offset: {
                    let offset = pos_in_obj_buffer.len().try_into().unwrap();
                    pos_in_obj_buffer.extend(
                        mesh.vertices
                            .iter()
                            .map(|&vertex| -> [f32; 3] { vertex.into() }),
                    );
                    offset
                },
                vertex_count: mesh.vertices.len().try_into().unwrap(),
                triangle_offset: {
                    let offset = triangle_buffer.len().try_into().unwrap();
                    triangle_buffer.extend(&mesh.triangles[..]);
                    offset
                },
                triangle_count: mesh.triangles.len().try_into().unwrap(),
            })
            .collect();

        Scene {
            pos_in_obj_buffer,
            triangle_buffer,
            mesh_descriptions,
        }
    }
}

const VS_POS_IN_OBJ_LOC: gl::AttributeLocation =
    unsafe { gl::AttributeLocation::from_i32_unchecked(0) };

const VS_P0_LOC: gl::AttributeLocation = unsafe { gl::AttributeLocation::from_i32_unchecked(0) };
const VS_P1_LOC: gl::AttributeLocation = unsafe { gl::AttributeLocation::from_i32_unchecked(1) };
const VS_RGBA_LOC: gl::AttributeLocation = unsafe { gl::AttributeLocation::from_i32_unchecked(2) };

const OBJ_TO_CLP_LOC: gl::UniformLocation = unsafe { gl::UniformLocation::from_i32_unchecked(0) };
const CLP_TO_CAM_LOC: gl::UniformLocation = unsafe { gl::UniformLocation::from_i32_unchecked(1) };
const RGBA_LOC: gl::UniformLocation = unsafe { gl::UniformLocation::from_i32_unchecked(2) };

#[derive(Debug)]
pub struct MeshDescription {
    pub vertex_offset: u32,
    pub vertex_count: u32,
    pub triangle_offset: u32,
    pub triangle_count: u32,
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vector3<f32>>,
    pub triangles: Vec<Triangle>,
    pub bvh: bvh::bvh::Tree,
}

fn main() {
    let path = std::env::args_os()
        .skip(1)
        .next()
        .unwrap_or(std::ffi::OsString::from("resources/sponza/sponza.obj"));
    let (models, _materials) = tobj::load_obj(path.as_ref()).expect("Failed to load model.");

    let mesh_count = models.len();
    let meshes: Vec<Mesh> = models
        .into_iter()
        .enumerate()
        .map(|(mesh_index, tobj::Model { name, mesh })| {
            println!(
                "Loading mesh {:?} {:03}/{:03}",
                &name,
                mesh_index + 1,
                mesh_count
            );

            let vertex_count = mesh.positions.len() / 3;

            let mut vertices = Vec::with_capacity(vertex_count);
            assert_eq!(vertex_count * 3, mesh.positions.len());
            for i in 0..vertex_count {
                vertices.push(Vector3 {
                    x: mesh.positions[i * 3 + 0],
                    y: mesh.positions[i * 3 + 1],
                    z: mesh.positions[i * 3 + 2],
                });
            }

            let vertex_count_u32: u32 = vertex_count.try_into().unwrap();
            let index_count = mesh.indices.len() / 3;
            let mut triangles = Vec::with_capacity(index_count);
            assert_eq!(index_count * 3, mesh.indices.len());
            for i in 0..index_count {
                let triangle = [
                    mesh.indices[i * 3 + 0],
                    mesh.indices[i * 3 + 1],
                    mesh.indices[i * 3 + 2],
                ];

                assert!(triangle[0] < vertex_count_u32);
                assert!(triangle[1] < vertex_count_u32);
                assert!(triangle[2] < vertex_count_u32);

                triangles.push(triangle);
            }

            let bvh = bvh::bvh::Tree::new(&vertices, &triangles);
            Mesh {
                name,
                vertices,
                triangles,
                bvh,
            }
        })
        .collect();

    let scene = Scene::from_meshes(&meshes);

    let event_loop = glutin::event_loop::EventLoop::new();

    let windowed_context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 5)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl_debug_flag(cfg!(debug_assertions))
        .build_windowed(
            glutin::window::WindowBuilder::new()
                .with_title(CARGO_PKG_NAME)
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0)),
            &event_loop,
        )
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl = unsafe {
        gl::Gl::load_with(|ptr| windowed_context.context().get_proc_address(ptr) as *const _)
    };

    let program = unsafe {
        let program = gl.create_program();

        let vs = {
            let name = gl.create_shader(gl::VERTEX_SHADER);
            let source = std::fs::read("resources/basic.vert").unwrap();
            gl.shader_source(name, &[&source[..]]);
            gl.compile_shader(name);
            // println!("{}", gl.get_shader_info_log(name));
            name
        };

        let fs = {
            let name = gl.create_shader(gl::FRAGMENT_SHADER);
            let source = std::fs::read("resources/basic.frag").unwrap();
            gl.shader_source(name, &[&source[..]]);
            gl.compile_shader(name);
            // println!("{}", gl.get_shader_info_log(name));
            name
        };

        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);

        println!("Program info log:\n{}", gl.get_program_info_log(program));

        program
    };

    let boxes_program = unsafe {
        let program = gl.create_program();

        let vs = {
            let name = gl.create_shader(gl::VERTEX_SHADER);
            let source = std::fs::read("resources/boxes.vert").unwrap();
            gl.shader_source(name, &[&source[..]]);
            gl.compile_shader(name);
            name
        };

        let ge = {
            let name = gl.create_shader(gl::GEOMETRY_SHADER);
            let source = std::fs::read("resources/boxes.geom").unwrap();
            gl.shader_source(name, &[&source[..]]);
            gl.compile_shader(name);
            name
        };

        let fs = {
            let name = gl.create_shader(gl::FRAGMENT_SHADER);
            let source = std::fs::read("resources/boxes.frag").unwrap();
            gl.shader_source(name, &[&source[..]]);
            gl.compile_shader(name);
            name
        };

        gl.attach_shader(program, vs);
        gl.attach_shader(program, ge);
        gl.attach_shader(program, fs);
        gl.link_program(program);

        println!("Program info log:\n{}", gl.get_program_info_log(program));

        program
    };

    let (vao, _vb, _eb) = unsafe {
        let vao = gl.create_vertex_array();
        let vb = gl.create_buffer();
        let eb = gl.create_buffer();

        gl.named_buffer_data(vb, scene.pos_in_obj_buffer.vec_as_bytes(), gl::STATIC_DRAW);
        gl.named_buffer_data(eb, scene.triangle_buffer.vec_as_bytes(), gl::STATIC_DRAW);

        const BBI_00: gl::VertexArrayBufferBindingIndex =
            gl::VertexArrayBufferBindingIndex::from_u32(0);

        gl.enable_vertex_array_attrib(vao, VS_POS_IN_OBJ_LOC);
        gl.vertex_array_attrib_format(vao, VS_POS_IN_OBJ_LOC, 3, gl::FLOAT, false, 0);
        gl.vertex_array_attrib_binding(vao, VS_POS_IN_OBJ_LOC, BBI_00);
        gl.vertex_array_vertex_buffer(vao, BBI_00, vb, 0, std::mem::size_of::<[f32; 3]>() as u32);

        gl.vertex_array_element_buffer(vao, eb);

        (vao, vb, eb)
    };

    let (boxes_vao, _vb, _eb, mesh_node_descriptions) = unsafe {
        let vao = gl.create_vertex_array();
        let vb = gl.create_buffer();
        let eb = gl.create_buffer();

        #[repr(C)]
        struct Vertex {
            p0: [f32; 3],
            p1: [f32; 3],
            rgba: [f32; 4],
        }

        let mut color_index = 0;

        let (vertex_buffer, mesh_node_descriptions) = meshes.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut vertex_buffer, mut mesh_node_descriptions), mesh| {
                let rgba = RGBA_PALETTE[color_index];
                color_index = (color_index + 1) % RGBA_PALETTE.len();
                let node_descriptions: Vec<u32> = mesh
                    .bvh
                    .nodes
                    .iter()
                    .map(|node| {
                        let vertex_index: u32 = vertex_buffer.len().try_into().unwrap();
                        vertex_buffer.push(Vertex {
                            p0: node.min.into(),
                            p1: node.max.into(),
                            rgba,
                        });
                        vertex_index
                    })
                    .collect();
                mesh_node_descriptions.push(node_descriptions);
                (vertex_buffer, mesh_node_descriptions)
            },
        );

        let point_buffer: Vec<u32> = (0u32..vertex_buffer.len().try_into().unwrap()).collect();

        gl.named_buffer_data(vb, vertex_buffer.vec_as_bytes(), gl::STATIC_DRAW);
        gl.named_buffer_data(eb, point_buffer.vec_as_bytes(), gl::STATIC_DRAW);

        const BBI_00: gl::VertexArrayBufferBindingIndex =
            gl::VertexArrayBufferBindingIndex::from_u32(0);

        gl.enable_vertex_array_attrib(vao, VS_P0_LOC);
        gl.vertex_array_attrib_format(vao, VS_P0_LOC, 3, gl::FLOAT, false, 0);
        gl.vertex_array_attrib_binding(vao, VS_P0_LOC, BBI_00);

        gl.enable_vertex_array_attrib(vao, VS_P1_LOC);
        gl.vertex_array_attrib_format(
            vao,
            VS_P1_LOC,
            3,
            gl::FLOAT,
            false,
            std::mem::size_of::<[f32; 3]>() as u32,
        );
        gl.vertex_array_attrib_binding(vao, VS_P1_LOC, BBI_00);

        gl.enable_vertex_array_attrib(vao, VS_RGBA_LOC);
        gl.vertex_array_attrib_format(
            vao,
            VS_RGBA_LOC,
            4,
            gl::FLOAT,
            false,
            std::mem::size_of::<[f32; 6]>() as u32,
        );
        gl.vertex_array_attrib_binding(vao, VS_RGBA_LOC, BBI_00);

        gl.vertex_array_vertex_buffer(vao, BBI_00, vb, 0, std::mem::size_of::<Vertex>() as u32);
        gl.vertex_array_element_buffer(vao, eb);

        (vao, vb, eb, mesh_node_descriptions)
    };

    let start = std::time::Instant::now();
    let mut keyboard_state = keyboard_state::KeyboardState::default();
    let mut window_state = window_state::WindowState::default();
    let mut mouse_state = mouse_state::MouseState::default();
    let mut camera = bvh::camera::Camera {
        transform: bvh::camera::CameraTransform {
            position: cgmath::Point3::new(0.0, 0.0, 5.0),
            yaw: cgmath::Deg(0.0).into(),
            pitch: cgmath::Deg(0.0).into(),
            fovy: cgmath::Deg(90.0).into(),
        },
        properties: bvh::camera::CameraProperties {
            z0: -200.0,
            z1: -0.5,
            positional_velocity: 0.2,
            angular_velocity: 0.4,
            zoom_velocity: 0.5,
        },
    };
    let mut focus_camera = false;
    let mut current_mesh = 0;
    let mut current_depth = 0;

    event_loop.run(move |event, _, control_flow| {
        use glutin::event::*;
        use glutin::event_loop::*;

        *control_flow = ControlFlow::Poll;

        let elapsed = (start.elapsed().as_micros() as f32) / 1_000_000.0;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    window_state.dimensions[0] = physical_size.width;
                    window_state.dimensions[1] = physical_size.height;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    match (input.virtual_keycode, input.state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        (Some(VirtualKeyCode::Space), ElementState::Pressed) => {
                            focus_camera = !focus_camera;
                        }
                        _ => {}
                    }

                    if input.state == ElementState::Pressed {
                        match input.scancode {
                            2 => {
                                current_mesh = match keyboard_state.lshift {
                                    ElementState::Released => {
                                        if current_mesh == meshes.len() - 1 {
                                            0
                                        } else {
                                            current_mesh + 1
                                        }
                                    }
                                    ElementState::Pressed => {
                                        if current_mesh == 0 {
                                            meshes.len() - 1
                                        } else {
                                            current_mesh - 1
                                        }
                                    }
                                };
                            }
                            3 => {
                                current_depth = match keyboard_state.lshift {
                                    ElementState::Released => current_depth + 1,
                                    ElementState::Pressed => {
                                        if current_depth == 0 {
                                            0
                                        } else {
                                            current_depth - 1
                                        }
                                    }
                                };
                            }
                            _ => {}
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    mouse_state.x = position.x as f64 / window_state.dimensions[0] as f64;
                    mouse_state.y = 1.0 - position.y as f64 / window_state.dimensions[1] as f64;
                }
                WindowEvent::Focused(focus) => window_state.focus = focus,
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(keyboard_input) => keyboard_state.update(keyboard_input),
                DeviceEvent::Motion { axis, value } => mouse_state.update_motion(axis, value),
                _ => {}
            },
            Event::MainEventsCleared => {
                // Simulate.
                fn key_delta(
                    n: glutin::event::ElementState,
                    p: glutin::event::ElementState,
                    amp: f32,
                ) -> f32 {
                    match (n, p) {
                        (
                            glutin::event::ElementState::Released,
                            glutin::event::ElementState::Released,
                        ) => 0.0,
                        (
                            glutin::event::ElementState::Released,
                            glutin::event::ElementState::Pressed,
                        ) => amp,
                        (
                            glutin::event::ElementState::Pressed,
                            glutin::event::ElementState::Released,
                        ) => -amp,
                        (
                            glutin::event::ElementState::Pressed,
                            glutin::event::ElementState::Pressed,
                        ) => 0.0,
                    }
                }

                let delta = bvh::camera::CameraDelta {
                    time: 1.0 / 60.0,
                    position: if window_state.focus {
                        let amp = match keyboard_state.lshift {
                            glutin::event::ElementState::Released => 1.0,
                            glutin::event::ElementState::Pressed => 4.0,
                        };
                        cgmath::Vector3 {
                            x: key_delta(keyboard_state.a, keyboard_state.d, amp),
                            y: key_delta(keyboard_state.z, keyboard_state.q, amp),
                            z: key_delta(keyboard_state.w, keyboard_state.s, amp),
                        }
                    } else {
                        use cgmath::prelude::*;
                        cgmath::Vector3::zero()
                    },
                    yaw: cgmath::Rad(if window_state.focus && focus_camera {
                        -mouse_state.dx as f32
                    } else {
                        0.0
                    }),
                    pitch: cgmath::Rad(if window_state.focus && focus_camera {
                        -mouse_state.dy as f32
                    } else {
                        0.0
                    }),
                    fovy: cgmath::Rad(if window_state.focus && focus_camera {
                        mouse_state.dscroll as f32
                    } else {
                        0.0
                    }),
                };
                camera.update(&delta);

                mouse_state.clear_motion();

                // Render.
                let frustum = {
                    use cgmath::*;
                    let dimensions = Vector2::from(window_state.dimensions)
                        .cast::<f64>()
                        .unwrap();
                    let dy = (Rad::from(camera.transform.fovy).0 as f64 * 0.5).tan();
                    let dx = dy * dimensions.x / dimensions.y;
                    bvh::frustum::Frustum3 {
                        x0: -dx,
                        x1: dx,
                        y0: -dy,
                        y1: dy,
                        z0: camera.properties.z0 as f64,
                        z1: camera.properties.z1 as f64,
                    }
                };

                let range = bvh::range::Range3 {
                    x0: -1.0,
                    x1: 1.0,
                    y0: -1.0,
                    y1: 1.0,
                    z0: 1.0,
                    z1: -1.0,
                };

                let ray = {
                    use cgmath::{InnerSpace, Rotation};

                    let (x, y) = if focus_camera {
                        (0.5, 0.5)
                    } else {
                        (mouse_state.x, mouse_state.y)
                    };

                    bvh::ray::Ray {
                        origin: camera.transform.position,
                        direction: camera
                            .transform
                            .rot_to_parent()
                            .cast::<f64>()
                            .unwrap()
                            .rotate_vector(cgmath::Vector3 {
                                x: (1.0 - x) * frustum.x0 + x * frustum.x1,
                                y: (1.0 - y) * frustum.y0 + y * frustum.y1,
                                z: -1.0,
                            })
                            .normalize()
                            .cast::<f32>()
                            .unwrap(),
                    }
                };

                let cast_start = std::time::Instant::now();
                let mut aabb_intersection_count = 0;
                let mut triangle_intersection_count = 0;

                // Cast ray, find closest hit.
                let mut closest_t = std::f32::INFINITY;
                let mut closest_mesh_index = 0;
                let mut mesh_hit_node_indices = Vec::new();

                for (mesh_index, mesh) in meshes.iter().enumerate() {
                    struct Item {
                        node_base: u32,
                        node_offset: u32,
                    }

                    let mut stack = vec![Item {
                        node_base: 0,
                        node_offset: 0,
                    }];

                    let hit_node_indices = {
                        mesh_hit_node_indices.push(Vec::new());
                        mesh_hit_node_indices.last_mut().unwrap()
                    };

                    while let Some(Item {
                        node_base,
                        node_offset,
                    }) = stack.pop()
                    {
                        let node_index = node_base + node_offset;
                        let node: &bvh::bvh::Node = &mesh.bvh.nodes[node_index as usize];

                        aabb_intersection_count += 1;
                        if let Some(box_t) = bvh::intersect::ray_versus_aabb(
                            ray,
                            bvh::aabb::AABB3 {
                                min: node.min,
                                max: node.max,
                            },
                        ) {
                            if box_t > closest_t {
                                // Can't find a closer intersection in this box.
                            } else {
                                hit_node_indices.push(node_index);

                                if node.count == std::u32::MAX {
                                    // branch.
                                    stack.push(Item {
                                        node_base: node.left_or_offset,
                                        node_offset: 0,
                                    });
                                } else {
                                    // leaf.
                                    for vertex_indices in mesh
                                        .bvh
                                        .triangles
                                        .iter()
                                        .skip(node.left_or_offset as usize)
                                        .take(node.count as usize)
                                    {
                                        let triangle = [
                                            cgmath::Point3::from(Into::<[f32; 3]>::into(
                                                mesh.vertices[vertex_indices[0] as usize],
                                            )),
                                            cgmath::Point3::from(Into::<[f32; 3]>::into(
                                                mesh.vertices[vertex_indices[1] as usize],
                                            )),
                                            cgmath::Point3::from(Into::<[f32; 3]>::into(
                                                mesh.vertices[vertex_indices[2] as usize],
                                            )),
                                        ];

                                        triangle_intersection_count += 1;
                                        if let Some(tri_int) =
                                            bvh::intersect::ray_versus_triangle(ray, triangle)
                                        {
                                            if tri_int.t < closest_t {
                                                closest_t = tri_int.t;
                                                closest_mesh_index = mesh_index;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        match node_offset {
                            0 => stack.push(Item {
                                node_base,
                                node_offset: 1,
                            }),
                            1 => {}
                            _ => unreachable!(),
                        }
                    }
                }

                let cast_elapsed = cast_start.elapsed();

                println!(
                    "aabb {:04}, triangle {:04}, {:?}",
                    aabb_intersection_count, triangle_intersection_count, cast_elapsed
                );

                let wld_to_cam = camera.transform.pos_from_parent().cast::<f64>().unwrap();
                let cam_to_clp = frustum.perspective(&range);
                let clp_to_cam = frustum.inverse_perspective(&range);
                let obj_to_clp = cam_to_clp * wld_to_cam;

                unsafe {
                    gl.viewport(
                        0,
                        0,
                        window_state.dimensions[0] as i32,
                        window_state.dimensions[1] as i32,
                    );

                    gl.enable(gl::DEPTH_TEST);
                    gl.enable(gl::CULL_FACE);
                    gl.cull_face(gl::BACK);

                    gl.clear_color(
                        0.7 + 0.1 * elapsed.sin(),
                        0.8 + 0.1 * (elapsed * 2.0).sin(),
                        0.9 + 0.1 * (elapsed * 3.0).sin(),
                        1.0,
                    );
                    gl.clear(gl::ClearFlag::COLOR_BUFFER | gl::ClearFlag::DEPTH_BUFFER);

                    gl.use_program(program);
                    gl.uniform_matrix4f(
                        CLP_TO_CAM_LOC,
                        gl::MajorAxis::Column,
                        clp_to_cam.cast::<f32>().unwrap().as_ref(),
                    );
                    gl.uniform_matrix4f(
                        OBJ_TO_CLP_LOC,
                        gl::MajorAxis::Column,
                        obj_to_clp.cast::<f32>().unwrap().as_ref(),
                    );
                    gl.bind_vertex_array(vao);

                    let mut color_index = 0;

                    for (mesh_index, mesh) in scene.mesh_descriptions.iter().enumerate() {
                        let color = {
                            let [r, g, b, a] = RGBA_PALETTE[color_index];
                            color_index = (color_index + 1) % RGBA_PALETTE.len();
                            if closest_t < std::f32::INFINITY && mesh_index == closest_mesh_index {
                                let s = (elapsed * 8.0).sin() * 0.5 + 0.5;
                                let f = |n, m| (1.0 - s) * n + s * m;
                                [f(r, 1.0), f(g, 0.0), f(b, 1.0), a]
                            } else {
                                [r * 0.8, g * 0.8, b * 0.8, a]
                            }
                        };

                        if mesh_index != current_mesh {
                            // continue;
                        }

                        gl.uniform_4f(RGBA_LOC, color);
                        gl.draw_elements_base_vertex(
                            gl::TRIANGLES,
                            mesh.triangle_count * 3,
                            gl::UNSIGNED_INT,
                            mesh.triangle_offset as usize * std::mem::size_of::<[u32; 3]>(),
                            mesh.vertex_offset,
                        );
                    }

                    gl.use_program(boxes_program);
                    gl.uniform_matrix4f(
                        CLP_TO_CAM_LOC,
                        gl::MajorAxis::Column,
                        clp_to_cam.cast::<f32>().unwrap().as_ref(),
                    );
                    gl.uniform_matrix4f(
                        OBJ_TO_CLP_LOC,
                        gl::MajorAxis::Column,
                        obj_to_clp.cast::<f32>().unwrap().as_ref(),
                    );
                    gl.bind_vertex_array(boxes_vao);

                    for (mesh_index, hit_node_indices) in mesh_hit_node_indices.iter().enumerate() {
                        let node_descriptions = &mesh_node_descriptions[mesh_index];

                        for &node_index in hit_node_indices.iter() {
                            let offset = node_descriptions[node_index as usize] as usize
                                * std::mem::size_of::<u32>();
                            gl.draw_elements_base_vertex(
                                gl::POINTS,
                                1,
                                gl::UNSIGNED_INT,
                                offset,
                                0,
                            );
                        }
                    }
                }

                windowed_context.swap_buffers().unwrap();
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
