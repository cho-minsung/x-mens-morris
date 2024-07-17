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
        # if there is a dash then it is the 9th layer
        if key[-1] == "-":
            key = key[:-1]

        # split the key into two parts
        player_1, player_2 = key.split("_")
        player_1 = [int(i) for i in player_1]
        player_2 = [int(i) for i in player_2]
        return [player_1, player_2]
    
    def print(self):
        if self.layer >= 8 and self.layer % 2 == 0:
            board  = f"{self.board}-"
        else:
            board = self.board
        print(f"{self.layer} {board} {self.game_over} {self.wins}")
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

def get_dict_from_node_list(node_list: list[Node]) -> dict:
    new_dict = {}
    for node in node_list:
        if node.layer == 8:
            new_dict[str(board_to_key(node.board)+"-")] = node.to_dict()
        else:
            new_dict[board_to_key(node.board)] = node.to_dict()
    return new_dict

def find_board_and_layer(node_list: list[Node], board: list[list[int]], layer: int):
    for node in node_list:
        if node.layer == layer and node.board == board:
            return node

def expand(node_list: list[Node]):
    new = 0
    for node in node_list:
        if node.layer >= 7:
            continue
        if node.layer % 2 == 0:
            new_boards = botZero.get_all_moves(node.board)
            for new_board in new_boards:
                if new_board not in [node.board for node in node_list]:
                    new_node = Node(node.layer + 1, board_to_key(new_board))
                    new_next_boards = botOne.get_all_moves(new_board)
                    new_node.children = [board_to_key(new_next_board) for new_next_board in new_next_boards]
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
                    new_next_boards = botZero.get_all_moves(new_board)
                    new_node.children = [board_to_key(new_next_board) for new_next_board in new_next_boards]
                    if check_win(new_board, 1):
                        new_node.game_over = True
                    node_list.append(new_node)
                    new += 1
                    continue

    # additionally add 8th layer
    for node in [node for node in node_list if node.layer == 6]:
        new_boards = botOne.get_all_moves(node.board)
        for new_board in new_boards:
            new_node = Node(8, board_to_key(new_board) + "-")
            if check_win(new_board, 1):
                new_node.game_over = True
            node_list.append(new_node)
            new += 1
        
            new_next_boards = botZero.get_all_moves(new_board)
            new_node.children = [board_to_key(new_next_board) for new_next_board in new_next_boards]
                    
    print("new nodes", new)

node_list = get_nodes_from_file("weight.json")

# expand(node_list)
# for node in node_list:
#     node.print()
node_dict = get_dict_from_node_list(node_list)

# with open("expanded1.json", "w") as f:
#     json.dump(node_dict, f, indent=4)

def play_game() -> tuple[list[Node], int]:
    # initialize the game data
    # to update win for layer and board match
    node_history: list[Node] = []
    layer = 0
    base_node = Node(layer, "_")
    current_node = base_node
    node_history.append(base_node)
    while 1:
        board = current_node.board
        if current_node.layer % 2 == 0:
            layer += 1
            new_board = botZero.make_move(board)
            if new_board == []:
                current_node.game_over = True
                return node_history, 1
            new_node = Node(layer, board_to_key(new_board))
            node_history.append(new_node)
            if check_win(new_board, 0):
                new_node.game_over = True
                return node_history, 0
            current_node = new_node
        else:
            layer += 1
            new_board = botOne.make_move(board)
            if new_board == []:
                current_node.game_over = True
                return node_history, 0
            new_node = Node(layer, board_to_key(new_board))
            node_history.append(new_node)
            if check_win(board, 1):
                new_node.game_over = True
                return node_history, 1
            current_node = new_node

    return node_history, 0

def back_propagate(node_history: list[Node], winner: int):
    for node in node_history:
        node.visits += 1
        if winner == node.layer % 2:
            node.wins += 1

def update_weight(nodes: dict, node_history: list[Node]):
    for node in node_history:
        key = board_to_key(node.board)
        if node.layer >= 8 and node.layer % 2 == 0:
            key += "-"
        if key[-1] == "-" and node.layer % 2 == 1:
            key = key[:-1]
        nodes[key]["wins"] += 1
        nodes[key]["visits"] += 1
        nodes[key]["wins"] += node.wins
        nodes[key]["visits"] += node.visits

for i in range(800000):
    node_history, winner = play_game()
    back_propagate(node_history, winner)
    update_weight(node_dict, node_history)

with open("weight.json", "w") as f:
    json.dump(node_dict, f, indent=2)

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