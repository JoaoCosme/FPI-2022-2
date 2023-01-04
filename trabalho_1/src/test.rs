use crate::matrix_ops::apply_kernel;

#[test]
    pub(crate) fn test_apply_matrix(){
        let laplacian = [[0.0,-1.0,0.0],[-1.0,4.0,-1.0],[0.0,-1.0,0.0]];
        let matrix = [[0,1,10],[0,10,5],[0,1,2]];
        assert_eq!(apply_kernel(laplacian, matrix),33);
}