import java.util.ArrayList;
class Stack
{
	private ArrayList<Integer> data;
	
	public Stack()
	{
		data = new ArrayList<Integer>();
	}
	
	public void push(final int d)
	{
		data.add(d);
	}
	
	public int pop()
	{
		int sz = data.size();
		int retVal = data.remove(sz - 1);
		return retVal;
	}
	
	public boolean isEmpty()
	{
		return data.isEmpty();
	}
}