pub fn apply_kernel(kernel: [[f32; 3]; 3], matrix: [[u8; 3]; 3]) -> u8 {
    return (kernel[0][0] * matrix[0][0] as f32
        + kernel[0][1] * matrix[0][1] as f32
        + kernel[0][2] * matrix[0][2] as f32
        + kernel[1][0] * matrix[1][0] as f32
        + kernel[1][1] * matrix[1][1] as f32
        + kernel[1][2] * matrix[1][2] as f32
        + kernel[2][0] * matrix[2][0] as f32
        + kernel[2][1] * matrix[2][1] as f32
        + kernel[2][2] * matrix[2][2] as f32) as u8;
}
