import unittest

from utils.test_utils import timing
from .day_05 import parse_input, part_one, part_two

class TestDayFive(unittest.TestCase):

    def setUp(self):
        self.input_1, self.input_2 = parse_input()
    
    @timing
    def test_part_one(self):
        self.assertEqual(part_one(self.input_1), 372139)

    @timing
    def test_part_two(self):
        self.assertEqual(part_two(self.input_2), 29629538)


if __name__ == '__main__':
    unittest.main()