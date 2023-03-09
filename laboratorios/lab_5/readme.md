# Fundamentos de Processamento de Imagens

## Laboratorio 5

Nome: João Pedro Cosme da Silva / Cartão 0031472

## Introdução

O presente relatório tem como objetivo demonstrar as atividades realizadas no Laboratorio 5, cujos objetivos foram a verificação de propriedades do teorema da Amostragem em diversas amostras em sua primeira parte, e na validação dos algoritmos de _upscaling_ e _downscaling_ no domínio frequência em imagens coloridas, cinzas e de alta frequência em sua segunda etapa.

## Plot de Funções Contínuas e sua amostragem

Nesta primeira etapa, com o objetivo da verificação do teorema da amostragem, realizamos o _plot_ das seguintes funções $f(t) = cos(u(2\pi/N)t)$ e $g(t) = cos((N-u)(2\pi/N)t)$ a fim de verificar a amostragem de funções de maneira adequada e com aliasing no caso da função $g(t)$.

Para os seguintes resultados, o seguintes código foi utilizado:

```matlab
% Q1

u = [1 3 5];
N = 200;
t = (0:1:(N-1))/N;

for i=1:3
    f_samples = cos(u(i)*2*pi*t);
    g_samples = cos((N-u(i))*2*pi*t);

    figure();
    subplot(1,2,1);
    plot(t,f_samples);
    title(strcat('cos(u*2pi*t, N=',num2str(N),' u =',num2str(u(i))));
    subplot(1,2,2);
    plot(t,g_samples);
    title(strcat('cos((N-u)*2pi*t, N=',num2str(N),' u =',num2str(N-u(i))));
end
```

Para os casos das frequências inicias, podemos verificar que para a função $f(t)$ os resultados obtidos possuem exatamente a mesma quantidade de ciclos que a esperada, demonstrando assim que foram corretamente amostradas. Já no caso da função $g(t)$, podemos ver um clássico caso de _aliasing_, onde uma função de alta frequência ao ser amostrada com uma quantidade insuficiente de amostras, acaba sendo recuperada como uma função de menor frequência, assim perdendo suas características originais.

Ao variarmos as variaveis `u` utilizadas no sampling, podemos ver que, a médida que nos aproximamos de $\frac{N}{2}$ e chegamos ao valor de 99, verificamos artefatos sub-Nyquist para ambas as funções $g(t)$ e $f(t)$. É importante ressaltar, que por mais que o gráfico recuperado possua está aperencia, os resultados encontrados para $f(t)$ ainda estão corretos, visto que respeitam o Teorema da Amostragem. Para que ocorresse aliasing em frequências maiores que as originais, mantendo o $N=200$, a partir de $u=100$ estariamos sujeitos a _aliasing_ em $f(t)$.

Já no caso de alterarmos o valor de N, iriamos obter dois resultados para funções adequadamente amostradas: no caso de aumentarmos o N, teriamos representações cada vez mais suaves, já que teriamos mais pontos para sua formação; já para redução de N, teriamos gráficos cada vez mais retos e abruptos, como podemos ver abaixo:

## Amostragem de Funções 2D

Nesta próxima etapa, repetimos as mesmas operações que as realizadas para funções 1D, só que incrementando nosso escopo para o espaço 2D. Para esta etapa, o seguinte código foi utilizado:

```matlab
% q2
u = [1 3 5];
v = [1 3 5];
N = 200;
M = 200;

f_samples_2D = zeros(N,M);
g_samples_2D = zeros(N,M);
for i=1:3
    for x = 1:N
        for y=1:M
            % f(x,y) = cos(2pi(ux/N + vy/M))
            f_samples_2D(x,y) = cos(2*pi*( u(i)*(x-1)/N + v(i)*(y-1)/M));
            g_samples_2D(x,y) = cos(2*pi*((N-u(i))*(x-1)/N +(M-v(i))*(y-1)/M));
        end
    end


    figure();
    subplot(1,2,1);
    imshow(uint8(128*(f_samples_2D) + 128));
    title(strcat('cos(u*2pi*t, N=',num2str(N),' u =',num2str(u(i))));
    subplot(1,2,2);
    imshow(uint8(128*(g_samples_2D) + 128));
    title(strcat('cos((N-u)*2pi*t, N=',num2str(N),' u =',num2str(N-u(i))));
end
```

Neste casos, a enquanto u=v e M=N, vemos imagens onde possuimos linhas em 45°, já que a variação da função base é a mesma em ambos os casos. Novamente, podemos verificar a amostragem correta de $f(t)$ e o aliasing em $g(t)$, conforme visto no caso da primeira questão.

A medida que u$\neq$v, passamos a observar inclinações que não em 45°, visto que agora a função possui uma frequência maior em uma das direções, assim gerando este efeito de inclinção no sentido com maior variação. Já a medida que N$\neq$M, observamos que a imagem se "achata" no sentido com mais amostras, já que estas medidas definem a quantidade de amostras em cada sentido.

Assim, a partir das experimentações acimas, pudemos validar que o Teorema da Amostragem se mantém válido para a amostragem de funções em 2D, seguindo as mesmas restrições que as validadas em 1D.

## Sub-sampling

Neste exercício, artificialmente geramos casos de sub-sampling para que pudéssemos verificar as distorções causadas na imagem base neste caso. Abaixo, segue o código utilizado nesta etapa:

```matlab

k = [2 4 8];

for i=1:3
    new_rows_cman_cman = rows_cman/k(i);
    new_columns_cman_cman = columns_cman/k(i);
    new_image_cman = zeros(new_rows_cman_cman,new_columns_cman_cman);

    for x=1:new_rows_cman_cman
        for y=1:new_columns_cman_cman
            new_image_cman(x,y)= cman(k(i)*x,y*k(i));
        end
    end

    new_rows_chirp = rows_chirp/k(i);
    new_columns_chirp = columns_chirp/k(i);
    new_image_chirps = zeros(new_rows_chirp,new_columns_chirp);

    for x=1:new_rows_chirp
        for y=1:new_columns_chirp
            new_image_chirps(x,y)= chirp(k(i)*x,y*k(i));
        end
    end

    figure();

    subplot(1,2,1);
    imshow(uint8(new_image_cman));
    title(strcat('Cman with k=',num2str(k(i))));

    subplot(1,2,2);
    imshow(uint8(new_image_chirps));
    title(strcat('Chirp with k=',num2str(k(i))));
end

```

Assim, podemos verificas os resultados obtidos abaixo:

Como podemos ver, a imagem do _cameraman_ vai se tornando _pixelizada_ e perdendo os detalhes da imagem original. Porém, como grande parte de sua imagem ainda é representada por baixas frequências, visto que imagens naturais seguem um comportamento de $\frac{1}{f}$ em seu espectro no domínio frequência. Porém, a imagem do `chirp` é uma imagem artificialmente gerada, composta por altas frequências, logo o sub-sampling causa a perca destes detalhes finos necessários para a composição da imagem original, gerando um efeito onde se perde completamente a base com a imagem original.

## Upscaling

## Downscaling
