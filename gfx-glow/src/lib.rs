mod device_glow;
mod shader_glow;

pub use device_glow::DeviceGlow;
pub use shader_glow::ShaderGlow;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
