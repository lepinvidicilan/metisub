use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(pos: [f32; 3], texture_coordonates: [f32; 2]) -> Result<Self, String> {
        Ok(Vertex {
            position: [pos[0], pos[1], pos[2]],
            tex_coords: [texture_coordonates[0], texture_coordonates[1]],
        })
    }

    pub fn _get_pos(&self) -> [f32; 3] {
        return self.position;
    }

    pub fn _set_pos(mut self, new_pos: [f32; 3]) -> Self {
        self.position = new_pos;
        return self;
    }
}

implement_vertex!(Vertex, position, tex_coords);
/*
pub struct Quads {
    vertices: [Vertex; 6],
    texture:
}*/
