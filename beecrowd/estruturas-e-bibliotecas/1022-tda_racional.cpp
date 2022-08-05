// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned int u;

using namespace std;

typedef struct ratio {
    int signal; // 0 = positive, 1 = negative
    int num, den;
} ratio_t;

typedef struct expression {
    char op;
    ratio_t r1, r2;
} expression_t;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int n; cin >> n;
    expression_t e;
    ratio_t res, simp;
    int mmc, mdc;

    while (n--) {
        scanf("%d / %d", &e.r1.num, &e.r1.den);
        scanf(" %c ", &e.op);
        scanf("%d / %d", &e.r2.num, &e.r2.den);

        switch (e.op) {
            case '+':
                res.num = e.r1.num*e.r2.den + e.r2.num*e.r1.den;
                res.den = e.r1.den*e.r2.den;
                break;
            case '-':
                res.num = e.r1.num*e.r2.den - e.r2.num*e.r1.den;
                res.den = e.r1.den*e.r2.den;
                break;
            case '*':
                res.num = e.r1.num*e.r2.num;
                res.den = e.r1.den*e.r2.den;
                break;
            case '/':
                res.num = e.r1.num*e.r2.den;
                res.den = e.r2.num*e.r1.den;
                break;
            default: break;
        }
        res.signal = ((res.num < 0 && res.den > 0) || (res.num > 0 && res.den < 0)) ? 1 : 0;
        mdc = __gcd(res.num, res.den);
        simp.num = res.num / mdc;
        simp.den = res.den / mdc;

        cout << ((res.signal) ? '-' : '\0') << abs(res.num) << '/' << abs(res.den);
        cout << " = ";
        cout << ((res.signal) ? '-' : '\0') << abs(simp.num) << '/' << abs(simp.den) << endl;
    }

    return 0;
}
