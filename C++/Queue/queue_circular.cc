#include <iostream>
#include <cstdlib>

using namespace std;

#define SIZE 10

typedef struct queue
{
	int beg;
	int end;
	int data[SIZE];
}queue;

void init(queue &q)
{
	q.beg = -1;
	q.end = -1;
}

bool isFull(queue &q)
{
	if((q.end == SIZE - 1 && q.beg == 0) || (q.end + 1 == q.beg && q.end != -1))
		return true;
	return false;
}

bool isEmpty(queue &q)
{
	return q.beg == -1;
}

void insert (queue &q, const int &d)
{
	if(!isFull(q))
	{
		if(isEmpty(q))
			q.beg = q.end;
		q.end++;
		if((q.end == SIZE && q.beg == 0) || q.end == q.beg)
		{
			q.end--;
		}
		else if(q.end == SIZE && q.beg != 0)
		{
			q.end = 0;
		}
		q.data[q.end] = d;
	}
}

int remove (queue &q)
{
	if(!isEmpty(q))
	{
		int d = q.data[q.beg];
		q.beg++;
		if(q.beg == q.end + 1) 
		{
			q.beg = -1;
		}
		else if(q.beg == SIZE)
		{
			q.beg = 0;
		}
		return d;
	}
	return -1;
}

int main()
{
        queue q;
        init(q);
        int i = 0;
        while(!isFull(q))
        {
                cout <<"I am adding "<<i<<" to queue"<<endl;
                insert(q, i);
                i++;
        }
        while(!isEmpty(q))
        {
                cout<<"I have removed "<<remove(q)<<" from queue"<<endl;
        }
return 0;
}
