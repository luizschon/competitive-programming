#include <bits/stdc++.h>

using namespace std;

string decide_ruler (string kingdom) {
    string vowels = "AEIOUaeiou";
    char last_letter = kingdom.back();
    
    if (last_letter == 'y' || last_letter == 'Y') {
            return "nobody";
    }
        
    for (auto vowel : vowels)
        if (last_letter == vowel)
            return "Alice";
                
    return "Bob";
}

int main () {
    int t;
    string kingdom;
    cin >> t;
    
    for (int i = 1; i <= t; i++) {
        cin >> kingdom;
        cout << "Case #" << i << ": ";
        cout << kingdom + " is ruled by " + decide_ruler(kingdom) + ".\n";
    }
    
    return 0;
}
