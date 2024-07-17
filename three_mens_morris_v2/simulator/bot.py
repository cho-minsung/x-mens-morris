from rules import legal_move_tree, win_lines
import random
import copy

class Bot():
    def __init__(self, order: int):
        # 0 or 1
        self.order = order
    
    def get_all_winning_moves(self, board: list[list[int]]) -> list[tuple[int, int]]:
        # takes the board and returns all the possible winning moves in tuple
        possible_moves: list[tuple[int, int]] = []
        for line in win_lines:
            temp_line = line.copy()
            movable_index = board[self.order].copy()
            for index in board[self.order]:
                if index in temp_line:
                    temp_line.remove(index)
                    movable_index.remove(index)
            
            if len(temp_line) != 1:
                continue

            new_index = temp_line[0]
            old_index = movable_index[0]
            
            # check if new index is already occupied
            enemy_order = 1 if self.order == 0 else 0
            if new_index in board[enemy_order]:
                continue
            
            # check if new index can be moved from old index
            if new_index in legal_move_tree[str(old_index)]:
                possible_moves.append((old_index, new_index))

        if len(possible_moves) == 0:
            return []
    
        return possible_moves
    
    def get_all_winning_new_piece(self, board: list[list[int]]) -> list[int]:
        # takes the board and returns all the possible winning new piece
        possible_index: list[int] = []
        for line in win_lines:
            temp_line = line.copy()
            for index in board[self.order]:
                if index in temp_line:
                    temp_line.remove(index)
            
            if len(temp_line) != 1:
                continue
            
            new_index = temp_line[0]

            enemy_order = 1 if self.order == 0 else 0
            if new_index in board[enemy_order]:
                continue
            
            possible_index.append(new_index)

        if len(possible_index) == 0:
            return []
        
        return possible_index

    def add_new_piece(self, board: list[list[int]], index: int) -> list[list[int]]:
        new_board = copy.deepcopy(board)
        new_board[self.order].append(index)
        # sort the new board
        new_board[self.order].sort()
        return new_board
    
    def get_all_possible_new_pieces(self, board: list[list[int]]) -> list[int]:
        possible_index: list[int] = self.get_all_winning_new_piece(board)
        if len(possible_index) > 0:
            return possible_index
        
        opponent_turn = 1 if self.order == 0 else 0
        possible_index: list[int] = [i for i in range(9) if i not in board[self.order] and i not in board[opponent_turn]]
        return possible_index
    
    def get_all_possible_moves(self, board: list[list[int]]) -> list[tuple[int, int]]:
        possible_moves: list[tuple[int, int]] = self.get_all_winning_moves(board)
        # if winning move exists then only return those
        if len(possible_moves) > 0:
            return possible_moves
        
        # get enemy position
        enemy = 1 if self.order == 0 else 0
        enemy_positions: list[int] = board[enemy]
        # get all possible moves except to enemy positions and its own positions
        for old_index in board[self.order]:
            for new_index in legal_move_tree[str(old_index)]:
                if new_index not in enemy_positions and new_index not in board[self.order]:
                    possible_moves.append((old_index, new_index))
        return possible_moves
    
    def update_board(self, board: list[list[int]], old_move: int, new_move: int) -> list[list[int]]:
        new_board = copy.deepcopy(board)
        new_board[self.order].remove(old_move)
        new_board[self.order].append(new_move)
        # sort the new board
        new_board[self.order].sort()
        return new_board
    
    def make_move(self, board: list[list[int]]) -> list[list[int]]:
        # get a move
        if len(board[self.order]) < 3:
            return self.make_random_piece(board)
        else:
            return self.make_random_move(board)
        
    def get_all_moves(self, board: list[list[int]]) -> list[list[list[int]]]:
        # get all possible moves
        possible_boards: list[list[list[int]]] = []
        if len(board[self.order]) < 3:
            possible_new: list[int] = self.get_all_possible_new_pieces(board)
            if len(possible_new) == 0:
                return []
            else:
                possible_boards: list[list[list[int]]] = []
                for move in possible_new:
                    possible_boards.append(self.add_new_piece(board, move))
        else:
            possible_moves: list[tuple[int, int]] = self.get_all_possible_moves(board)
            if len(possible_moves) == 0:
                return []
            for move in possible_moves:
                possible_boards.append(self.update_board(board, move[0], move[1]))
        return possible_boards

    def make_random_move(self, board: list[list[int]]) -> list[list[int]]:
        # get all possible moves
        possible_moves: list[tuple[int, int]] = self.get_all_possible_moves(board)
        if len(possible_moves) == 0:
            return []
        random_move = random.choice(possible_moves)
        return self.update_board(board, random_move[0], random_move[1])
    
    def make_random_piece(self, board: list[list[int]]) -> list[list[int]]:
        # get a random move
        possible_index = self.get_all_possible_new_pieces(board)
        random_piece = random.choice(possible_index)
        return self.add_new_piece(board, random_piece)
        

