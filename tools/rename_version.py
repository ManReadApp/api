import sys

def rename_folders(root_dir, old_folder_name, new_folder_name):
    for root, dirs, files in os.walk(root_dir):
        for folder in dirs:
            if folder == old_folder_name:
                old_path = os.path.join(root, folder)
                new_path = os.path.join(root, new_folder_name)
                os.rename(old_path, new_path)
                print(f"Renamed folder: {old_path} -> {new_path}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python3 file.py old_folder_name new_folder_name")
        sys.exit(1)
    
    old_folder_name = sys.argv[1]
    new_folder_name = sys.argv[2]
    
    # Replace 'path_to_your_folder' with the path to your folder containing subfolders
    folder_path = './'
    rename_folders(folder_path, old_folder_name, new_folder_name)
