import java.util.ArrayList;


public class Subject
{
	private int a;
	ArrayList<Observer> obj;
	public Subject()
	{
		obj = new ArrayList<Observer>();
	}
	public void setA(int b)
	{
		a = b;
		notifyObservers();
	}
	public void register(Observer o)
	{
		obj.add(o);
	}
	public int getA()
	{
		return a;
	}
	private void notifyObservers()
	{
		for(Observer o : obj)
		{
			o.update();
		}
	}
}