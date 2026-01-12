
// https://godbolt.org/z/chnj64dva
valarray<int> data;               // initially size 0
cout << data.size() << "\n";      // Output: 0
data.resize(100);                 // enlarged
cout << data.size() << "\n";      // Output: 100
valarray<int> data2(200);         // space for 200 values
cout << data2.size() << "\n";     // Output: 200
valarray<int> dataC(5, 20);       // twenty 5s, different from vector
cout << dataC.size() <<": dataC[6]="<< dataC[6]<< "\n"; // Output: 20: dataC[6]=5
valarray dataD{ 2, 3, 5, 7, 11 }; // valarray<int>, initialization list
cout << dataD.size() <<": dataD[3]=" <<dataD[3]<< "\n"; // Output: 5: dataD[3]=7