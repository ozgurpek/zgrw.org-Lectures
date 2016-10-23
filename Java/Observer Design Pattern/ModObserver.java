public class ModObserver extends Observer
{
	public ModObserver(Subject s, int b)
	{
		super(s,b);
	}
	public void update()
	{
		System.out.println(s.getA() + " % "+ b);
	}
}