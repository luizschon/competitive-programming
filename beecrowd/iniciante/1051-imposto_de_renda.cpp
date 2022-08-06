// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 02:31:05 PM -03 2022 
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

    double salary, taxes = 0;
    cin >> salary;

    if (salary <= 2000)
        cout << "Isento\n";
    else {
        if (salary > 2000)
            taxes += min(salary-2000, (double) 1000)*8/100;
        if (salary > 3000)
            taxes += min(salary-3000, (double) 1500)*18/100;
        if (salary > 4500)
            taxes += (salary-4500)*28/100;
    }

    cout << fixed << setprecision(2);
    if (taxes != 0)
        cout << "R$ " << taxes << endl;

    return 0;
}
