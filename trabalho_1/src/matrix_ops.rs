pub mod matrix_ops{
    pub fn apply_kernel(kernel:[[f32;3];3], matrix:[[u8;3];3]) -> u8{
        return kernel[0][0]*matrix[0][0];
    }
}