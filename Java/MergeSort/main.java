import java.util.*;

public class main
{
	public static void main(String []args)
	{
		ArrayList<Integer> arr = new ArrayList<Integer>();
		Random ran = new Random();
		for(int i = 0; i < 25; ++i)
		{
			int d = ran.nextInt(100) + 1;
			arr.add(d);
			System.out.print(d + " ");
		}
		System.out.println(" ");
		System.out.println(" ");
		Sorter.sort(arr);
		for(int a : arr)
		{
			System.out.print(a + " ");
		}
		System.out.println(" ");
	}
}