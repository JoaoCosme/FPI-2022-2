function show_image_and_fft(path)

    figure();
    subplot(1,3,1);
    img = imread(path);
    imshow(img);
    title("Imagem original");
    subplot(1,3,2);
    imshow(log(abs(fftshift(fft2(img)))),[3 10]);
    title("Espectro");
    subplot(1,3,3);
    inversa = ifft2(fft2(img));
    imshow(inversa);
    title("Inversa da transformada");

