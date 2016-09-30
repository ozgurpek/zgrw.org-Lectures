#ifndef CLASSES_H
#define CLASSES_H

#include <vector>

class Observer;

class Subject
{
private:
	int val;
	std::vector<Observer *> observers;
	void notify();
public:
	Subject();
	void setVal(int v);
	int getVal();
	void subscribe(Observer * o);
};

class Observer
{
protected:
	int denom;
	Subject *model;
public:
	explicit Observer(Subject *model, int denom);
	virtual void update() = 0;
	Subject *getSubject();
};

class divObserver : public Observer
{
public:
	divObserver(Subject *model, int denom);
	void update();
};

class modObserver : public Observer
{
public:
	modObserver(Subject *model, int denom);
	void update();
};


#endif