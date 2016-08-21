#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>
#include <algorithm>

using namespace std;


vector<int> mergeSort(const vector<int>& v)
{
	if(v.size() > 1)
	{
		int nend = floor(v.size() / 2);
		int end = v.size();
		vector<int> p1;
		vector<int> p2;
		int i = 0, j = 0;
		for(i = 0; i < nend; ++i)
		{
			p1.push_back(v[i]);
		}
		for(i = nend; i < end; ++i)
		{
			p2.push_back(v[i]);
		}
		
		p1 = mergeSort(p1);
		p2 = mergeSort(p2);
		i = 0;
		j = 0;
		vector<int> vn;
		while( i < nend && j < end - nend)
		{
			if(p1[i] > p2[j])
			{
				vn.push_back(p2[j]);
				++j;
			}
			else
			{
				vn.push_back(p1[i]);
				++i;
			}
		}
		while( i < nend) //if not finished
		{
			vn.push_back(p1[i]);
			++i;
		}
		while( j < end - nend) //because second array might be bigger
		{
			vn.push_back(p2[j]);
			++j;
		}
		return vn;
	}
	else
	{
		return v;
	}
}



int main()
{
	srand(time(0));
	vector<int> v;
	cout<<"Array:  ";
	v.push_back(23);
for(int i = 0; i < 25; ++i)
	{
		int d = rand() % 100 + 1;
		v.push_back(d);
		cout<<d<<' ';
	}
	v = mergeSort(v);
	cout<<endl<<endl<<"Sorted Array:  ";
	for(auto it : v)
	{
		cout<<it<<' ';
	}
	cout<<endl<<endl;
}