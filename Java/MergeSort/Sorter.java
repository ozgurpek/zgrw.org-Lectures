import java.util.*;


class Sorter
{
	public static void sort(ArrayList<Integer> arr)
	{
		if(arr.size() <= 1)
			return;
		int end = arr.size();
		int mid = (end + 0) / 2;
		ArrayList<Integer> larr = new ArrayList<Integer>(arr.subList(0, mid));
		ArrayList<Integer> rarr = new ArrayList<Integer>(arr.subList(mid, end));
		sort(larr);
		sort(rarr);
		int i = 0, j = 0;
		ArrayList<Integer> narr = new ArrayList<Integer>();
		while(i < larr.size() && j < rarr.size())
		{
			if( larr.get(i) <= rarr.get(j) )
			{
				narr.add(larr.get(i));
				++i;
			}
			else if( larr.get(i) >= rarr.get(j) )
			{
				narr.add(rarr.get(j));
				++j;
			}
		}
		
		while(i < larr.size())
		{
			narr.add(larr.get(i));
			++i;
		}
		
		while(j < rarr.size())
		{
			narr.add(rarr.get(j));
			++j;
		}
		
		for( i = 0; i < narr.size(); ++i)
		{
			arr.set(i, narr.get(i));
		}
	}
}