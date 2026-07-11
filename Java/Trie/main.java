import java.util.ArrayList;
import java.util.List;

class main {
    private static class Node {
        final Node[] children = new Node[26];
        boolean word;
    }
    private static class Trie {
        private final Node root = new Node();
        boolean insert(String word) {
            Node current = root;
            for (char character : word.toCharArray()) {
                int index = character - 'a';
                if (current.children[index] == null) current.children[index] = new Node();
                current = current.children[index];
            }
            boolean inserted = !current.word; current.word = true; return inserted;
        }
        boolean exists(String word) {
            return nodeFor(word) != null;
        }
        List<String> guesses(String prefix) {
            List<String> matches = new ArrayList<String>(); Node node = nodeFor(prefix);
            if (node != null) collect(node, new StringBuilder(prefix), matches); return matches;
        }
        private Node nodeFor(String text) {
            Node current = root;
            for (char character : text.toCharArray()) { current = current.children[character - 'a']; if (current == null) return null; }
            return current;
        }
        private void collect(Node node, StringBuilder text, List<String> matches) {
            if (node.word) matches.add(text.toString());
            for (int i = 0; i < 26; i++) if (node.children[i] != null) { text.append((char) ('a' + i)); collect(node.children[i], text, matches); text.deleteCharAt(text.length() - 1); }
        }
    }
    public static void main(String[] args) {
        Trie trie = new Trie();
        for (String word : new String[] {"test", "tabak", "tarak", "tarti", "tasa", "tatil", "tup"}) trie.insert(word);
        System.out.println(trie.exists("test"));
        System.out.println("Second insert " + trie.insert("tup"));
        for (String guess : trie.guesses("ta")) System.out.println(guess);
    }
}
