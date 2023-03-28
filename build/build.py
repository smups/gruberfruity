version = "0.1"
colors_folder = "../colors/"

class GruberFruit:
    """
        Class for creating Gruber Fruit style themes based on a base template
        and a colors json file.
    """
    template_folder = "../templates/"
    theme_folder = "../themes/"
    
    def __init__(self, name, emoji, base, colors: dict):
        self.name = f"{name} {base.capitalize()} {emoji}"
        self.base = base
        self.colors = colors
        
        print(f"""created gfruit theme: "{self.name}" """)
        
    def create_theme(self):
        # (1) read the template folder
        output = ""
        with open(f"{self.template_folder}{self.base}.jsonc", encoding="utf-8") as f:
            output = f.read()
            f.flush()
            
        # (2) replace the keywords in the template with the colors dict. entries
        for keyword, value in self.colors.items():
            output = output.replace(keyword, f""" "{value}" """)
        
        # (3) add name
        output = output.replace("__name__", f""" "{self.name}" """)
            
        # (4) write the result to the theme directory
        with open(f"{self.theme_folder}{self.name}.json", "w+", encoding="utf-8") as f:
            f.write(output)
            f.flush()
        
        print(f"""parsed gfruit theme: "{self.name}" """)
            
import json    
import os

print(f"Running Gruber Fruit's theme parser v{version}...")

# (1) read all files in the colors folder
for f in os.listdir(colors_folder):
    path = os.path.join(colors_folder, f)
    try:
        # (2) parse file
        theme_json = json.load(open(path,encoding="utf-8"))
        GruberFruit(
            theme_json["name"],
            theme_json["emoji"],
            theme_json["base"],
            theme_json["colors"]
        ).create_theme()
    except Exception as err:
        print(err)