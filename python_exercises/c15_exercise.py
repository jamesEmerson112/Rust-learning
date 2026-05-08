def normalize_word(word: str) -> str:
    stripped = word.strip()
    while stripped and not stripped[0].isalnum():
        stripped = stripped[1:]
    while stripped and not stripped[-1].isalnum():
        stripped = stripped[:-1]
    return stripped.lower()
