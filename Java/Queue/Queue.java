
class Queue
{
	private int []list;
	private int head;
	private int tail;
	
	public Queue(final int size)
	{
		list = new int[size];
		head = -1;
		tail = -1;
	}
	
	public boolean isEmpty()
	{
		return -1 == head;
	}
	
	public boolean isFull()
	{
		if((tail == list.length - 1 && head == 0) || (tail + 1 == head && tail != -1))
			return true;
		return false;
	}
	
	public void insert(final int d)
	{
		if(!isFull())
		{
			if(isEmpty())
			{
				tail = head = 0;
			}
			else
			{
				++tail;
				if(tail == list.length)
					tail = 0;
			}
			list[tail] = d;
		}
	}
	public int remove()
	{
		int retVal = -1;
		if(!isEmpty())
		{
			retVal = list[head];
			++head;
			if(head == tail + 1)
				head = -1;
			else if(head == list.length)
				head = 0;
		}
		return retVal;
	}
}