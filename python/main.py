import requests
from selectolax.parser import HTMLParser
import json
from urllib.parse import urlencode


search_string = "fountains of wayne"
url = "https://www.songsterr.com/?{}".format(urlencode({"pattern": search_string}))
print(url)

response = requests.get(url)

data = response.text
tree = HTMLParser(data)

for item in tree.css("script"):
    try:
        if item.attributes["id"] == "state":
            print("---ITEM---")
            state = item.text()

            songs = json.loads(state)["songs"]["songs"]["list"]
            for song in songs:
                print(song)
                print(f"{song['artist']} -> {song['title']}")
            print()

    except KeyError:
        pass
