//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/oxbcE18TM
#include <memory> // unique_ptr
#include <iostream>
#include <string>
using std::unique_ptr; using std::cout;
template <typename K, typename D> class Tree; // forward declaration
template <typename K, typename D>
class Node {
    friend class Tree<K,D>; // allow access to private members
    K key;
    D data;
    unique_ptr<Node> left, right;
public:
    Node(const K& k, const D& d) : key(k), data(d) { }
};
template <typename K, typename D>
class Tree {
public:
    void insert(const K &key, const D& data);
    D* find(const K &key) { return findRec(key, root); }
private:
    D* findRec(const K &key, unique_ptr<Node<K,D>> &node);
    unique_ptr<Node<K,D>> root;
};
template <typename K, typename D>
void Tree<K,D>::insert(const K& key, const D& data) {
    auto *current = &root;
    while(*current) { // as long as unique_ptr contains something
        auto &node = *(current->get());
        if (key < node.key) {
            current = &node.left;
        } else if (node.key < key) {
            current = &node.right;
        }
    }
    *current = std::make_unique<Node<K,D>>(key,data);
};
template <typename K, typename D>
D* Tree<K,D>::findRec(const K& key, unique_ptr<Node<K,D>> &where) {
    if(!where)
        return nullptr;
    auto &node = *(where.get());
    if(key < node.key)
        return findRec(key, node.left);
    if(node.key < key)
        return findRec(key, node.right);
    return &node.data; // key == node.key
};
int main() {
    Tree<int,std::string> bt {};
    bt.insert(3, "three");
    bt.insert(2, "two");
    bt.insert(4, "four");
    auto where = bt.find(7);
    if(where==nullptr) cout<<"no 7\n";      // output: no 7
    where = bt.find(3);
    if(where!=nullptr) cout<<*where<<"\n";  // output: three
}