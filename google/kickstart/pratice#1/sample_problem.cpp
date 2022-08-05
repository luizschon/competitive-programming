#include <bits/stdc++.h>

using namespace std;

int main () {
    int total_candies, t, n, m, temp;
    cin >> t;
    
    for (int i = 1; i <= t; i++) {
        cin >> n >> m;
        total_candies = 0;
        
        while (n--) {
            cin >> temp;
            total_candies += temp;
        }
        cout << "Case #" << i << ": " << total_candies % m << endl;
    }
    
    return 0;
}
