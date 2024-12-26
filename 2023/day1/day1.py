import sys

mapping = {
  "one": "o1e",
  "two": "t2o",
  "three": "t3e",
  "four": "f4r",
  "five": "f5e",
  "six": "s6x",
  "seven": "s7n",
  "eight": "e8t",
  "nine": "n9e",
  "zero": "z0o"
}

def get_digit(calibration, part2, from_last=False):
    if part2:
      for map in mapping.keys():
          calibration = calibration.replace(map, mapping[map])

    if from_last:
        calibration = calibration[::-1]

    for digit in calibration:
        if digit.isnumeric():
            return digit
    
    raise TypeError("No digit found in calibration")

def get_score(calibrations, part2):
    score = 0

    for calibration in calibrations:
        first_digit = get_digit(calibration, part2=part2)
        last_digit = get_digit(calibration, part2=part2, from_last=True)
        score += int(first_digit + last_digit)

    return score

def main(filename):
    with open(filename) as f:
        calibrations = [x.strip() for x in f.readlines()]
        print(f"Part 1: {get_score(calibrations, False)}")
        print(f"Part 2: {get_score(calibrations, True)}")
    
if __name__ == "__main__":
    main(sys.argv[1])