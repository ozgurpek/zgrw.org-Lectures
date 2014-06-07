#include <iostream>

using namespace std;

class list
{
public:
	list * insert (const int &d);
	list * searchAndRemove(const int &d);
	list * removeHead (int &d);
	list * removeLast(int &d);
	friend list * init(list *, int d);
private:
	list();
	list *next;
	int data;
};

list::list()
{
}

list * init(list *head, int d)
{
	if(head == NULL)
	{
		head = new list;
		head -> data = d;
		head -> next = NULL;
	}
	return head;
}

list * list::insert (const int &d)
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

list * list::searchAndRemove(const int &d)
{
	list *ptr = this;
	list *pptr = NULL;
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

list * list::removeHead (int &d)
{
	list *ptr = this;
	if(ptr != NULL)
	{
		d = ptr->data;
		ptr = ptr-> next;
		delete this;
	}
	return ptr;
}

list * list::removeLast(int &d)
{
	list *ptr = this;
	list *pptr = NULL;
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
	list *head = NULL;
	head = init(head, 0);
	int i = 1;
	for(; i < 10; i++ )
	{
		cout <<"I am adding "<<i<<" to list"<<endl;
		head = head->insert(i);
	}
	head = head->searchAndRemove(0);
	head = head->searchAndRemove(5);
	head = head->searchAndRemove(9);
	while(head != NULL)
	{
		int d;
		head = head->removeHead(d);
		cout<<"I have removed "<<d<<" from list"<<endl;
	}	
return 0;
}
		