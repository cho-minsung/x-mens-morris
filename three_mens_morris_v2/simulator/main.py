import copy
import json
from rules import check_win
from bot import Bot

botZero = Bot(0)
botOne = Bot(1)

def board_to_key(board: list[list[int]]) -> str:
    # examples:
    # [[], []] -> "_"
    # [[0], []] -> "0_"
    # [[0, 1], [2]] -> "01_2"
    # [[0, 1], [2, 3]] -> "01_23"
    player_1 = "".join([str(i) for i in board[0]])
    player_2 = "".join([str(i) for i in board[1]])
    return f"{player_1}_{player_2}"

class Node:
    def __init__(
            self,
            layer: int = 0,
            board: str = "_",
            game_over: bool = False,
            wins: int = 0,
            visits: int = 0,
            children: list[str] = []
            ):
        self.layer = layer
        self.board = self.key_to_board(board)
        self.wins = wins
        self.visits = visits
        self.game_over = game_over
        self.children: list[str] = children
        # self.print()
    
    def key_to_board(self, key: str) -> list[list[int]]:
        # examples:
        # "_" -> [[], []]
        # "0_1" -> [[0], []]
        # "01_2" -> [[0, 1], [2]]
        # "01_23" -> [[0, 1], [2, 3]]

        # split the key into two parts
        player_1, player_2 = key.split("_")
        player_1 = [int(i) for i in player_1]
        player_2 = [int(i) for i in player_2]
        return [player_1, player_2]
    
    def print(self):
        print(f"{self.layer} {self.board} {self.game_over} {self.wins}")
        if len(self.children) == 0:
            return
        for child in self.children:
            print(child)
    
    def to_dict(self):
        return {
            "layer": self.layer,
            "game_over": self.game_over,
            "wins": self.wins,
            "visits": self.visits,
            "children": self.children
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
    
def get_nodes_from_file(file: str) -> list[Node]:
    with open(file, "r") as f:
        data = json.load(f)

    # convert key to board
    node_list: list[Node] = []
    for key, val in data.items():
        node_list.append(Node(
            layer = val["layer"],
            board = key,
            game_over = val["game_over"],
            wins = val["wins"],
            visits = val["visits"],
            children = val["children"]
            ))
    return node_list

def find_board_and_layer(node_list: list[Node], board: list[list[int]], layer: int):
    for node in node_list:
        if node.layer == layer and node.board == board:
            return node

def expand(node_list: list[Node]):
    new = 0
    for node in node_list:
        if node.layer >= 6:
            continue
        if node.layer % 2 == 0:
            new_boards = botZero.get_all_moves(node.board)
            for new_board in new_boards:
                if new_board not in [node.board for node in node_list]:
                    new_node = Node(node.layer + 1, board_to_key(new_board))
                    new_next_boards = botOne.get_all_moves(new_node.board)
                    for new_next_board in new_next_boards:
                        new_node.children.append(board_to_key(new_next_board))
                    if check_win(new_board, 0):
                        new_node.game_over = True
                    node_list.append(new_node)
                    new += 1
                    continue
        else:
            new_boards = botOne.get_all_moves(node.board)
            for new_board in new_boards:
                if new_board not in [node.board for node in node_list]:
                    new_node = Node(node.layer + 1, board_to_key(new_board))
                    new_next_boards = botZero.get_all_moves(new_node.board)
                    for new_next_board in new_next_boards:
                        new_node.children.append(board_to_key(new_next_board))
                    if check_win(new_board, 1):
                        new_node.game_over = True
                    node_list.append(new_node)
                    new += 1
                    continue
    print("new nodes", new)

nodes = get_nodes_from_file("empty_weight.json")

expand(nodes)

new_dict = {}
for node in nodes:
    new_dict[board_to_key(node.board)] = node.to_dict()

with open("expanded.json", "w") as f:
    json.dump(new_dict, f)

exit()

def play_game(base_weight: Node):
    # initialize the game data
    # to update win for layer and board match
    move_history = []
    state = 0
    game_over: bool = False
    winner = None
    layer = 1
    current_node = base_weight
    while game_over == False:
        board = current_node.board
        if state == 0:
            new_board = botZero.make_move(board)
            # check if the new board is already in the children
            while new_board not in [child.board for child in current_node.children]:
                new_board = botZero.make_move(board)
                new_node = Node(layer, new_board)
            if not check_win(new_board, 0):
                state = 1
                layer += 1
                # new_node.print()
                current_node.children.append(new_node)
                current_node = new_node
                continue
            new_node.game_over = True
            # new_node.print()
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
                # new_node.print()
                current_node.children.append(new_node)
                current_node = new_node
                continue
            new_node.game_over = True
            # new_node.print()
            winner = 1
            current_node.children.append(new_node)
            current_node = new_node
            game_over = True

    # winner is current state
    # print("winner:", winner)
    # print("layer:", layer)

    backpropagate(init_node, winner)

    return init_node, layer


# iterate through it and update wins and visits
def backpropagate(node: Node, winner: int):
    node.visits += 1
    if (winner == 0 and node.layer % 2 == 1) or (winner == 1 and node.layer % 2 == 0):
            node.wins += 1
    for child in node.children:
        backpropagate(child, winner)


def update_weight(base_weight: Node, new_game: Node):
    hit = False
    current_branch = base_weight
    current_node = new_game
    while 1:
        # print(f"current branch: {current_branch.layer} {current_branch.board}")
        # print(f"current node: {current_node.layer} {current_node.board}")
        if current_branch.layer == current_node.layer and current_branch.board == current_node.board:
            # print("current node children count", len(current_node.children))
            # print("same layer same board. moving to next layer")
            current_branch.visits += current_node.visits
            current_branch.wins += current_node.wins
            # current is only one line
            # print(len(current_node.children))
            if len(current_node.children) == 0:
                return False
            current_node = current_node.children[0]
            continue
        if current_branch.layer == current_node.layer - 1:
            # print("checking within children")
            if current_branch.children == []:
                current_branch.children.append(current_node)
                return True
            # check if current node board is in the children
            if current_node.board in [child.board for child in current_branch.children]:
                # print("found child")
                for child in current_branch.children:
                    if child.board == current_node.board:
                        current_branch = child
                        break
                continue
            # print("adding node to branch")
            current_branch.children.append(current_node)
            return True
    return hit

with open("empty_weight.json", "r") as f:
    weight = json.load(f)
    weight = Node().from_dict(weight)

# while 1:
#     hit_count = 0
#     for i in range(100):
#         # print("game", i)
#         new_game, layer = play_game()
#         if layer > 20:
#             continue
#         if update_weight(weight, new_game):
#             hit_count += 1
#         # weight.print()
#     if hit_count < 10:
#         break
#     print("hit count", hit_count, "out of 100")

# with open("weight.json", "w") as f:
#     json.dump(weight.to_dict(), f)