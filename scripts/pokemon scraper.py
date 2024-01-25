#fetches all the pokemon names in a dir
#designed to work with pokesprites
import os

def get_file_names():
    current_directory = os.getcwd()
    file_names = [file for file in os.listdir(current_directory) if os.path.isfile(os.path.join(current_directory, file))]
    return file_names

# Get the file names
file_names = get_file_names()

with open("pokemon.txt", 'w') as f:
    for name in file_names:
        f.write(name.split(".png")[0] + "\n")
