class main
{
	public static void main(String []args)
	{
		MyList lst = new MyList();
		lst.insert(3);
		lst.print();
		lst.insert(35);
		lst.print();
		lst.insert(6);
		lst.print();
		lst.removeHead();
		lst.print();
		lst.insert(9);
		lst.insert(8);
		lst.insert(7);
		lst.removeLast();
		lst.print();
		lst.removeLast();
		lst.print();
		lst.insert(11);
		lst.insert(8);
		lst.insert(7);
		lst.insert(9);
		lst.insert(81);
		lst.insert(71);
		lst.print();
		lst.searchAndRemove(71);
		lst.print();
		lst.searchAndRemove(7);
		lst.print();
	}
}