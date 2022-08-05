// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 03:38:55 AM -03 2022 
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

    int ini_h, ini_m, end_h, end_m, total_h, total_m;
    cin >> ini_h >> ini_m >> end_h >> end_m;

    total_h = end_h-ini_h;
    total_m = end_m-ini_m;

    if (total_h == 0) {
        if (total_m <= 0)
            total_h += 24;
    } else if (total_h < 0) {
        total_h += 24;
    }

    if (total_m < 0) {
        total_h -= 1;
        total_m += 60;
    }
    cout << "O JOGO DUROU " << total_h << " HORA(S) E " << total_m << " MINUTO(S)\n";

    return 0;
}
