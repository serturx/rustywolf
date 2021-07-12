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

pub const INT: isize = 4;
pub const FLOAT: isize = 4;
pub const DOUBLE: isize = 8;
