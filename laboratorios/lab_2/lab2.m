# Lab 2

cman = imread("cameraman.tif");

figure();
subplot(3,2,1);
imshow(cman);
title('Original');

# Q2 - Q3

cman_ft = fft2(cman);


# Q2

cman_real = real(cman_ft);
cman_imag = imag(cman_ft);

subplot(3,2,3);
imshow(uint8(cman_real));
title("Real part");

subplot(3,2,4);
imshow(uint8(cman_imag));
title("Image part");


# Q3
subplot(3,2,2)
imshow(log(abs(cman_ft)),[3 10]);
title("Especto de Amplitude");

# Q4
cman_recon = ifft2(cman_ft);
cman_real = uint8(real(cman_recon));
cman_imag = uint8(imag(cman_recon));
subplot(3,2,5);
imshow(cman_real+cman_imag);
title("Inverse FT");

# Q5
cman_recon_from_real = ifft2(real(cman_ft));
cman_recon_from_imag = ifft2(imag(cman_ft)*i);

figure();
subplot(2,2,1);
imshow(uint8(cman_recon_from_real));
title("Rec from Real Part");

subplot(2,2,2);
imshow(uint8(cman_recon_from_imag));
title("Rec from Imag Part");

subplot(2,2,3);
cman_sum = cman_recon_from_imag + cman_recon_from_real;
cman_real = uint8(real(cman_sum));
cman_imag = uint8(imag(cman_sum));
cman_sum = cman_recon_from_imag + cman_recon_from_real;
imshow(uint8(cman_real+cman_imag));
title("Rec from Sum of Parts");

# Q7
figure();
subplot(2,2,1);
cman_ft_shift = fftshift(cman_ft);
imshow(log(abs(cman_ft_shift)),[3 10]);
title("Shifted spectre")

# Q8
inverse_shifted = ifft2(cman_ft_shift);
cman_real = uint8(real(cman_recon));
cman_imag = uint8(imag(cman_recon));
subplot(2,2,2);
recovered_from_shift = cman_real + cman_imag;
imshow(recovered_from_shift);
title("Inverse FT");

# Q9 - Inverse Shift

subplot(2,2,3);
cman_ft_unshift = ifftshift(cman_ft_shift);
imshow(log(abs(cman_ft_unshift)),[3 10]);
title("Unshifted spectre")


# Q9 - Two Shifts

subplot(2,2,4);
cman_ft_shifted_twice = fftshift(cman_ft_shift);
imshow(log(abs(cman_ft_shifted_twice)),[3 10]);
title("Shifted spectre twice")


# Q10 - Recover original image
figure()
inverse_shifted = ifftshift(fft2(inverse_shifted));
cman_recon = ifft2(inverse_shifted);
cman_real = uint8(real(cman_recon));
cman_imag = uint8(imag(cman_recon));
recovered_from_shift = cman_real + cman_imag;
imshow(recovered_from_shift);
title("Q10 Recon");


