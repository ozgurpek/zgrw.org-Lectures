#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib>
#include <ctime>

using namespace std;

#define SIZE 50


int maxSum(const vector<int> &v)
{
	int sum = 0;
	for(int val : v)
	{
		if(val > 0)
			sum += val;
	}
	return sum;
}

int maxContSeq(const vector<int> &v) //Kaden's algorithm
{
	int sumEndsHere = 0;
	int sumSoFar = 0;
	for(int val : v)
	{
		sumEndsHere = max(0, sumEndsHere + val);
		sumSoFar = max(sumEndsHere, sumSoFar);
	}
	return sumSoFar;
}


int main()
{
	srand(time(0));
	vector<int> v;
	for(int i = 0; i < SIZE; ++i)
	{
		int d = rand() % 16;
		if(rand() %2 == 1)
			d *= -1;
		v.push_back(d);
		cout<<d<<' ';
	}
	cout<<endl<<endl;
	cout<<"Max Sum: "<<maxSum(v)<<endl;
	cout<<"Max Cont Sum: "<<maxContSeq(v)<<endl;
	
}