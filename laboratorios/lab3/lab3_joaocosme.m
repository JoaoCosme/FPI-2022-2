close all;


    show_image_and_fft("bw_delta_origin.bmp");
    show_image_and_fft("bw_white_sqr.bmp");

    figure()
    img = imread("bw_delta_origin.bmp");
    subplot(1,2,1);
    imshow(abs(ifft2(img)));
    title("Inversa do Delta");
    img = imread("bw_white_sqr.bmp");
    subplot(1,2,2);
    imshow(abs(ifft2(img)));
    title("Inversa do Quadrado Branco");


    show_image_and_fft("bw_vertical.bmp");
    show_image_and_fft("bw_horizontal.bmp");
    show_image_and_fft("bw_triangle.bmp");
    show_image_and_fft("cameraman.tif");
    show_rotated_image(45);
    show_rotated_image(90);
    show_rotated_image(120);






