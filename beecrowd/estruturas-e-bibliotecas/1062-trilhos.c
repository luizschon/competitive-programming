#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_SIZE 1000

typedef struct pilha_t {
    int chave;
    struct pilha_t * prox;
} pilha_t;

_Bool vazia (pilha_t * pilha) {
    return !pilha;
}

pilha_t * novo_elemento (int chave) {
    pilha_t * e = (pilha_t*) malloc(sizeof(pilha_t));
    if (!e)
        return NULL;

    e->chave = chave;
    e->prox = NULL;

    return e;
}

void push (pilha_t ** pilha, pilha_t * e) {
    if (!pilha || !e)
        return;

    if (vazia(*pilha))
    {
        *pilha = e;
        return;
    }
    e->prox = *pilha;
    *pilha = e;
}

void pop (pilha_t ** pilha) {
    pilha_t * temp = NULL;

    if (pilha && !vazia(*pilha))
    {
        temp = *pilha;
        *pilha = (*pilha)->prox;
        free(temp);
        temp = NULL;
    }
}

void esvazia_pilha (pilha_t ** pilha) {
    pilha_t * temp = NULL;

    if (pilha && !vazia(*pilha))
    {
        while(!vazia(*pilha))
        {
            temp = *pilha;
            *pilha = (*pilha)->prox;
            free(temp);
            temp = NULL;
        }
    }
}

int main ()
{
    pilha_t * pilha = NULL;
    int N, i, j=0;
    int vagoes[MAX_SIZE];
    
    while (scanf("%d", &N) && N)
    {
        while (1)
        {
            j=0;
            for (i=0; i<N; i++)
            {
                scanf("%d", &vagoes[i]);
                if (vagoes[i] == 0)
                {
                    printf("\n");
                    j=-1;
                    break;
                }
            }
            if (j==-1)
                break;

            for (i=1; i<=N; i++)
            {
                push(&pilha, novo_elemento(i));
                while (!vazia(pilha) && pilha->chave == vagoes[j])
                {
                    j++;
                    pop(&pilha);
                }
            }

            if (vazia(pilha))
                printf("Yes\n");
            else
                printf("No\n");

            esvazia_pilha(&pilha);
            pilha = NULL;
        }
    }


    return 0;
}
