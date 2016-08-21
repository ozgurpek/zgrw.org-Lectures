#include <iostream>
#include <list>
#include <vector>
#include <type_traits>
#include <algorithm>

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
	return data % size;
}


int main()
{
	HashMap<int> hm(10);
	hm.insert(10);
	hm.insert(20);
	hm.insert(30);
	hm.insert(40);
	hm.insert(50);
	hm.insert(60);
	hm.insert(70);
	hm.insert(80);
	hm.insert(90);
	hm.insert(1);
	hm.insert(2);
	hm.insert(3);
	hm.insert(4);
	hm.insert(5);
	hm.insert(6);
	hm.insert(7);
	hm.insert(8);
	hm.insert(9);
	hm.insert(11);
	hm.insert(22);
	hm.insert(33);
	hm.insert(44);
	hm.insert(55);
	hm.insert(66);
	hm.insert(77);
	hm.insert(88);
	hm.insert(99);
	
	cout<<"Search for 10: "<<hm.search(10)<<endl;
	cout<<"Search for 19: "<<hm.search(19)<<endl;
	cout<<"Search for 99: "<<hm.search(99)<<endl;
	cout<<"Search for 9: "<<hm.search(9)<<endl;
	cout<<"Search for 20: "<<hm.search(20)<<endl;
	cout<<"Search for 101: "<<hm.search(101)<<endl;
}