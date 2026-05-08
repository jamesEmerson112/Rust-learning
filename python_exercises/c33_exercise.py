class Gradebook:
    def __init__(self):
        self.scores: dict[str, list[int]] = {}

    def add_score(self, student: str, score: int) -> None:
        if student not in self.scores:
            self.scores[student] = []
        self.scores[student].append(score)

    def average(self, student: str) -> float | None:
        if student not in self.scores or not self.scores[student]:
            return None
        vals = self.scores[student]
        return sum(vals) / len(vals)
