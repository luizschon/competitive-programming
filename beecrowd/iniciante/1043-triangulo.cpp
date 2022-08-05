// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 02:58:45 AM -03 2022 
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

    double a, b, c;
    cin >> a >> b >> c;
    cout << fixed << setprecision(1);

    if ((abs(b-c) < a and b+c > a) and
        (abs(a-c) < b and a+c > b) and
        (abs(a-b) < c and a+b > c)) {
        cout << "Perimetro = " << a+b+c << endl;
    } else {
        cout << "Area = " << (a+b)*c/2 << endl;
    }

    return 0;
}
