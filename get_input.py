import requests
import datetime
import os


def fetch_input(year: str, day: str) -> str:
    with open('cookie_id') as f:
        cookie_id = f.read()
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    r = requests.get(url, cookies={"session": cookie_id})
    return r.text


def save_input(year: str, day: str):
    input = fetch_input(year, day)
    if not os.path.exists(f"./input/{year}"):
        os.mkdir(f"./input/{year}")
    path = f"./input/{year}/day{day}.txt"
    with open(path, 'w') as file:
        file.write(input)


if __name__ == "__main__":
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
