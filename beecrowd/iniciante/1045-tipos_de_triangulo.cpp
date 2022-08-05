// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 03:16:04 AM -03 2022 
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

    vector<double> v(3,0);
    cin >> v[0] >> v[1] >> v[2];
    sort(v.begin(), v.end(), greater<double>());

    if (v[0] >= v[1] + v[2])
        cout << "NAO FORMA TRIANGULO\n";
    else if (v[0]*v[0] == v[1]*v[1] + v[2]*v[2])
        cout << "TRIANGULO RETANGULO\n";
    else if (v[0]*v[0] > v[1]*v[1] + v[2]*v[2])
        cout << "TRIANGULO OBTUSANGULO\n";
    else if (v[0]*v[0] < v[1]*v[1] + v[2]*v[2])
        cout << "TRIANGULO ACUTANGULO\n";

    if (v[0] == v[1] and v[1] == v[2])
        cout << "TRIANGULO EQUILATERO\n";
    else if (v[0] == v[1] or v[0] == v[2] or v[1] == v[2])
        cout << "TRIANGULO ISOSCELES\n";

    return 0;
}
