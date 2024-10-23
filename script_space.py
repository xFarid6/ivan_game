# import os

# print(os.getcwd())
# path = os.getcwd() + "/assets/playing-cards-assets-master/png"

# for f in os.listdir(path=path):
#     print('"' + f.replace(".png", "") + '"' + ",")

###################################

frase = input("Inserisci una frase: ")
# Slicing per ottenere la seconda parola
# Usa il metodo split() per dividere la frase
seconda_parola = frase.split(" ")[1]
# Stampa la seconda parola ripetuta 3 volte
print(seconda_parola)



