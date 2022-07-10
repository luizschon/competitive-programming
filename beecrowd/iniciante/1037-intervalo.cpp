// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>
#include <cstddef>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    double num;
    cin >> num;

    if (num >= 0 && num <= 25)
        cout << "Intervalo [0,25]\n";
    else if (num > 25 && num <= 50)
        cout << "Intervalo (25,50]\n";
    else if (num > 50 && num <= 75)
        cout << "Intervalo (50,75]\n";
    else if (num > 75 && num <= 100)
        cout << "Intervalo (75,100]\n";
    else 
        cout << "Fora de intervalo\n";

    return 0;
}
