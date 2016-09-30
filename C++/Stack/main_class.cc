//A generic stack declaration with a class for any type of data


#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::vector;


template <class T>
class stack
{
public:
	stack();
	~stack();
	void push(const T&);
	T pop();
	bool isEmpty();
	bool isFull();
private:
	stack(const stack &); //copy and assignment is forbidden
	stack & operator=(const stack &); //copy and assignment is forbidden
	vector<T> data;
};

template <class T>
stack<T>::stack()
{
	//do nothing for now
}
template <class T>
stack<T>::~stack()
{
	//do nothing for now
}
template <class T>
bool stack<T>::isEmpty()
{
	if(data.size() == 0)
		return true;
	return false;
}
template <class T>
bool stack<T>::isFull()
{
	if(data.size() == data.max_size())
		return true;
	return false;
}
template <class T>
void stack<T>::push(const T &data)
{
	if(!isFull())
	{
		this->data.push_back(data);
	}
	else
	{
		cout<<"Stack is full!"<<endl;
	}
}
template <class T>
T stack<T>::pop ()
{
	if(!isEmpty())
	{
		int temp = data.back();
		data.pop_back();
		return temp;
	}
	else
	{
		cout<<"Stack is empty!"<<endl;
	}
}

int main()
{
	stack<int> s;
	int i = 0;
	for(; i < 10; i++ )
	{
		cout <<"I am adding "<<i<<" to stack"<<endl;
		s.push(i);
	}
	while(!s.isEmpty())
	{
		cout<<"I have popped out "<<s.pop()<<" from stack"<<endl;
	}	
return 0;
}
