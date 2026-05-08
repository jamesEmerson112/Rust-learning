import time

# Build a ~1M word string by repeating a paragraph
base = ("the quick brown fox jumps over the lazy dog and the fox runs fast "
        "through the dark forest while the dog sleeps by the warm fire and "
        "the cat watches from the window as birds fly over the old stone wall")
text = " ".join([base] * 30_000)

start = time.perf_counter()
counts = {}
for word in text.split():
    w = word.lower()
    counts[w] = counts.get(w, 0) + 1
elapsed = (time.perf_counter() - start) * 1000

print(f"{elapsed:.4f}")
print(f"unique = {len(counts)}", file=__import__('sys').stderr)
