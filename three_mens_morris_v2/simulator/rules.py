from collections import Counter

# initialize the winning moves
legal_move_tree = { # old move: [new moves]
    "0": [1, 3, 4],
    "1": [0, 2, 4],
    "2": [1, 4, 5],
    "3": [0, 4, 6],
    "4": [0, 1, 2, 3, 5, 6, 7, 8],
    "5": [2, 4, 8],
    "6": [3, 4, 7],
    "7": [4, 6, 8],
    "8": [4, 5, 7],
}

# initialize the winning lines, sorted
win_lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
]

def validate_board(board: list[list[int]]) -> bool:
    # assert only two players are in the board
    if len(board) != 2:
        return False

    # check if the board has only 1s, 2s and 0s
    for player in board:
        for piece in player:
            if piece not in range(0, 9):
                return False

    # check if the each player has no more than 3 pieces
    if len(board[0]) > 3 or len(board[1]) > 3:
        return False

    # check if player count is greater than two
    if abs(len(board[0]) - len(board[1])) >= 2:
        return False

    return True

def validate_move(board: list[list[int]], old_move: int, new_move: int, player: int) -> bool:
    if not validate_board(board):
        return False
    
    # check if the move is valid
    if not 0 <= old_move < 9 or not 0 <= new_move < 9:
        return False

    # check if the player is valid
    if not player in [1, 2]:
        return False
    
    player -= 1

    # reject if old move index is not the player's
    if old_move not in board[player]:
        return False
    
    # check if the move is legal
    if new_move not in legal_move_tree[str(old_move)]:
        return False
    
    return True
    

def check_win(board: list[list[int]], player: int) -> bool:
    if not validate_board(board):
        return False
    
    if player not in [0, 1]:
        return False

    # check if the player has won
    for line in win_lines:
        if Counter(board[player]) == Counter(line):
            return True

    return False