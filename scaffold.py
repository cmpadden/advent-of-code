# /// script
# dependencies = [
#   "requests",
#   "lxml",
# ]
# ///

import logging
import os
import sys

import requests

from lxml import html

# Configure logging to show ERROR level and above by default
logging.basicConfig(
    level=logging.ERROR,
    format='%(levelname)s: %(message)s'
)
logger = logging.getLogger(__name__)

if len(sys.argv) != 4:
    print("USAGE: scaffold <YEAR> <DAY> <SESSION_COOKIE>")
    sys.exit(1)

year, day, session_cookie = sys.argv[1], sys.argv[2], sys.argv[3]


_dir = f"{year}/{int(day):02}"

logging.info(f"Creating directory: {_dir}")
os.makedirs(_dir, exist_ok=True)

url = f"https://adventofcode.com/{year}/day/{int(day)}"
url_input = f"https://adventofcode.com/{year}/day/{int(day)}/input"

try:
    response = requests.get(url)
    if response.status_code != 200:
        logger.error(f"Failed to fetch problem page from {url}")
        logger.error(f"Status code: {response.status_code}")
        logger.error(f"Response: {response.text[:500]}")
        sys.exit(1)
except requests.exceptions.RequestException as e:
    logger.error(f"Network error when fetching problem page: {e}")
    sys.exit(1)

tree = html.fromstring(response.content)
el_desc = tree.find('.//article[@class="day-desc"]')

with open(f"{_dir}/main.py", "w") as f:
    f.writelines(
        [
            '"""',
            el_desc.text_content(),
            '"""',
        ]
    )

try:
    response = requests.get(
        url_input,
        cookies={
            "session": session_cookie,
        },
    )
    if response.status_code != 200:
        logger.error(f"Failed to fetch input from {url_input}")
        logger.error(f"Status code: {response.status_code}")
        logger.error(f"Response: {response.text[:500]}")
        if response.status_code == 400:
            logger.error("HINT: This might indicate an invalid or expired session cookie")
        elif response.status_code == 404:
            logger.error("HINT: The puzzle input might not be available yet")
        sys.exit(1)
except requests.exceptions.RequestException as e:
    logger.error(f"Network error when fetching input: {e}")
    sys.exit(1)

with open(f"{_dir}/input.txt", "w") as f:
    f.write(response.content.decode("UTF-8"))
