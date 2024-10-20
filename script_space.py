import os

print(os.getcwd())
path = os.getcwd() + "/assets/playing-cards-assets-master/png"

for f in os.listdir(path=path):
    print('"' + f.replace(".png", "") + '"' + ",")