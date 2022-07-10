#include <stdio.h>
#define MAX_LENGTH 255

int main ()
{
    int n_quest=1, valor_cinza[MAX_LENGTH][5], i=0, j, flag, opcao_escolhida;

    while(1)
    {
        scanf("%d", &n_quest);
        if (!n_quest) break;

        for (i=0; i<n_quest; i++)
        {
            flag=0; 

            for (j=0; j<5; j++)
            {
                scanf("%d", &valor_cinza[i][j]);
                if (valor_cinza[i][j]<=127)
                {
                    flag++;
                    opcao_escolhida = j;
                }
            }
            if (flag==1)
            {
                switch(opcao_escolhida)
                {
                    case 0: printf("A\n"); break;
                    case 1: printf("B\n"); break;
                    case 2: printf("C\n"); break;
                    case 3: printf("D\n"); break;
                    case 4: printf("E\n"); break;
                }
            }
            else printf("*\n");
        }
    }
    return 0;
}
