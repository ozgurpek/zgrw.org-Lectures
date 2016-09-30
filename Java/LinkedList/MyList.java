

class MyList
{
	private class Node
	{
		private Node next;
		private int data;
		
		public Node(final int d)
		{
			next = null;
			data = d;
		}
	}
	
	private Node head;
	
	public void insert(final int d)
	{
		if(head == null)
		{
			head = new Node(d);
		}
		else
		{
			Node ref = head;
			while(ref.next != null)
			{
				ref = ref.next;
			}
			ref.next = new Node(d);
		}
	}
	
	public void removeHead()
	{
		if(head != null)
		{
			head = head.next;
		}
	}
	
	public void removeLast()
	{
		Node ref;
		Node pref = null;
		if(head != null)
		{
			ref = head;
			while(ref.next != null)
			{
				pref = ref;
				ref = ref.next;
			}
			if(pref == null)
			{
				head = null;
			}
			else
			{
				pref.next = null;
			}
		}
	}
	
	public void searchAndRemove(final int d)
	{
		Node ref = head;
		Node pref = null;
		while(ref != null)
		{
			if(d == ref.data )
			{
				if(null == pref)
				{
					this.removeHead();
					break;
				}
				else
				{
					pref.next = ref.next;
					break;
				}
			}
			pref = ref;
			ref = ref.next;
		}
	}
	
	void print()
	{
		Node ref = head;
		while(ref != null)
		{
			System.out.print(ref.data + " ");
			ref = ref.next;
		}
		System.out.println("");
	}
}