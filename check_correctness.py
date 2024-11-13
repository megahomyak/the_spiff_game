import re

def read(file_name):
    with open(file_name, encoding="utf-8") as f:
        return f.read()

all_images_requested = set(re.findall(r'image\("\w+"\)', flags=re.DOTALL))
read("files/game_resources/script.js")
