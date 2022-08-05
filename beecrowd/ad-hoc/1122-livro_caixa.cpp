#include <bits/stdc++.h>
#include <cstdio>

#define MAX_SIZE 40
#define mp make_pair

using namespace std;

// Vetor que armazena o sinal de cada transação:
// -1: subtração
//  0: não definido
//  1: soma
int sign[MAX_SIZE];
int n, f, fluxo[MAX_SIZE];
map<pair<int, int>, int> memo;

/*  */
int busca_completa(int idx, int curr_sum) {
    if (idx == n) {
        if (curr_sum == f)
            return 1;
        else
            return 0;
    }

    if (memo.count(mp(curr_sum, idx)) > 0)
        return memo[mp(curr_sum, idx)];

    int right, left;
    right = busca_completa(idx+1, curr_sum+fluxo[idx]);
    left = busca_completa(idx+1, curr_sum-fluxo[idx]);

    if (right && !left)
        sign[idx] = 1;
    else if (!right && left)
        sign[idx] = -1;
    else if (right && left)
        sign[idx] = 0;

    return memo[mp(curr_sum, idx)] = (right || left) ? 1 : 0;
}

int main () {
    while(scanf("%d %d", &n, &f)) {
        if (n == 0 && f == 0)
            break;
        for (int i=0; i<n; i++) {
            scanf("%d", &fluxo[i]);
            // Inicializa vetor que armazena o sinal das operações
            sign[i] = 0;
        }

        int verificado = busca_completa(0, 0);

        for (int i=0; i<n && verificado; i++) {
            switch (sign[i]) {
                case -1: printf("-");
                case 0:  printf("?");
                case 1:  printf("+");
            }
        }
        if (!verificado)
            printf("*");

        printf("\n");
    }

    return 0;
}
