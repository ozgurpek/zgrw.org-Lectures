
public class maxseq
{
	public static void main(String []args)
	{
		int []arr = {8,-7,5,3,87,1,-156,9,0};
		maxseq mxs = new maxseq();
		System.out.println("Max sum is: " + mxs.maxSum(arr));
		int res = mxs.maxSeq(arr);
		System.out.println("Max seq is: " + res);
	}
	
	public int maxSeq(final int []arr)
	{
		int sumSoFar = 0;
		int sumMax = 0;
		for(int i = 0; i < arr.length; ++i)
		{
			sumSoFar = Math.max(0, sumSoFar + arr[i]);
			sumMax = Math.max(sumMax, sumSoFar);
		}
		return sumMax;
	}

	public int maxSum(final int []arr)
	{
		int sum = 0;
		for(int value : arr)
		{
			if(value > 0)
				sum += value;
		}
		return sum;
	}
}
