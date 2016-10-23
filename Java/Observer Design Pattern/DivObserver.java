public class DivObserver extends Observer
{
	public DivObserver(Subject s, int b)
	{
		super(s,b);
	}
	public void update()
	{
		System.out.println(s.getA() + " / "+ b);
	}
}