close all;

low_pass = imread('low_pass.bmp');
high_pass = imread('high_pass.bmp');
gaussian_mask = imread('gaussian_mask.png');
clown_with_noise = imread('Periodic_noise_clown.tif');
cman = imread('cameraman.tif');
clown_filter = imread('clown_filter.bmp');
clown_filter_pos = imread('clown_filter_half_pos.bmp');
clown_filter_neg = imread('clown_filter_half_neg.bmp');


% Showing figures

figure();
subplot(2,2,1);
imshow(low_pass);
title('Low pass filter');

subplot(2,2,2);
imshow(high_pass);
title('High pass filter');

subplot(2,2,3);
imshow(gaussian_mask);
title('Gaussian Mask');

subplot(2,2,4);
imshow(clown_with_noise);
title('Clown with noise');

% Questao 2

quest_dois(cman,low_pass);
quest_dois(cman,high_pass);
quest_dois(cman,gaussian_mask);

% Questao 4

quest_dois(clown_with_noise,clown_filter);
quest_dois(clown_with_noise,clown_filter_pos);
quest_dois(clown_with_noise,clown_filter_neg);

