#include <iostream>
#include <vector>
#include <ctime>
#include <cstdlib>
#include <algorithm>

using namespace std;

#define MAX 250

int findMissing(const vector<int> &vec, const int &max) //O(N) solution
{
    int remain = (max * (max + 1)) / 2;
    for(int i = 0; i < vec.size(); ++i) //O(n) loop
    {
        remain -= vec[i];
    }
    return remain;
}

int findMissing(vector<int> &vec) //O(nlogn) solution
{
    sort(vec.begin(), vec.end()); //O(nlogn) sort
    int i;
    for(i = 0; i < vec.size(); ++i) //O(n) loop
    {
        if(vec[i] != i + 1)
        {
            break;
        }
    }
    return i + 1; //because i starts from 0
}

int findMissingNonUnique(vector<int> &vec) //O(nlogn) solution
{
    sort(vec.begin(), vec.end()); //O(nlogn) sort
    int i;
    int lookUp = 1;
    for(i = 0; i < vec.size(); ++i) //O(n) loop
    {
        if(vec[i] > lookUp)
        {
            break;
        }
        else if (vec[i] == lookUp)
        {
            ++lookUp;
        }
    }
    return lookUp;
}

int main()
{
    srand(time(0));
    int missing = rand() % MAX + 1;
    cout<<"Missing should be found as: "<< missing << endl;
    vector<int> v;
    cout<<"Array is: ";
    for(int i = 1; i <= MAX; ++i)
    {
        if( i != missing )
        {
            v.push_back(i);
            cout<<i<<" ";
        }
    }
    cout<<endl<<endl;
    cout<<"Missing value is (O(N)): "<<findMissing(v, MAX)<<endl;
    cout<<"Missing value is (O(nlogn)): "<<findMissing(v)<<endl;
    v.clear();
    int d;
    cout<<endl<<endl;
    cout<<"Non-unique array is: ";
    for(int i = 1; i <= MAX; ++i)
    {
        d = rand() % 3 + 1; // max three occurences
        for(int j = 0; j < d && i != missing; ++j)
        {
                v.push_back(i);
                cout<<i<<' ';
        }
    }
    cout<<endl<<endl;
    cout<<"Missing non-unique value is (O(nlogn)): "<<findMissingNonUnique(v)<<endl;
}
