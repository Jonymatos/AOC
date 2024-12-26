import sys
import re

def process_games(games, part2=False):
    pattern = re.compile(r'(\d+) (\w+)')

    def get_minimums(game):
        minimus = {'red': None, 'green': None, 'blue': None}
        for set_game in game:

            matches = pattern.findall(set_game)
            for amount, color in matches:
                if minimus[color] is None or int(amount) > minimus[color]:
                    minimus[color] = int(amount)

        return minimus

    def validate(game):
        colors_limit = {'red': 12, 'green': 13, 'blue': 14}
        for set_game in game:

            matches = pattern.findall(set_game)
            for amount, color in matches:
                if int(amount) > colors_limit[color]:
                    return False
                
        return True

    sum_result = 0
    for id_game, game in enumerate(games):

        if not part2 and validate(game):
            sum_result += id_game + 1

        elif part2:
            minimums = get_minimums(game)
            sum_result += (minimums['red'] * minimums['green'] * minimums['blue'])

    return sum_result


def main(filename):
    with open(filename) as f:
        games = [x.strip().split(':')[1].split(';') for x in f.readlines()]
        print(f'Part 1: {process_games(games)}')
        print(f'Part 2: {process_games(games, True)}')

if __name__ == "__main__":
    main(sys.argv[1])