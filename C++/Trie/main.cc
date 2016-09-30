#include <iostream>
#include <vector>
#include <string>
#include <ctime>
#include <algorithm>
#include <cstdlib>

using namespace std;

#define SIZE 26

typedef struct node
{
	node()
	:child(SIZE, NULL)
	{
	}
	char val;
	vector<struct node *> child;
}node;


bool insertTrie(node *head, const string &txt)
{
	node *currNode = head;
	bool inserted = false;
	for(int i = 0; i < txt.size(); ++i)
	{
		int idx = txt[i] - 'a';
		if(currNode->child[idx] == NULL)
		{
			node *tmp = new node;
			tmp->val = txt[i];
			currNode->child[idx] = tmp;
			currNode = tmp;
			inserted = true;
		}
		else
		{
			currNode = currNode->child[idx];
		}
	}
	return inserted;
}


bool isExist(node *head, const string &txt)
{
	node *currNode = head;
	bool exists = true;
	for(int i = 0; i < txt.size(); ++i)
	{
		int idx = txt[i] - 'a';
		if(currNode->child[idx] == NULL)
		{
			exists = false;
			break;
		}
		currNode = currNode->child[idx];
	}
	return exists;
}

vector<string> guessStr(node *head, const string &txt)
{
	srand(time(0));
	node *currNode = head;
	vector<string> g;
	if(isExist(head, txt))
	{	
		for(int i = 0; i < txt.size(); ++i)
		{
			int idx = txt[i] - 'a';
			currNode = currNode->child[idx];
		}
		
		//Find all non-null Idx
		vector<int> idxs;
		for(int i = 0; i < SIZE; ++i)
		{
			if(currNode->child[i] != NULL)
			{
				idxs.push_back(i);
			}
		}
		
		//Select 3 random idxs
		node *next = NULL;
		string str;
		bool found = false;
		for(int k = 0; k < 3; ++k)
		{
			str = txt;
			int idx = rand() % idxs.size();
			idx = idxs[idx];
			next = currNode->child[idx];
			str += next->val;
			while(next != NULL)
			{
				found = false;
				for(int i = 0; i < SIZE; ++i)
				{
					if(next->child[i] != NULL)
					{
						next = next->child[i];
						found = true;
						str += next->val;
					}
				}
				if(!found)
				{
					next = NULL;
				}
			}
			if(find(g.begin(), g.end(), str) == g.end())
			{
				g.push_back(str);
			}
		}
	}
	return g;
}

int main()
{
	node *head = new node;
	insertTrie(head, "test");
	cout<<isExist(head, "test")<<endl;
	insertTrie(head, "tabak");
	insertTrie(head, "turta");
	insertTrie(head, "tup");
	cout<<"Second insert "<<insertTrie(head, "tup")<<endl;
	insertTrie(head, "araba");
	insertTrie(head, "bebek");
	insertTrie(head, "ceket");
	insertTrie(head, "deniz");
	insertTrie(head, "etek");
	insertTrie(head, "fasulye");
	insertTrie(head, "gezi");
	insertTrie(head, "harita");
	insertTrie(head, "iplik");
	insertTrie(head, "jakuzi");
	insertTrie(head, "kalem");
	insertTrie(head, "lale");
	insertTrie(head, "muzik");
	insertTrie(head, "nese");
	insertTrie(head, "okul");
	insertTrie(head, "parke");
	insertTrie(head, "quadro");
	insertTrie(head, "rize");
	insertTrie(head, "sokak");
	insertTrie(head, "tarak");
	insertTrie(head, "tarti");
	insertTrie(head, "taka");
	insertTrie(head, "tasa");
	insertTrie(head, "taki");
	insertTrie(head, "tatil");
	insertTrie(head, "ucak");
	insertTrie(head, "veli");
	insertTrie(head, "win");
	insertTrie(head, "xavier");
	insertTrie(head, "yeni");
	insertTrie(head, "zeki");
	vector<string> g = guessStr(head, "ta"); //word guesses starting with ta
	for(auto s : g)
	{
		cout<<s<<endl;
	}
}