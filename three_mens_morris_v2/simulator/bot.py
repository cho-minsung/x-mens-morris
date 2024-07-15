from rules import legal_move_tree, win_lines
import random

class Bot():
    def __init__(self, order: int):
        # 0 or 1
        self.order = order

    def add_new_piece(self, board: list[list[int]], index: int) -> list[list[int]]:
        new_board = board.copy()
        new_board[self.order].append(index)
        # sort the new board
        new_board[self.order].sort()
        return new_board
    
    def update_board(self, board: list[list[int]], old_move: int, new_move: int) -> list[list[int]]:
        new_board = board.copy()
        new_board[self.order].remove(old_move)
        new_board[self.order].append(new_move)
        # sort the new board
        new_board[self.order].sort()
        return new_board
    
    def get_early_win(self, board: list[list[int]]):
        possible_index = []
        for line in win_lines:
            temp_line = line.copy()
            for index in board[self.order]:
                if index in temp_line:
                    temp_line.remove(index)
            
            if len(temp_line) != 1:
                return None
            
            enemy_order = 1 if self.order == 0 else 0
            for index in board[enemy_order]:
                if index == temp_line[0]:
                    return None
            
            possible_index.append(temp_line[0])

        if len(possible_index) == 0:
            return None
        
        new_index = random.choice(possible_index)
        return self.add_new_piece(board, new_index)

    def get_winning_index(self, board: list[list[int]]):
        possible_moves = []
        for line in win_lines:
            temp_line = line.copy()
            movable_index = board[self.order].copy()
            for index in board[self.order]:
                if index in temp_line:
                    temp_line.remove(index)
                    movable_index.remove(index)
            
            if len(temp_line) != 1:
                return None
            
            enemy_order = 1 if self.order == 0 else 0
            for index in board[enemy_order]:
                if index == temp_line[0]:
                    return None
            
            # check if remaining index can be moved there
            if temp_line[0] in legal_move_tree[str(movable_index[0])]:
                possible_moves.append((index, temp_line[0]))

        if len(possible_moves) == 0:
            return None
        
        old_move, new_move = random.choice(possible_moves)
        return self.update_board(board, old_move, new_move)
    
    def make_move(self, board: list[list[int]]) -> list[list[int]]:
        # get a move
        if len(board[self.order]) < 3:
            win_index = self.get_early_win(board)
            if win_index is not None:
                return win_index
            return self.make_random_piece(board)
        else:
            new_board = self.get_winning_index(board)
            if new_board is not None:
                return new_board
            else:
                return self.make_random_move(board)

    def make_random_move(self, board: list[list[int]]) -> list[list[int]]:
        # get all possible moves
        possible_moves: list[tuple[int, int]] = []
        # get enemy position
        enemy = 1 if self.order == 0 else 0
        enemy_positions: list[int] = board[enemy]
        # get all possible moves except to enemy positions and its own positions
        for old_index in board[self.order]:
            for new_index in legal_move_tree[str(old_index)]:
                if new_index not in enemy_positions and new_index not in board[self.order]:
                    possible_moves.append((old_index, new_index))
        
        random_move = random.choice(possible_moves)
        return self.update_board(board, random_move[0], random_move[1])
    
    def make_random_piece(self, board: list[list[int]]) -> list[list[int]]:
        # get a random move
        possible_index = [i for i in range(9) if i not in board[0] and i not in board[1]]
        random_piece = random.choice(possible_index)
        return self.add_new_piece(board, random_piece)
        

