def normalise(word: str) -> str:
    stripped = word.strip()
    while stripped and not stripped[0].isalnum():
        stripped = stripped[1:]
    while stripped and not stripped[-1].isalnum():
        stripped = stripped[:-1]
    return stripped.lower()


def word_count(text: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for word in text.split():
        w = normalise(word)
        if w:
            counts[w] = counts.get(w, 0) + 1
    return counts
