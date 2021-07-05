pub mod debug;
mod framebuffer;
mod shader;
mod ssbo;
mod texturesampler;

pub use framebuffer::Framebuffer;
pub use shader::Shader;
pub use ssbo::ISSBO;
pub use ssbo::SSBO;
pub use texturesampler::TextureSampler;

pub const INT32: isize = 4;
pub const UINT32: isize = 4;
pub const FLOAT32: isize = 4;
pub const FLOAT64: isize = 8;
