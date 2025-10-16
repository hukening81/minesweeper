import requests
from os.path import join, dirname,exists
import os

DATA_FOLDER = join(dirname(__file__), "./assets")

BASE_URL = "https://minesweeper.online/img/skins/hd/"
IMAGE_SUBFIX = ".svg"


def make_url(name: str) -> str:
    return BASE_URL + name + IMAGE_SUBFIX


def main():
    # create folder
    if not exists(DATA_FOLDER):
        os.mkdir(DATA_FOLDER)
    
    urls = []

    # closed cell
    urls.append(make_url("closed"))

    # number cells
    for i in range(9):
        urls.append(make_url(f"type{i}"))

    # yellow number cells
    for i in range(9):
        urls.append(make_url(f"type{i}_yellow"))

    # mine cells
    urls.append(make_url("mine"))
    urls.append(make_url("mine_red"))
    urls.append(make_url("mine_wrong"))

    # face image
    urls.append(make_url("face_lose"))
    urls.append(make_url("face_pressed"))
    urls.append(make_url("face_unpressed"))

    # download these images
    for image in urls:
        image: str
        with open(join(DATA_FOLDER, image.split("/")[-1]), "bw") as file:
            file.write(requests.get(image,timeout=1000).content)


if __name__ == "__main__":
    main()
