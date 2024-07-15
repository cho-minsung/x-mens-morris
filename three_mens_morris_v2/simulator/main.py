import copy
import json
import queue
from rules import check_win
from bot import Bot

# initialize the game data
board: list[list[int]] = [[], []]
state = 0
game_over: bool = False
winner = None

botZero = Bot(0)
botOne = Bot(1)

class Node:
    def __init__(self, layer: int = 0, board: list[list[int]] = [[], []]):
        self.layer = layer
        self.board = board
        self.wins = 0
        self.visits = 0
        self.game_over = False
        self.children: list['Node'] = []
    
    def print(self):
        print(f"{self.layer} {self.board} {self.game_over} {self.wins}")
        for child in self.children:
            if child is not None:
                child.print()

    def from_dict(self, data: dict):
        self.layer = data["layer"]
        self.board = data["board"]
        self.game_over = data["game_over"]
        self.wins = data.get("wins", 0)
        self.visits = data.get("visits", 0)
        self.children = [Node().from_dict(child) for child in data.get("children", [])]
        return self
    
    def to_dict(self):
        return {
            "layer": self.layer,
            "board": self.board,
            "game_over": self.game_over,
            "wins": self.wins,
            "visits": self.visits,
            "children": [child.to_dict() for child in self.children]
        }

def render_board(board: list[list[int]]):
    # render the board
    flat_board = [0 for _ in range(9)]
    for i in board[0]:
        flat_board[i] = 1
    for i in board[1]:
        flat_board[i] = 2
    # print three rows
    for i in range(3):
        print(flat_board[i * 3], flat_board[i * 3 + 1], flat_board[i * 3 + 2])

with open("weight1.json", "r") as f:
    weight = json.load(f)
    weight = Node().from_dict(weight)
    weight.print()

layer = 1
init_node = Node(0, copy.deepcopy(board))
current_node = init_node
while game_over == False:
    if state == 0:
        move = botZero.make_move(board) # this updates the board
        new_node = Node(layer, copy.deepcopy(board))
        if not check_win(board, 0):
            state = 1
            layer += 1
            new_node.print()
            current_node.children.append(new_node)
            current_node = new_node
            continue
        new_node.game_over = True
        new_node.print()
        winner = 0
        current_node.children.append(new_node)
        current_node = new_node
        game_over = True
    else:
        board = botOne.make_move(board)
        new_node = Node(layer, copy.deepcopy(board))
        if not check_win(board, 1):
            state = 0
            layer += 1
            new_node.print()
            current_node.children.append(new_node)
            current_node = new_node
            continue
        new_node.game_over = True
        new_node.print()
        winner = 1
        current_node.children.append(new_node)
        current_node = new_node
        game_over = True

# winner is current state
# bot zero won
print("winner:", winner)
print("layer:", layer)

# with open("weight1.json", "w") as f:
#     json.dump(init_tree.to_dict(), f)

# won_nodes = []
# iterate through it and update wins and visits
def backpropagate(node: Node, winner: int):
    if (winner == 0 and node.layer % 2 == 1) or (winner == 1 and node.layer % 2 == 0):
            node.wins += 1
    for child in node.children:
        backpropagate(child, winner)

backpropagate(init_node, winner)

init_node.print()

current_branch = weight
current_node = init_node
while current_node.layer != []:
    print(f"current branch: {current_branch.layer} {current_branch.board}")
    print(f"current node: {current_node.layer} {current_node.board}")
    if current_branch.layer == current_node.layer and current_branch.board == current_node.board and current_branch.children != []:
        print("same layer same board. moving to next layer")
        current_branch.visits += current_node.visits
        current_branch.wins += current_node.wins
        current_node = current_node.children[0]
        continue
    if current_branch.layer == current_node.layer - 1 and current_branch.children == []:
        print("adding node to branch")
        current_branch.children.append(current_node)
        break
    if current_branch.layer == current_node.layer - 1:
        print(f"current branch children: {len(current_branch.children)}")
        # check if current node board is in the children
        for child in current_branch.children:
            if child.board == current_node.board:
                current_branch = child
                continue
        current_branch.children.append(current_node)
        break

weight.print()

with open("weight.json", "w") as f:
    json.dump(weight.to_dict(), f)