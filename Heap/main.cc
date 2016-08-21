#include <iostream>
#include <vector>
#include <algorithm>
#include <ctime>
#include <cstdlib>


using namespace std;


template <typename T>
class Heap //min heap implementation
{
public:
	Heap();
	void insert(const T& data);
	T getMin();
	T removeMin();
	bool isEmpty()
	{
		return v.empty();
	}
private:
	void heapify(int idx);
	void heapifyDel(int idx);
	vector<T> v;
};

template <typename T>
Heap<T>::Heap()
{
}

template <typename T>
void Heap<T>::insert(const T& data)
{
	int idx = v.size();
	v.push_back(data);
	heapify(idx);
}

template <typename T>
T Heap<T>::getMin()
{
	return v[0];
}

template <typename T>
void Heap<T>::heapify(int idx)
{
	if(idx != 0) //combo breaker
	{
		int parrent = (idx %2 == 0)?idx / 2: (idx - 1)/2;
		if(v[idx] < v[parrent])
		{
			swap(v[idx], v[parrent]);
			heapify(parrent);
		}
	}
}

template <typename T>
T Heap<T>::removeMin()
{
	int idx = v.size() - 1;
	swap(v[idx], v[0]);
	T val = v[idx];
	v.pop_back();
	heapifyDel(0);
	return val;
}

template <typename T>
void Heap<T>::heapifyDel(int idx)
{
	if(idx <= ceil((v.size() - 1) / 2) - 1 && v.size() > 1)
	{
		int ch1 = (idx == 0) ? 1 : idx * 2;
		int ch2 = ch1 + 1;
		int mIdx;
		
		if(v[ch1] < v[ch2])
			mIdx = ch1;
		else
			mIdx = ch2;
		
		if(v[idx] > v[mIdx])
		{
			swap(v[idx], v[mIdx]);
			heapifyDel(mIdx);
		}
	}
}

int main()
{
	srand(time(0));
	Heap<int> h;
	cout<<"Random array;"<<endl;
	for(int i = 0; i < 11; ++i)
	{
		int d = rand() % 20 + 1;
		h.insert(d);
		cout<<d<<' ';
	}
	cout<<endl<<endl;
	cout<<"Heap sorted array;"<<endl;
	while(!h.isEmpty()) //Here an example of heap sort :)
	{
		cout<<h.removeMin()<<endl;
	}
}