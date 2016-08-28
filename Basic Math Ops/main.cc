#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int operator*(const vector<int> &v1, const vector<int> &v2)
{
	int sum = 0;
	int sz = min(v1.size(), v2.size()); //remaining dimensions for the smaller vector will be accepted as 0 as we were in the bigger dimension world
	for(int i = 0; i < sz; ++i)
	{
		sum += v1[i] * v2[i];
	}
	return sum;
}

vector<vector<int>> operator*(const vector<vector<int>>& m1, const vector<vector<int>>& m2)
{
	int sz1 = m1.size();
	int sz2 = m2.size();
	vector<vector<int>> res;
	int sum;
	if(sz1 == m2[0].size() && sz2 ==m1[0].size())
	{
		for(int i = 0; i < sz1; ++i)
		{
			vector<int> row;
			for(int k = 0; k < sz1; ++k)
			{
				sum = 0;
				for(int j = 0; j < sz2; ++j)
				{
					sum += m1[i][j] * m2[j][k];
				}
				row.push_back(sum);
			}
			res.push_back(row);
			
		}
	}
	return res;
}

ostream & operator<<(ostream &out, const vector<vector<int>>& m1)
{
	for(auto ml : m1)
	{
		for(auto ms : ml)
		{
			out<<ms<< ' ';
		}
		out<<endl;
	}
	return out;
}

int main()
{
	vector<int> v1 = {1, 2, 3};
	vector<int> v2 = {3, 2, 1};
	cout<<"v1 * v2 = "<<v1 * v2<<endl;
	vector<vector<int>> m1 = {{1,2,3}, {4,5,6}};
	vector<vector<int>> m2 = {{1,2}, {3,4}, {5,6}};
	cout<<"m1 * m2; "<<endl<<m1 * m2<<endl;
}