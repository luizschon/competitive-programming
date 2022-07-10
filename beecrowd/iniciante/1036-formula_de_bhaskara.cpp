// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    double a, b, c;
    cin >> a >> b >> c;   

    double delta = b*b - 4*a*c;

    if (delta < 0 || a == 0) {
        cout << "Impossivel calcular\n";
        return 0;
    }
    
    cout << fixed << setprecision(5);
    cout << "R1 = " << (-b + sqrt(delta))/(2*a) << endl;
    cout << "R2 = " << (-b - sqrt(delta))/(2*a) << endl;

    return 0;
}
