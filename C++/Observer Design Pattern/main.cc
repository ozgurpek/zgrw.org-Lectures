#include <iostream>

#include "classes.h"

using namespace std;


int main()
{
	Subject s;
	divObserver d1(&s, 3);
	modObserver m1(&s, 7);
	divObserver d2(&s, 4);
	modObserver m2(&s, 5);
	s.setVal(10);
	
}