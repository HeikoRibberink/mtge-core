use std::{
	error::Error,
	ops::{Deref, DerefMut},
};

use rayon::prelude::*;

use glium::{
	self, backend::Facade, implement_uniform_block, implement_vertex, texture::TextureHandle,
	uniform, uniforms::UniformBuffer, DrawError, IndexBuffer, Program, Surface, VertexBuffer, ProgramCreationError, buffer::BufferCreationError,
};

use nalgebra_glm::*;

use self::sprite::Sprite2d;

pub mod sprite;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
	//Array of texturehandles, vertex contain index into that array.
	pub vert_pos: [f32; 3],
	pub tex_coords: [f32; 2],
	pub tex_index: u32,
}
implement_vertex!(Vertex, vert_pos, tex_coords, tex_index);

const TEX_CAP: usize = 1024;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Textures<'a> {
	textures: [TextureHandle<'a>; TEX_CAP],
}
implement_uniform_block!(Textures<'a>, textures);

impl<'a> Deref for Textures<'a> {
	type Target = [TextureHandle<'a>; TEX_CAP];

	fn deref(&self) -> &Self::Target {
		&self.textures
	}
}

impl<'a> DerefMut for Textures<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.textures
	}
}

const VERTEX_SHADER: &str = include_str!("../../shaders/sprite2d/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../../shaders/sprite2d/fragment.glsl");

pub fn generate_program<F: ?Sized + Facade>(facade: &F) -> Result<Program, ProgramCreationError> {
	let fragment_shader_replaced =
		FRAGMENT_SHADER.replace("<%texture_count%>", &format!("{}", TEX_CAP));
	glium::program::Program::from_source(
		facade,
		VERTEX_SHADER,
		&fragment_shader_replaced,
		None,
	)
}

pub fn generate_textures<'a, F: ?Sized + Facade>(
	facade: &F,
	default_texture: TextureHandle<'a>,
) -> Result<UniformBuffer<Textures<'a>>, BufferCreationError> {
	UniformBuffer::new(
		facade,
		Textures {
			textures: [default_texture; TEX_CAP],
		},
	)
}

pub fn generate_buffers<F: Facade + ?Sized>(
	facade: &F,
	sprites: &[Sprite2d],
) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u32>), Box<dyn Error>> {
	let vertices: Vec<Vertex> = (0..sprites.len() * 4)
		.into_par_iter()
		.map(|i| {
			let part_i = i & 0b11;
			let index = i >> 2;
			sprites[index].vertices[part_i]
		})
		.collect();
	let vertex_buffer = VertexBuffer::new(facade, &vertices)?;
	let indices: Vec<u32> = (0..sprites.len() * 6)
		.into_par_iter()
		.map(|i| {
			const BLUEPRINT: [u32; 6] = [0, 1, 3, 3, 1, 2];
			let part_i = i % 6;
			let index = (i / 6) as u32;
			index + BLUEPRINT[part_i]
		})
		.collect();
	let index_buffer =
		IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &indices)?;
	Ok((vertex_buffer, index_buffer))
}

pub fn render<'a, S>(
	program: &Program,
	texture_buffer: &UniformBuffer<Textures<'a>>,
	surface: &mut S,
	camera: Mat4,
	vertex_buffer: &VertexBuffer<Vertex>,
	index_buffer: &IndexBuffer<u32>,
) -> Result<(), DrawError>
where
	S: Surface,
{
	//TODO implement rendering mechanism, where two rectangles per texture are generated and moved to the correct place using camera offsets.
	let uniforms = uniform! {
		camera: camera.data.0,
		Textures: texture_buffer
	};
	surface.draw(
		vertex_buffer,
		index_buffer,
		program,
		&uniforms,
		&Default::default(),
	)?;
	Ok(())
}
