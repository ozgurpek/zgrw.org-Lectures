#include <iostream>

using std::cout;
using std::endl;

#define MAX_SIZE 10

typedef struct stack
{
	int data[MAX_SIZE];
	int ptr;
}stack;

void init (stack &s)
{
	s.ptr = 0;
}

bool isEmpty(const stack &s)
{
	if(s.ptr == 0)
		return true;
	return false;
}

bool isFull(const stack &s)
{
	if(s.ptr == MAX_SIZE)
		return true;
	return false;
}

void push(stack &s, const int &data)
{
	if(!isFull(s))
	{
		s.data[s.ptr] = data;
		s.ptr++;
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
		s.ptr--;
		return s.data[s.ptr];
	}
	else
	{
		cout<<"Stack is empty!"<<endl;
	}
}

int main()
{
	stack s;
	init(s);
	int i = 0;
	while(!isFull(s))
	{
		cout <<"I am adding "<<i<<" to stack"<<endl;
		push(s, i);
		i++;
	}
	while(!isEmpty(s))
	{
		cout<<"I have popped out "<<pop(s)<<" from stack"<<endl;
	}	
return 0;
}
