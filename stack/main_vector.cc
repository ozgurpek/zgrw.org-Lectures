#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::vector;

//#define MAX_SIZE 10

typedef struct stack
{
	vector<int> data;
//	int data[MAX_SIZE];
//	int ptr;
}stack;

/*void init (stack &s)
{
	s.ptr = 0;
}*/

bool isEmpty(const stack &s)
{
	if(s.data.size() == 0)
		return true;
	return false;
}

bool isFull(const stack &s)
{
	if(s.data.size() == s.data.max_size())
		return true;
	return false;
}

void push(stack &s, const int &data)
{
	if(!isFull(s))
	{
		s.data.push_back(data);
	}
	else
	{
		cout<<"Stack is full!"<<endl;
	}
}

int pop (stack &s)
{
	if(!isEmpty(s))
	{
		int temp = s.data.back();
		s.data.pop_back();
		return temp;
	}
	else
	{
		cout<<"Stack is empty!"<<endl;
	}
}

int main()
{
	stack s;
	int i = 0;
	for(; i < 10; i++ )
	{
		cout <<"I am adding "<<i<<" to stack"<<endl;
		push(s, i);
	}
	while(!isEmpty(s))
	{
		cout<<"I have popped out "<<pop(s)<<" from stack"<<endl;
	}	
return 0;
}
