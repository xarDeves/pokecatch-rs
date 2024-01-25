import os

def add_female_suffix_to_files():
    current_directory = os.getcwd()
    
    for file_name in os.listdir(current_directory):
        if os.path.isfile(os.path.join(current_directory, file_name)):
            # Check if the file is a PNG file
            if file_name.lower().endswith(".png"):
                # Add "-female" suffix to the file name
                new_file_name = file_name.split(".png")[0] + "-female.png"
                
                # Rename the file
                os.rename(os.path.join(current_directory, file_name), os.path.join(current_directory, new_file_name))

# Add "-female" suffix to files in the current directory
add_female_suffix_to_files()
