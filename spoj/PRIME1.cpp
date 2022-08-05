// vim: noai:ts=4:sw=4
#include <algorithm>
#include <bits/stdc++.h>
#include <vector>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned int u;
typedef std::vector<int> vi;

using namespace std;

/* Sieve of Sundaram is a deterministic algorithm to find all
 * prime numbers up to a specified integer (n) */
vi sieve_of_Sundaram(int n) {
    vector<bool> integers; 
    vi known_primes = {2}; 

    if (n >= 3) {
        int k = (n-3)/2+1;

        for (int i=0; i<k; i++)
            integers.pb(true);

        int range = (sqrt(n)-3)/2+1;
        for (int i=0; i<=range; i++) {
            int p = 2*i + 3;
            int s = (p*p - 3)/2;

            for (int j=s; j<k; j+=p)
                integers[j] = false;
        }
        
        for (int i=0; i<k; i++) {
            if (integers[i] == true)
                known_primes.pb(2*(i+1)+1);
        }
    }

    return known_primes;
}

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int t;
    ll m, n;
    cin >> t;

    while (t--) {
        cin >> m >> n;
        vi primes = sieve_of_Sundaram(n);

        for(auto it=lower_bound(primes.begin(), primes.end(), m); it!=primes.end(); it++)
            cout << *it << '\n';

        if (t != 0)
            cout << '\n';
    }

    return 0;
}
