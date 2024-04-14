import argparse
import re
import subprocess
from pathlib import Path
from typing import Dict

argparser = argparse.ArgumentParser()
argparser.add_argument("--env-file", default=".env")

args = argparser.parse_args()


def read_env_file(env_file_path: str) -> Dict[str, str]:
    """
    Read the .env file and return a dictionary of key-value pairs.
    """

    secrets: Dict[str, str] = {}

    file = Path(env_file_path)

    if not file.exists():
        raise FileNotFoundError(f"File '{env_file_path}' not found.")

    with open(file) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            key, value = line.split("=", 1)
            if value == "":
                continue

            stripped_value = re.sub("[\"']", "", value)

            secrets[key] = stripped_value

    return secrets


def set_fly_secrets(secrets: Dict[str, str]) -> None:
    """
    Set secrets for fly using the `flyctl secrets set` command.
    """

    # check that flyctl is installed
    try:
        subprocess.run(["flyctl", "version"], check=True)
    except FileNotFoundError:
        raise FileNotFoundError("flyctl not found. Please install flyctl.")

    for key, value in secrets.items():
        subprocess.run(["flyctl", "secrets", "set", f"{key}={value}"])
        print(f"Set secret {key}")


if __name__ == "__main__":
    try:
        env_values = read_env_file(args.env_file)
        set_fly_secrets(env_values)
    except Exception as e:
        print(e)
        exit(1)
