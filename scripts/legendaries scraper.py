#legendaries_dom.txt : https://bulbapedia.bulbagarden.net/wiki/User:Focus58/List_of_Legendary_Pok%C3%A9mon
#CARE! this does not account for special forms such as:
#"deoxys-attack", "giratina-origin", etc...
#but it will scrape their names

def scrapeById():
    ids = []
    with open("legendaries_dom.txt", "r") as f:
        for line in f:
            if ("#") in line:
                ids.append(line.split(">")[1].strip().replace("#", ""))

    for _id in ids:
        print(_id)

    with open("legendary_pokemon.txt", "w") as f:   
        for _id in ids:
            f.write(_id + "\n")

def scrapeByName():
    names = []
    with open("legendaries_dom.txt", "r") as f:
        for line in f:
            if ('<td align="center">') in line:
                names.append(line.split(' ')[3].split('"')[1])

    for name in names:
        print(name)

    with open("legendary_pokemon.txt", "w") as f:   
        for name in names:
            f.write(name + "\n")

#scrapeById()
scrapeByName()
