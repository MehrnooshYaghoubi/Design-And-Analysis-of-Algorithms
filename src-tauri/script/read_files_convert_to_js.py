import os
import json

def folder_structure_to_json(root_path, output_file):
    structure = {}

    for dirpath, dirnames, filenames in os.walk(root_path):
        if dirpath == root_path or os.path.dirname(dirpath) == root_path:
            folder = os.path.basename(dirpath)
            structure[folder] = filenames

    with open(output_file, "w") as f:
        json.dump(structure, f, indent=2)

# Replace with your actual paths
path = "src-tauri/src/Algorithms"
output = "./public/folder_structure.json"

folder_structure_to_json(path, output)
print(f"Structure saved to {output}")
