use crate::geometry::Vertex;
use crate::Error;
use std::fmt;
use std::fmt::Formatter;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Debug)]
pub struct Mesh {
    geometry: MeshGeometry,
    buffers: Option<MeshBuffers>,
}

impl Mesh {
    pub fn new(geometry: MeshGeometry, buffers: Option<MeshBuffers>) -> Self {
        Self { geometry, buffers }
    }

    pub fn from_geometry(geometry: MeshGeometry, device: &wgpu::Device) -> Result<Self, Error> {
        let buffers = Some(geometry.create_buffers(device)?);
        Ok(Self { buffers, geometry })
    }

    #[inline]
    pub fn buffers(&self) -> Option<&MeshBuffers> {
        self.buffers.as_ref()
    }
    #[inline]
    pub fn geometry(&self) -> &MeshGeometry {
        &self.geometry
    }
}

#[derive(Debug, Clone)]
pub struct MeshGeometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub label: Option<String>,
}

impl Default for MeshGeometry {
    fn default() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            label: None,
        }
    }
}

impl MeshGeometry {
    fn create_buffers(&self, device: &wgpu::Device) -> Result<MeshBuffers, Error> {
        let label = match &self.label {
            Some(l) => Some(l.as_str()),
            None => None,
        };
        let ibo = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsage::INDEX,
        });

        let vbo = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        return Ok(MeshBuffers {
            vertex_buffer: vbo,
            index_buffer: ibo,
        });
    }
}

#[derive(Debug)]
pub struct MeshBuffers {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
}
