public class main
{
	public static void main(String []args)
	{
		Subject s = new Subject();
		DivObserver d1 = new DivObserver(s, 3);
		ModObserver m1 = new ModObserver(s, 7);
		DivObserver d2 = new DivObserver(s, 4);
		ModObserver m2 = new ModObserver(s, 5);
		s.setA(12);
	}
}