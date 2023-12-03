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
    now = datetime.date.today()
    save_input(now.year, now.day)
