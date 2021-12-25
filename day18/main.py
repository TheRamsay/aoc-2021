from typing import List, Union


x = eval("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")

class Node:
    def __init__(self, parent: Union["Node", None]) -> None:
        self.parent: Union[None, "Node"] = parent
        self.left: Union[int, Node, None] = None
        self.right: Union[int, Node, None] = None



def create_tree(arr: list, parent: Union[None, Node]):

    node = Node(parent)

    if isinstance(arr[0], int):
        node.left = arr[0]
    else:
        node.left = create_tree(arr[0], node)

    if isinstance(arr[1], int):
        node.right = arr[1]
    else:
        node.right = create_tree(arr[1], node)
    
    return node

root = create_tree(x, None)

print(root.left.left.left)
print(root.right)

