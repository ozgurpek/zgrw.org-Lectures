import java.util.*;

public class main
{
	public static void main(String... args)
	{
		Integer[] arr = new Integer[25];
		Random rnd = new Random();
		for(int i = 0; i < 25; ++i)
		{
			int d = rnd.nextInt(100) + 1;
			arr[i] = d;
			System.out.print(d + " ");
		}
		System.out.println(" ");
		System.out.println(" ");
		Sorter.sort(arr);
		for(Integer a : arr)
		{
			System.out.print(a + " ");
		}
		System.out.println(" ");

	}
}