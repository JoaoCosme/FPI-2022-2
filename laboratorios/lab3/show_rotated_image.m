function show_rotated_image(degrees)

    figure();
    subplot(2,2,1);
    img = imread("cameraman.tif");
    imshow(img);
    title("Imagem original");
    subplot(2,2,2);
    imshow(log(abs(fftshift(fft2(img)))),[3 10]);
    title("Espectro");
    cman_rot = imrotate(img, degrees, 'bilinear', 'crop');
    title("Imagem rotacionada");
    subplot(2,2,3);
    imshow(cman_rot);
    subplot(2,2,4);
    imshow(log(abs(fftshift(fft2(cman_rot)))),[3 10]);
    title("Espectro da imagem rotacionada");
