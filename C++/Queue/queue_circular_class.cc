#include <iostream>
#include <cstdlib>

using namespace std;

#define SIZE 10

template <class T>
class queue
{
public:
	queue();
	bool isFull();
	bool isEmpty();
	void insert (const T &d);
	T remove ();
private:
	int beg;
	int end;
	T data[SIZE];
};

template <class T>
queue<T>::queue()
{
	beg = -1;
	end = -1;
}

template <class T>
bool queue<T>::isFull()
{
	if((end == SIZE - 1 && beg == 0) || (end + 1 == beg && end != -1))
		return true;
	return false;
}

template <class T>
bool queue<T>::isEmpty()
{
	return beg == -1;
}

template <class T>
void queue<T>::insert (const T &d)
{
	if(!isFull())
	{
		data[end + 1] = d;
		if(isEmpty())
			beg = end;
		end++;
		if((end == SIZE && beg == 0) || end == beg)
		{
			end--;
		}
		else if(end == SIZE && beg != 0)
		{
			end = 0;
		}
	}
}

template <class T>
T queue<T>::remove ()
{
	if(!isEmpty())
	{
		T d = data[beg];
		beg++;
		if(beg == end + 1) 
		{
			beg = -1;
		}
		else if(beg == SIZE)
		{
			beg = 0;
		}
		return d;
	}
	return -1;
}

int main()
{
        queue <int>q;
        int i = 0;
        while(!q.isFull())
        {
                cout <<"I am adding "<<i<<" to queue"<<endl;
                q.insert(i);
                i++;
        }
        while(!q.isEmpty())
        {
                cout<<"I have removed "<<q.remove()<<" from queue"<<endl;
        }
return 0;
}
