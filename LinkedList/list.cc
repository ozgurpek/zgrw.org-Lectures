#include <iostream>

using namespace std;

typedef struct list
{
	list *next;
	int data;
}list;

list* init (list* head)
{
	head = NULL;
	return head;
}

list * insert (list *head, int d)
{
	if(head == NULL)
	{
		head = new list;
		head->data = d;
		head->next = NULL;
	}
	else
	{
		head-> next = insert(head->next, d);
	}
	return head;
}

list * searchAndRemove(list *head, int d)
{
	list *ptr = head;
	list *pptr = NULL;
	while(ptr != NULL)
	{
		if(ptr->data == d)
		{
			if(pptr != NULL)
			{
				pptr->next = ptr->next;
			}
			else
			{
				head = ptr->next;
			}
			delete ptr;
			return head;
		}
		pptr = ptr;
		ptr = ptr->next;
	}
	return head;
}

list * removeHead (list *head, int &d)
{
	list *ptr = head;
	if(head != NULL)
	{
		d = head->data;
		head = head -> next;
		delete ptr;
	}
	return head;
}

list * removeLast(list * head, int &d)
{
	list *ptr = head;
	list *pptr = NULL;
	if(head != NULL)
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
			return head;
		}
		else
		{
			delete ptr;
			return NULL;
		}
	}
	return head;
}

int main()
{
	list *head;
	head = init(head);
	int i = 0;
	for(; i < 10; i++ )
	{
		cout <<"I am adding "<<i<<" to list"<<endl;
		head = insert(head, i);
	}
	head = searchAndRemove(head, 0);
	head = searchAndRemove(head, 5);
	head = searchAndRemove(head, 9);
	while(head != NULL)
	{
		int d;
		head = removeHead(head, d);
		cout<<"I have removed "<<d<<" from list"<<endl;
	}	
return 0;
}
		