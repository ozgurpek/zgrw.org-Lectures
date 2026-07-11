class TrieNode {
  public children: Array<TrieNode | undefined> = new Array(26);
  public word = false;
}
class Trie {
  private root = new TrieNode();
  public insert(word: string): boolean {
    let node = this.root,
      inserted = false;
    for (const char of word) {
      const index = char.charCodeAt(0) - 97;
      if (!node.children[index]) {
        node.children[index] = new TrieNode();
        inserted = true;
      }
      node = node.children[index]!;
    }
    node.word = true;
    return inserted;
  }
  public exists(text: string): boolean {
    return this.nodeFor(text) !== undefined;
  }
  public guesses(prefix: string): string[] {
    const node = this.nodeFor(prefix);
    const results: string[] = [];
    if (node) this.collect(node, prefix, results);
    return results;
  }
  private nodeFor(text: string): TrieNode | undefined {
    let node: TrieNode | undefined = this.root;
    for (const char of text)
      node = node && node.children[char.charCodeAt(0) - 97];
    return node;
  }
  private collect(node: TrieNode, text: string, results: string[]): void {
    if (node.word) results.push(text);
    node.children.forEach((child, index) => {
      if (child)
        this.collect(child, text + String.fromCharCode(97 + index), results);
    });
  }
}
const trie = new Trie();
[
  "test",
  "tabak",
  "turta",
  "tup",
  "tarak",
  "tarti",
  "taka",
  "tasa",
  "taki",
  "tatil",
].forEach((word) => trie.insert(word));
console.log(trie.exists("test"));
console.log("Second insert", trie.insert("tup"));
trie
  .guesses("ta")
  .slice(0, 3)
  .forEach((word) => console.log(word));
