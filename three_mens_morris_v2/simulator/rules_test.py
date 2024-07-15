import unittest

import rules


class TestRules(unittest.TestCase):

    def test_valid_move(self):
        # test both players
        board = [[0], []]
        old_move = 0
        new_move = 1
        player = 1
        self.assertTrue(rules.validate_move(board, old_move, new_move, player))

        board = [[], [0]]
        old_move = 0
        new_move = 1
        player = 2
        self.assertTrue(rules.validate_move(board, old_move, new_move, player))
    
    def test_valid_cross_move(self):
        # test all cross moves
        board = [[0], []]
        old_move = 0
        new_move = 4
        player = 1
        self.assertTrue(rules.validate_move(board, old_move, new_move, player))

        board = [[4], []]
        old_move = 4
        new_move = 0
        self.assertTrue(rules.validate_move(board, old_move, new_move, player))

    def test_invalid_move(self):
        # test all cross moves
        board = [[4], []]
        old_move = 0
        new_move = 8
        player = 1
        self.assertFalse(rules.validate_move(board, old_move, new_move, player))

    def test_other_player_move(self):
        # test all cross moves
        board = [[0], []]
        old_move = 0
        new_move = 1
        player = 2
        self.assertFalse(rules.validate_move(board, old_move, new_move, player))

    def test_valid_board(self):
        board = [[0], [2]]
        self.assertTrue(rules.validate_board(board))

        board = [[0, 1, 3], [2, 4, 5]]
        self.assertTrue(rules.validate_board(board))

    def test_invalid_player_length(self):
        board = [[0], [2], [3]]
        self.assertFalse(rules.validate_board(board))

    def test_inequal_counts(self):
        board = [[0], [2, 3, 4]]
        self.assertFalse(rules.validate_board(board))

    def test_validate_win(self):
        board = [[0, 1, 2], [3, 4, 7]]
        self.assertTrue(rules.check_win(board, 1))

        board = [[0, 1, 7], [3, 4, 5]]
        self.assertTrue(rules.check_win(board, 2))


if __name__ == "__main__":
    unittest.main()