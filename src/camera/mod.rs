// 整体抽象是摄像机，
// 但是主要是三个矩阵
// 模型矩阵，投影矩阵，视图矩阵(摄像机坐标)
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}