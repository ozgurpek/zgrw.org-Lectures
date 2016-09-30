#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>
#include <algorithm>

using namespace std;

void quickSort(vector<int>& v, int beg, int end)
{
	if(end > beg + 1)
	{
		int pv = (rand() % (end - beg)) + beg;
		for(int i = beg; i < end; ++i)
		{
			if(v[i] < v[pv] && i > pv)
			{
				if(i == pv + 1)
				{
					swap(v[pv], v[i]);
					pv = i;
				}
				else
				{
					swap(v[pv], v[pv + 1]);
					swap(v[pv], v[i]);
					++pv;
				}
			}
			
			else if(v[i] > v[pv] && i < pv)
			{
				
				swap(v[pv], v[i]);
				pv = i;

			}
		}
		quickSort(v, beg, pv);
		quickSort(v, pv, end);
	}
}


void quickSort(vector<int>& v)
{
	int beg = 0;
	int end = v.size();
	quickSort(v, beg, end);
}

int main()
{
	srand(time(0));
	vector<int> v;
	cout<<"Array:  ";
	for(int i = 0; i < 20; ++i)
	{
		int d = rand() % 100 + 1;
		v.push_back(d);
		cout<<d<<' ';
	}
	quickSort(v);
	cout<<endl<<endl<<"Sorted Array:  ";
	for(auto it : v)
	{
		cout<<it<<' ';
	}
	cout<<endl<<endl;
}