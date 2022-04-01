import unittest
from nors import count

class TestNors(unittest.TestCase):
    def test_works(self):
        self.assertEqual(count("./tests/10.csv")["lines"], 10)
        self.assertEqual(count("./tests/10.csv")["csv_records"], 10)
