#include <stdio.h>

int main ()
{
    int segundos, minutos, horas;
    scanf("%d", &segundos);

    horas = segundos/(60*60);
    segundos %= 60*60;
    minutos = segundos/60;
    segundos %= 60;

    printf("%d:%d:%d\n", horas, minutos, segundos);

    return 0;
}
