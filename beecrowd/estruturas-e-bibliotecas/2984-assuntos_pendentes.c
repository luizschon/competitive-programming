#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SIZE 100000

int main ()
{
    char string[MAX_SIZE];
    unsigned int pendencia = 0;

    scanf("%s[^\n]", string);

    for (int i=0; i<strlen(string); i++)
    {
        if (string[i] == '(')
            pendencia++;
        if (string[i] == ')' && pendencia > 0)
            pendencia--;
    }
    if (!pendencia)
        printf("Partiu RU!\n");
    else
        printf("Ainda temos %d assunto(s) pendente(s)!\n", abs(pendencia));
}
