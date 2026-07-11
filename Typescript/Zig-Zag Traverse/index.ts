class TreeNode {
  public left?: TreeNode;
  public right?: TreeNode;
  public constructor(public value: number) {}
}

function zigZag(root: TreeNode): number[] {
  let current: TreeNode[] = [root],
    next: TreeNode[] = [],
    leftToRight = true;
  const result: number[] = [];
  while (current.length) {
    const node = current.pop()!;
    result.push(node.value);
    if (leftToRight) {
      if (node.right) next.push(node.right);
      if (node.left) next.push(node.left);
    } else {
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    if (!current.length) {
      current = next;
      next = [];
      leftToRight = !leftToRight;
    }
  }
  return result;
}

const root = new TreeNode(1);
root.left = new TreeNode(2);
root.right = new TreeNode(3);
root.left.left = new TreeNode(4);
root.left.right = new TreeNode(5);
root.right.left = new TreeNode(6);
root.right.right = new TreeNode(7);
console.log(zigZag(root).join(" "));
