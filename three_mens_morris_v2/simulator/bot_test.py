import unittest

from bot import Bot


class TestBot(unittest.TestCase):

    def testBotMakeRandomNewMove(self):
        bot = Bot(0)
        board: list[list[int]] = [[], []]
        move = bot.make_move(board)

        self.assertTrue(move in [0, 1, 2, 3, 4, 5, 6, 7, 8])
    
    def testBotMakeRandomMove(self):
        bot = Bot(0)
        board: list[list[int]] = [[0, 1, 2], [3, 4, 5]]
        result = bot.make_move(board)

        self.assertTrue(isinstance(result, tuple))
        if not isinstance(result, tuple):
            return
        (old_index, new_index) = result
        self.assertTrue(old_index in [0, 1, 2])
        self.assertTrue(new_index not in [3, 4, 5])
