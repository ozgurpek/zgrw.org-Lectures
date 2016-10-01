
class main
{
	public static void main(String []args)
	{
		Stack s = new Stack();
		for(int i = 0; i < 10; ++i)
		{
			System.out.println("Pushing " + i + " ");
			s.push(i);
		}
		System.out.println(" ");
		while(!s.isEmpty())
		{
			System.out.println("Popping " + s.pop() + " ");
		}
	}
}