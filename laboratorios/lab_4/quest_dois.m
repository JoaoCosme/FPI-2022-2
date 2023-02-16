function quest_dois(image,filter)
    figure();
    
    subplot(2,3,1);
    imshow(filter);
    title('Filter');
    cameraman = image;
    
    subplot(2,3,2);
    imshow(cameraman);
    title('Imagem Original');
    cman_fft2 = fft2(cameraman);
    
    subplot(2,3,3);
    imshow(log(abs(cman_fft2)),[]);
    title('Epectro');
    
    subplot(2,3,4);
    cman_fft2_shifter = fftshift(cman_fft2);
    imshow(log(abs(cman_fft2_shifter)),[]);
    title('Epectro Shiftado');
    
    filter_adjusted = double(filter) / double(max(max(filter)));
    filtered = filter_adjusted .* cman_fft2_shifter;
    
    subplot(2,3,5);
    imshow(log(abs(filtered)),[3 10]);
    title('Epectro Filtrado');
    
    subplot(2,3,6);
    imshow(uint8(real(ifft2(fftshift(filtered)))));
    title('Filtrado');
   
end