import os
import sys

if __name__ == '__main__':
    import sys
    import random
    import string

    if len(sys.argv) != 4:
        print('Usage: {} <vault_path_linked> <vault_path_unlinked> <output_test_file>'.format(sys.argv[0]))
        sys.exit(1)

    print('Generating test file...')
    print(f'{sys.argv}')
