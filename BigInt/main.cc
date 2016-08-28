#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include<limits>

using namespace std;

typedef unsigned long long int ulli;

class bigInt
{
private:
	vector<int> val;
	int end;
	bigInt karatsuba(bigInt &lhs, bigInt &rhs);
	void partialCopy(bigInt &tar, const bigInt &rhs, int beg, int fin);
	bigInt operator*(int rhs);
public:
	bigInt();
	explicit bigInt(ulli a);
	void set(ulli a);
	bigInt operator+(bigInt &rhs);
	bigInt operator-(bigInt &rhs);
	bigInt &operator=(bigInt &rhs);
	bigInt operator*(bigInt &rhs);
	friend ostream &operator<<(ostream & out, const bigInt &rhs);
	void doubleMe();
	ulli getVal();
};

ostream &operator<<(ostream & out, const bigInt &rhs)
{
	for(int i = rhs.end; i >= 0 && rhs.val[i] >= 0; --i)
	{
		out<<rhs.val[i];
	}
	return out;
}

bigInt::bigInt():val(1000, 0), end(-1)
{
	
}

bigInt::bigInt(ulli a):val(1000, 0), end(-1)
{
	set(a);
	
}

void bigInt::set(ulli a)
{
	end = -1;
	int rem = 0;
	ulli div = a;
	while(div > 0)
	{
		rem = div % 10;
		div /= 10;
		++end;
		val[end] = rem;
	}
}

bigInt & bigInt::operator=(bigInt &rhs)
{
	this->end = rhs.end;
	for(int i = 0; i <= end; ++i)
	{
		val[i] = rhs.val[i];
	}
	return *this;
}

bigInt bigInt::operator+(bigInt &rhs)
{
	bigInt nVal;
	int mx = max(end, rhs.end);
	int over = 0;
	int temp = 0;
	nVal.end = mx;
	for(int i = 0; i <= mx; ++i )
	{
		temp = val[i] + rhs.val[i] + over;
		over = temp / 10;
		nVal.val[i] = temp % 10;
	}
	if(over != 0)
	{
		nVal.end++;
		nVal.val[nVal.end] = over;
	}
	return nVal;
}

void bigInt::doubleMe()
{
	bigInt rhs = karatsuba(*this, *this);
	end = rhs.end;
	for(int i = 0; i <= end; ++i)
	{
		val[i] = rhs.val[i];
	}
}

	bigInt bigInt::karatsuba(bigInt &lhs, bigInt &rhs)
	{
		
		
		bigInt t1, t2;
		t1.end = 0;
		t2.end = 0;
		int over = 0;
		int i, j;
		for(i = 0; i <= end; ++i)
		{
			t1.end = 0;
			for(j = 0; j <= rhs.end; ++j)
			{
				int a = val[i] * rhs.val[j];
				a += over;
				over = a / 10;
				a %= 10;
				t1.val[j] = a;
			}
			t1.end = j - 1;
			if(over != 0)
			{
				t1.val[j] = over;
				t1.end++;
			}
			t1 * i;
			bigInt s = t2 + t1;
			t2 = s;
		}
		return t2;
		//AB * CD = (B*D) + [(A+B)(C+D) - B*D - A*C]*10^n + A*C*10^2n
		/*if(lhs.end <= 10 || rhs.end <= 10)
		{
			return bigInt(lhs.getVal() * rhs.getVal());
		}
		int mx = max(lhs.end, rhs.end);
		int mid = mx / 2;
		bigInt low1, low2, high1, high2;
		partialCopy(low1, lhs, 0, mid);
		partialCopy(low2, rhs, 0, mid);
		partialCopy(high1, lhs, mid + 1, lhs.end);
		partialCopy(high1, lhs, mid + 1, rhs.end);
		bigInt bd = karatsuba(low1, low2);
		bigInt ac = karatsuba(high1, high2);
		bigInt sum1 = low1 + high1;
		bigInt sum2 = low2 + high2;
		bigInt abcd = karatsuba(sum1, sum2);
		bigInt middle = (abcd - bd - ac);
		middle*mid;
		ac * (2 * mid);
		return bd +  middle + ac; //integer multiplications actually are like 10 ^ mid...*/
	}


	bigInt bigInt::operator*(bigInt &rhs)
	{
		return karatsuba(*this, rhs);
	}

bigInt bigInt::operator*(int rhs) //this will only recive powers of 10
{
	end += rhs;
	while(rhs > 0)
	{
		val.insert(val.begin(), 0);
		rhs--;
	}
	return *this;
}

void bigInt::partialCopy(bigInt &tar, const bigInt &rhs, int beg, int fin)
{
	if(fin >= beg)
	{
		int end = fin - beg;
		for(int i = 0; i <= end; ++i, ++beg)
		{
			tar.val[i] = rhs.val[beg];
		}
	}
}

ulli bigInt::getVal()
{
	ulli val = 0;
	if(end <= 20)
	{
		for(int i = 0; i <= end; ++i)
		{
			val += this->val[i] * pow(10, i);
		}
	}
	return val;
}

bigInt bigInt::operator-(bigInt &rhs)
{
	bigInt nV;
	if(end < rhs.end)
		return bigInt(0);
	for(int i = 0; i <= end; ++i)
	{
		if(val[i] < rhs.val[i])
		{
			if( i+1 > end)
				return bigInt(0);
			val[i] += 10;
			val[i + 1]--;
		}
		nV.val[i] = val[i] - rhs.val[i];
	}
	int e = end;
	while(e >= 0)
	{
		if(nV.val[e] != 0)
		{
			nV.end = e;
			break;
		}
		--e;
	}
	return nV;
}

int main()
{
	bigInt b1(9);
	bigInt b2(3);
	cout<<b1<<" + "<<b2<<" = "<<b1 + b2<<endl;
	cout<<b1<<" - "<<b2<<" = "<<b1 - b2<<endl;
	/*cout<<b1<<" * 1"<<" = "<<b1 * 1<<endl;
	cout<<b1<<endl;
	b1.set(9);*/
	cout<<b1<<" * "<<b2<<" = "<<b1 * b2<<endl;
	cout<<b1.getVal()<<endl;
	b1.set(numeric_limits<ulli>::max());
	cout<<b1<<endl;
	cout<<b1<<" + "<<b2<<" = "<<b1 + b2<<endl;
	cout<<b1<<" * "<<b2<<" = "<<b1 * b2<<endl;
	b2.set(15);
	b2.doubleMe();
	cout<<b2<<endl;
	
}