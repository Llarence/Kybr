from datasets import load_dataset
from tqdm import tqdm
import numpy as np
from numba import jit

# The trailing zeros created are sus

path = 'data/code.data'
max_size = 1000_000_000
del_rate = 0.2

counts = np.zeros((98, 98), dtype=np.int64)

# Maybe should convert spaces to tabs because that is how they are inputted in the editors
# This could be rewritten to use Pandas or something to make this very fast
def update_counts(text):
    prev = None
    unicode_count = 0
    count = 0
    for byte in bytes(text, 'utf-8'):
        if unicode_count > 0:
            unicode_count -= 1
            continue

        value = None
        # This bit masking could be optimized I think
        if byte & 0b11100000 == 0b11000000:
            unicode_count = 1
        elif byte & 0b11110000 == 0b11100000:
            unicode_count = 2
        elif byte & 0b11111000 == 0b11110000:
            unicode_count = 3
        elif 32 <= byte <= 126:
            value = byte - 30
        elif byte == 9:
            value = 1
        elif byte == 10:
            value = 0
        elif byte == 13:
            continue

        if value == None:
            prev = None
            continue

        if prev != None:
            counts[prev, value] += 1
            count += 1

        counts[value, 97] += 1
        prev = value

    return count


# Could be speed up with concurrency
with tqdm(total=max_size) as bar:
    size = 0
    for data in load_dataset('codeparrot/github-code', streaming=True, split='train'):
        text = data['code']

        text_size = update_counts(text)
        bar.update(text_size)

        size += text_size
        if size > max_size:
            break


for i in range(98):
    counts[i, 97] *= 0.2

counts = counts / np.sum(counts)

with open(path, 'wb+') as file:
    file.write(counts.astype(dtype=np.float64).tobytes())
