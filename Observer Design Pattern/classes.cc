#include <iostream>
#include "classes.h"

using namespace std;



Subject::Subject()
:val(1)
{
}

void Subject::setVal(int v)
{
	val = v;
	notify();
}

int Subject::getVal()
{
	return val;
}

void Subject::subscribe(Observer * o)
{
	observers.push_back(o);
}

void Subject::notify()
{
	for(auto o : observers)
	{
		o->update();
	}
}

Observer::Observer(Subject *model, int denom)
:model(model), denom(denom)
{
	model->subscribe(this);
}

Subject *Observer::getSubject()
{
	return model;
}


divObserver::divObserver(Subject *model, int denom)
:Observer(model, denom)
{
}

void divObserver::update()
{
	int v = getSubject()->getVal();
	cout<<v<<" / "<<denom<<" = "<<v/denom<<endl;
}


modObserver::modObserver(Subject *model, int denom)
:Observer(model, denom)
{
}

void modObserver::update()
{
	int v = getSubject()->getVal();
	cout<<v<<" % "<<denom<<" = "<<v%denom<<endl;
}
