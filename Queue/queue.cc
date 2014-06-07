#include <iostream>

using namespace std;

#define SIZE 10

typedef struct queue
{
	int end;
	int data[SIZE];
}queue;

void init(queue &q)
{
	q.end = 0;
}

bool isFull(queue &q)
{
	return q.end == SIZE;
}

bool isEmpty(queue &q)
{
	return q.end == 0;
}

void insert (queue &q, const int &d)
{
	if(!isFull(q))
	{
		q.data[q.end++] = d;
	}
}

void shiftQueue (queue &q)
{
	for(int i = 0; i < q.end - 1; i++)
	{
		q.data[i] = q.data[i + 1];
	}
	q.end--;
}

int remove (queue &q)
{
	if(!isEmpty(q))
	{
		int d = q.data[0];
		shiftQueue(q);
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
