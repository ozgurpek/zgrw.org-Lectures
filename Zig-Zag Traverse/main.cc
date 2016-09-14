#include <iostream>
#include <stack>

using namespace std;

struct node
{
	node *left;
	node *right;
	int data;
};

void zigzag(struct node * root)
{
	stack<struct node*> s1;
	stack<struct node*> s2;
	s1.push(root);
	while(!s1.empty() || !s2.empty())
	{
		while(!s1.empty())
		{
			struct node *temp = s1.top();
			s1.pop();
			cout<<temp->data<<' ';
			if(temp->right != NULL)
				s2.push(temp->right);
			if(temp->left != NULL)
				s2.push(temp->left);
		}
		
		while(!s2.empty())
		{
			struct node *temp = s2.top();
			s2.pop();
			cout<<temp->data<<' ';
			if(temp->left != NULL)
				s1.push(temp->left);
			if(temp->right != NULL)
				s1.push(temp->right);
		}
	}
	
}

int main()
{
	struct node *root = new struct node;
	root->data = 1;
	root->left = new struct node;
	root->left->data = 2;
	root->right = new struct node;
	root->right->data = 3;
	root->left->left = new struct node;
	root->left->left->data = 4;
	root->left->right = new struct node;
	root->left->right->data = 5;
	
	root->right->left = new struct node;
	root->right->left->data = 6;
	root->right->right = new struct node;
	root->right->right->data = 7;
	zigzag(root);
}