use glam::DVec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FieldSample {
    pub omega: f32,
    pub flow: [f32; 3],
}

impl From<(f64, DVec3)> for FieldSample {
    fn from((omega, flow): (f64, DVec3)) -> Self {
        FieldSample {
            omega: omega as f32,
            flow: [flow.x as f32, flow.y as f32, flow.z as f32],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameUniform {
    pub jd: f32,
    pub _pad: [f32; 3],
}
