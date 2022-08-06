// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 01:52:11 PM -03 2022 
 */
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned int u;
typedef std::pair<int, int> pii;
typedef std::vector<int> vi;
typedef std::pair<ll, ll> pll;
typedef std::vector<ll> vll;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int ddd;
    cin >> ddd;

    switch (ddd) {
        case 61: cout << "Brasilia\n"; break;
        case 71: cout << "Salvador\n"; break;
        case 11: cout << "Sao Paulo\n"; break;
        case 21: cout << "Rio de Janeiro\n"; break;
        case 32: cout << "Juiz de Fora\n"; break;
        case 19: cout << "Campinas\n"; break;
        case 27: cout << "Vitoria\n"; break;
        case 31: cout << "Belo Horizonte\n"; break;
        default: cout << "DDD nao cadastrado\n"; break;
    }

    return 0;
}
