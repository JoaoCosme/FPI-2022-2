function apply_linear_functions()

close all;

    figure();
    subplot(2,2,1);
    img = imread('cameraman.tif');
    [rows, columns] = size(img);
    imshow(img);
    title('Imagem original');
    subplot(2,2,2);
    img_ft = fft2(img);
    imshow(log(abs(fftshift(img_ft))),[3 10]);
    title('Espectro');
    subplot(2,2,3);
    img_brilho = img_ft;
    img_brilho(1) =img_brilho(1)+100*rows*columns;
    imshow(uint8(ifft2(img_brilho)));
    title('Aumento de brilho');
    subplot(2,2,4);
    img_brilho = img_ft;
    img_brilho(1) =img_brilho(1)-100*rows*columns;
    imshow(uint8(ifft2(img_brilho)));
    title('Redução de brilho');

    
    figure();
    subplot(2,2,1);
    imshow(img);
    title('Imagem original');
    subplot(2,2,2);
    img_ft = fft2(img);
    imshow(log(abs(fftshift(img_ft))),[3 10]);
    title('Espectro');
    subplot(2,2,3);
    img_contraste = img_ft;
    imshow(uint8(ifft2(img_contraste*1.5)));
    title('Aumento de ccontraste');
    subplot(2,2,4);
    imshow(uint8(ifft2(img_contraste*0.5)));
    title('Redução de contraste');
    
    
     figure();
    subplot(1,2,1);
    imshow(img);
    title('Imagem original');
    subplot(1,2,2);
    img_ft = fft2(img);
    img_ft = - img_ft;
    img_ft(1) = 255 * rows * columns + img_ft(1); 
    imshow(uint8(ifft2(img_ft)));
    title('Negativo');

    figure();
    img = imread('kitty.png');
    [rows, columns, numberOfChannels] = size(img);
    subplot(1,2,1);
    imshow(img);
    title('Imagem original');
    
    ft_red = fft2(img(:,:,1));
    ft_green = fft2(img(:,:,2));
    ft_blue = fft2(img(:,:,3));
%     
    ft_red = - ft_red;
    ft_red(1) = 255 * rows * columns + ft_red(1); 
%     
    ft_green = - ft_green;
    ft_green(1) = 255 * rows * columns + ft_green(1); 
%     
    ft_blue = - ft_blue;
    ft_blue(1) = 255 * rows * columns + ft_blue(1); 
    
    reconst = uint8(cat(3, ifft2(ft_red), ifft2(ft_green), ifft2(ft_blue)));
    subplot(1,2,2);
    imshow(reconst);
    title('Negativo');

    
 