#include <iostream>
#include <list>
#include <vector>
#include <type_traits>
#include <algorithm>
#include <cstdlib>
#include <ctime>
#include <cmath>

using namespace std;

#define DEF_SIZE 20




template <typename T>
class HashMap
{
public:
	HashMap(); //Default coonstructor
	HashMap(int size); //Define size
	void insert(const T& data);
	bool search(const T& data);
private:
	int hash(typename std::enable_if<std::is_arithmetic<T>::value, T>::type data);
	int size;
	vector<list<T>> v; 
	
};

template<typename T>
HashMap<T>::HashMap()
:size(DEF_SIZE), v(size)
{
}

template<typename T>
HashMap<T>::HashMap(int size)
:size(size), v(size)
{
}

template<typename T>
void HashMap<T>::insert(const T& data)
{
	int idx = hash(data);
	v[idx].push_back(data);
}

template<typename T>
bool HashMap<T>::search(const T& data)
{
	int idx = hash(data);
	auto it = find(v[idx].begin(), v[idx].end(), data);
	return it != v[idx].end(); //check if it is in the list or not and return the result
}

template<typename T>
int HashMap<T>::hash(typename std::enable_if<std::is_arithmetic<T>::value, T>::type data)
{
	return abs(data) % size;
}


int main()
{
	srand(time(0));
	vector<int> v;
	int i;
	int target;
	for(i = 0; i < DEF_SIZE * 3; ++i)
	{
		int d = rand() % 16;
		if (rand() % 2 == 1)
			d *= -1;
		v.push_back(d);
		cout<<d<<' ';
	}
	cout<<endl<<endl<<"Enter a number between 1 - 15 as sum: ";
	cin>>target;
	HashMap<int> hm;
	for(int it : v)
	{
		int rm = target - it;
		if(hm.search(rm))
		{
			cout<<it<<" - "<<rm<<endl;
		}
		hm.insert(it);
		
	}
}