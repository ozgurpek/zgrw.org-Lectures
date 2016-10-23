import java.util.*;

class Sorter
{
	private  Integer []arr;
	public  void sort(Integer[] array)
	{
		arr = array;
		sort(0, arr.length);
	}
	private  void sort(int beg, int end)
	{
		if(end > beg + 1)
		{
			Random rnd = new Random();
			int pv = rnd.nextInt(end - beg) + beg;
			for(int i = beg; i < end; ++i)
			{
				if( i < pv && arr[i] > arr[pv] )
				{
					swap(i, pv);
					pv = i;
				}
				else if ( i > pv && arr[i] < arr[pv] )
				{
					if(i == pv + 1)
					{
						swap(i, pv);
						pv = i;
					}
					else
					{
						swap(i, pv);
						++pv;
						swap(i, pv);
					}
				}
			}
			sort(beg, pv);
			sort(pv, end);
		}

	}
	
	public  void swap ( Integer a, Integer b)
	{
		Integer temp = new Integer(arr[a].intValue());
		arr[a] = arr[b];
		arr[b] = temp;
	}
}