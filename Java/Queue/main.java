class main
{
	public static void main(String []args)
	{
		Queue q = new Queue(10);
		int i = 9;
		while(!q.isFull())
		{
			System.out.println("I am adding " + i + " to the queue");
			q.insert(i--);
		}
		
		while(!q.isEmpty())
		{
			int val = q.remove();
			System.out.println("I have removing " + val + " from the queue");
		}
	}
}