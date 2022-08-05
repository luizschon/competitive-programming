// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Universidade de Brasília - UnB - Fri Aug  5 04:07:02 AM -03 2022 
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

    string ani[8] = {"aguia","pomba","homem","vaca","pulga","lagarta","sanguessuga","minhoca"};
    string c1, c2, c3, ans;
    int idx = 0;
    cin >> c1 >> c2 >> c3;

    if (c1 == "invertebrado")
        idx |= 1 << 2;
    if (c2 == "mamifero" or c2 == "anelideo")
        idx |= 1 << 1;
    if (c2 == "ave")
        if (c3 == "onivoro")
            idx |= 1;
    if (c2 == "mamifero")
        if (c3 == "herbivoro")
            idx |= 1;
    if (c2 == "inseto")
        if (c3 == "herbivoro")
            idx |= 1;
    if (c2 == "anelideo")
        if (c3 == "onivoro")
            idx |= 1;

    cout << ani[idx] << endl;

    return 0;
}
