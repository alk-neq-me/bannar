import unittest
import ctypes
import sys


if sys.platform.startswith('linux'):
    library_file = './target/release/libbannar.so'
elif sys.platform.startswith('darwin'):
    raise ValueError("Unsupported platform")
elif sys.platform.startswith('win'):
    library_file = './target/x86_64-pc-windows-gnu/release/bannar.dll'
else:
    raise ValueError("Unsupported platform")

lib = ctypes.CDLL(library_file)

class TestMyLibrary(unittest.TestCase):

    def test_add_numbers(self):
        result = lib.show_bannar()
        self.assertEqual(result, 0)

if __name__ == '__main__':
    unittest.main()

