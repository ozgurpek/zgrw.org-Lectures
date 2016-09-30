#include <iostream>

using namespace std;

template <class T>
class list
{
public:
	list<T> * insert (const T &d);
	list<T> * searchAndRemove(const T &d);
	list<T> * removeHead (T &d);
	list<T> * removeLast(T &d);
	template <class Y>
	friend list<Y> * init(list<Y> *, const Y &d);
private:
	list();
	list<T> *next;
	T data;
};

template <class T>
list<T>::list()
{
}

template <class T>
list<T> * init(list<T> *head, const T &d)
{
	if(head == NULL)
	{
		head = new list<T>;
		head -> data = d;
		head -> next = NULL;
	}
	return head;
}

template <class T>
list<T> * list<T>::insert (const T &d)
{
	if(this != NULL)
	{
		if(this-> next == NULL)
			this -> next = init(this -> next, d);
		else
			this-> next = this->next->insert(d);
	}
	return this;
}

template <class T>
list<T> * list<T>::searchAndRemove(const T &d)
{
	list<T> *ptr = this;
	list<T> *pptr = NULL;
	while(ptr != NULL)
	{
		if(ptr->data == d)
		{
			if(pptr != NULL)
			{
				pptr->next = ptr->next;
				delete ptr;
				return this;
			}
			else
			{
				ptr = ptr->next;
				delete this;
				return ptr;
			}
		}
		pptr = ptr;
		ptr = ptr->next;
	}
	return this;
}

template <class T>
list<T> * list<T>::removeHead (T &d)
{
	list<T> *ptr = this;
	if(ptr != NULL)
	{
		d = ptr->data;
		ptr = ptr-> next;
		delete this;
	}
	return ptr;
}

template <class T>
list<T> * list<T>::removeLast(T &d)
{
	list<T> *ptr = this;
	list<T> *pptr = NULL;
	if(this != NULL)
	{
		while(ptr -> next != NULL)
		{
			pptr= ptr;
			ptr = ptr -> next;
		}
		d = ptr->data;
		if(pptr != NULL)
		{
			pptr ->next = ptr->next;
			delete ptr;
			return this;
		}
		else
		{
			delete ptr;
			return NULL;
		}
	}
	return this;
}

int main()
{
	list <double> *head = NULL;
	head = init(head, 0.0);
	double i = 1;
	for(; i < 10; i++ )
	{
		cout <<"I am adding "<<i<<" to list"<<endl;
		head = head->insert(i);
	}
	head = head->searchAndRemove(0.0);
	head = head->searchAndRemove(5.0);
	head = head->searchAndRemove(9.0);
	while(head != NULL)
	{
		double d;
		head = head->removeHead(d);
		cout<<"I have removed "<<d<<" from list"<<endl;
	}	
return 0;
}
		