use nalgebra_glm::*;

use super::Vertex;

#[derive(Clone, Copy)]
pub struct Sprite2d {
	pub vertices: [Vertex; 4],
}

impl Sprite2d {
	pub fn new(tex_index: u32, tr: Mat3, layer: f32) -> Self {
		//Down-left, rotate clockwise
		let mut p = tr * vec3(-0.5, -0.5, layer);
		let v0 = Vertex {
			vert_pos: p.data.0[0],
			tex_coords: [0.0, 0.0],
			tex_index,
		};
		p = tr * vec3(-0.5, 0.5, layer);
		let v1 = Vertex {
			vert_pos: p.data.0[0],
			tex_coords: [0.0, 1.0],
			tex_index,
		};
		p = tr * vec3(0.5, 0.5, layer);
		let v2 = Vertex {
			vert_pos: p.data.0[0],
			tex_coords: [1.0, 1.0],
			tex_index,
		};
		p = tr * vec3(0.5, -0.5, layer);
		let v3 = Vertex {
			vert_pos: p.data.0[0],
			tex_coords: [1.0, 0.0],
			tex_index,
		};
		Self {
			vertices: [v0, v1, v2, v3],
		}
	}
	// pub fn modify(&mut self, tex_index: u32, tr: Mat3, layer: f32) {
	// 	let mut p = tr * vec3(-0.5, -0.5, layer);
	// 	self.vertices[0] = Vertex {
	// 		vert_pos: p.data.0[0],
	// 		tex_coords: [0.0, 0.0],
	// 		tex_index,
	// 	};
	// 	p = tr * vec3(-0.5, 0.5, layer);
	// 	self.vertices[1] = Vertex {
	// 		vert_pos: p.data.0[0],
	// 		tex_coords: [0.0, 0.0],
	// 		tex_index,
	// 	};
	// 	p = tr * vec3(0.5, 0.5, layer);
	// 	self.vertices[2] = Vertex {
	// 		vert_pos: p.data.0[0],
	// 		tex_coords: [0.0, 0.0],
	// 		tex_index,
	// 	};
	// 	p = tr * vec3(0.5, -0.5, layer);
	// 	self.vertices[3] = Vertex {
	// 		vert_pos: p.data.0[0],
	// 		tex_coords: [0.0, 0.0],
	// 		tex_index,
	// 	};
	// }
}
