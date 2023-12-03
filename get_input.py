import requests
import datetime
import os
import argparse


def fetch_input(year: str, day: str) -> str:
    with open('cookie_id') as f:
        cookie_id = f.read()
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    r = requests.get(url, cookies={"session": cookie_id})
    if r.status_code != '200':
        raise Exception(f"bad response from server: {r.text}")
    return r.text


def save_input(year: str, day: str):
    input = fetch_input(year, day)
    if not os.path.exists(f"./input/{year}"):
        os.mkdir(f"./input/{year}")
    path = f"./input/{year}/day{day}.txt"
    with open(path, 'w') as file:
        file.write(input)


def fetch_all():
    FIRST_YEAR = 2015
    current_year = datetime.date.today().year

    for year in range(FIRST_YEAR, current_year + 1):
        if year != current_year:
            last_day = 25
        else:
            last_day = datetime.date.today().day
        for day in range(1, last_day + 1):
            if not os.path.exists(f"./input/{year}/day{day}.txt"):
                save_input(year, day)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("year", type=int, nargs="?",
                        default=datetime.date.today().year)
    parser.add_argument("day", type=int, nargs="?",
                        default=datetime.date.today().day)
    parser.add_argument("-a", "--all", action="store_true")
    args = parser.parse_args()
    if args.all:
        fetch_all()
    else:
        save_input(args.year, args.day)


if __name__ == "__main__":
    main()
