from itertools import combinations, permutations

ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"]

suits = ["♠", "♦", "♥", "♣"]

deck = [f"{rank} of {suit}" for rank in ranks for suit in suits]

size = 3

partial = list(permutations(deck, size))

hands = list(combinations(deck, size))


def categorize(hand):
    rank_counts = {}
    for card in hand:
        rank = card.split()[0]
        if rank in rank_counts:
            rank_counts[rank] += 1
        else:
            rank_counts[rank] = 1

    sorted_ranks = sorted([card.split()[0] for card in hand])

    def straight():
        """3 in sequence (includes AKQ)."""
        ordering = []

        example = [
            ["A", "2", "3"],
            ["2", "3", "4"],
            ["3", "4", "5"],
            ["4", "5", "6"],
            ["5", "6", "7"],
            ["6", "7", "8"],
            ["7", "8", "9"],
            ["8", "9", "10"],
            ["9", "10", "J"],
            ["10", "J", "Q"],
            ["J", "Q", "K"],
            ["Q", "K", "A"],
        ]

        for i in range(len(example)):
            perm = list(permutations(example[i]))
            for j in range(len(perm)):
                ordering.append(list(perm[j]))

        sample = sorted_ranks in ordering
        return sample

    def flush():
        """3 suited."""
        return len(set(card.split()[-1] for card in hand)) == 1

    def royal_flush():
        """Three cards of the same suit, AKQ in any order."""
        suits = set(card.split()[-1] for card in hand)
        ranks = set(card.split()[0] for card in hand)
        return len(suits) == 1 and ranks == {"A", "K", "Q"}

    if royal_flush():
        return "Royal Flush"

    if straight() and flush():
        return "Straight Flush"

    def three_aces():
        """3 Aces (any combo of suits)"""
        ranks = [card.split()[0] for card in hand]
        return ranks.count("A") == 3

    if three_aces():
        return "Three Aces"

    def three_of_a_kind():
        """Three cards of the same rank."""
        for rank, count in rank_counts.items():
            if count == 3:
                return True
        return False

    if three_of_a_kind():
        return "Three of a Kind"

    if straight():
        return "Straight"

    if flush():
        return "Flush"

    if 2 in rank_counts.values():
        return "Pair"

    return "High Card"

