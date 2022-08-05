// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 03:56:44 AM -03 2022 
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

    double salary,  reajust;
    int percent;
    cin >> salary;

    if (salary <= 400)
        percent = 15;
    else if (salary > 400 and salary <= 800)
        percent = 12;
    else if (salary > 800 and salary <= 1200)
        percent = 10;
    else if (salary > 1200 and salary <= 2000)
        percent = 7;
    else if (salary > 2000)
        percent = 4;

    reajust = salary*percent/100;
    cout << fixed << setprecision(2);
    cout << "Novo salario: " << salary+reajust << endl;
    cout << "Reajuste ganho: " << reajust << endl;
    cout << "Em percentual: " << percent << " %\n";

    return 0;
}
