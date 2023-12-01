from typing import Optional


class Tree:
    def __init__(self, name, parent=None, children=None):    
        self.name: str = name
        self.parent: Optional["Tree"] = parent
        if children is None:
            children = []
        self.children: list["Tree"] = children

    def __contains__(self, item: str) -> bool:
        if self.name == item:
            return True
        return any(item in child for child in self.children)

    def add_child(self, child: "Tree") -> "Tree":
        if child.parent is not None:
            child.parent.remove_child(child)
        child.parent = self
        self.children.append(child)

    def remove_child(self, child: "Tree"):
        if child in self.children:
            self.children.remove(child)
        child.parent = None

    def path(self) -> str:
        if self.parent is not None:
            return f"{self.parent.path()},{self.name}"
        return self.name
    
    def leaf_nodes(self):
        if not self.children:
            yield self.path()
        for child in self.children:
            yield from child.leaf_nodes()


class Graph:
    def __init__(self, edges=None):
        self.edges: set[tuple[str, str]] = set()
        if edges is not None:
            self.edges.update(edges)
        self.vertices: set[str] = {
            v for edge in self.edges for v in edge
        }

    def __getitem__(self, item) -> set[tuple[str, str]]:
        return { start if item == end else end for start, end in self.edges if item in [start, end] }

    def remove(self, node: str) -> "Graph":
        self.vertices = self.vertices - set(node)
        self.edges = self.edges - {edge for edge in self.edges if node in edge}
        return self

    def copy(self) -> "Graph":
        return Graph(self.edges.copy())
    


def paths1(graph: Graph, node: str) -> Tree:
    tree = Tree(node)
    if node == 'end':
        return tree
    
    subgraph = graph.copy()
    if node.islower():
        subgraph.remove(node)
    for other in graph[node]:
        tree.add_child(paths1(subgraph, other))
    return tree
    

def paths2(graph: Graph, node: str, visited = False) -> Tree:
    tree = Tree(node)
    if node == 'end':
        return tree

    subgraph = graph.copy()
    visited_subgraph = graph.copy()
    if node == 'start':
        subgraph.remove(node)
        visited_subgraph.remove(node)
    elif node.islower():
        subgraph.remove(node)
    for other in graph[node]:
        tree.add_child(paths2(subgraph, other, visited))
        if node.islower() and not visited:
            tree.add_child(paths2(visited_subgraph, other, True))
    
    return tree


def solution1(graph):
    tree = paths1(graph, "start")
    leaves = sorted(node for node in tree.leaf_nodes() if node.endswith('end'))
    return len(leaves)


def solution2(graph):
    tree = paths2(graph, "start")
    leaves = sorted(set(node for node in tree.leaf_nodes() if node.endswith('end')))
    return len(leaves)

def main():
    with open("example.txt") as file:
        graph = Graph([tuple(line.strip().split('-')) for line in file])
    print(solution1(graph.copy()))
    print(solution2(graph.copy()))


if __name__ == "__main__":
    main()