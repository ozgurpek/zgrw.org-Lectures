public class Observer
{
	protected Subject s;
	protected int b;
	public Observer(Subject s, int b)
	{
		this.s = s;
		this.b = b;
		s.register(this);
	}
	public void update()
	{
		System.out.println("I am observer a = "+ s.getA() + ", b = "+ b);
	}
}