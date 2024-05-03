Typst : https://typst.app/project/wTy2rQCF4xiFPr_3nY_YgF

# Peer 
1. 
Les communications sont sur le port TCP, envoie de message via TCP, et suivent le format :
```
announce listen $Port seed [$Filename1 $Length1
$PieceSize1 $Key1 $Filename2 $Length2 $PieceSize2 $Key2
…] leech [$Key3 $Key4 …]
> ok
```
1.1 Annonce au tracker son port d'écoute ```< announce listen $Port```
1.2 Annonce au tracker les fichiers que l'on partage ```< seed [filename taille tailleblock clef ]``` 

2. 
2.1 Demander liste des fichiers présent sur le réseaux vérifiant des criteres ```< look [critere1 critere2]```
2.2 Le server nous renvoie ```> list [filename taille tailleblock clef...]```
2.3 Le critere d'égalité de nom de fichier doit être implémenté

3. 
3.1 Télécharger un fichier en conaissant la clef ```< get $Key```
3.2 Le server nous renvoie > peers clef [ip1:port1 ip2:port2 ...]

4. 
4.1 Le pair demande les buffermap à tous les pair qu'il a reçu à l'étape d'avant
```> interested clef``` il recoit ```< clef buffermap```

Entre 4. et 5. 
Il trouve les paquets les plus rares (les moins présents dans les buffermap) et les télécharge

5.
5.1 Télécharger des pieces d'un fichier ```< getpieces clef [3 5 7 8 9]```
Il recoit ```> data clef [3:%piece3 5:%piece5] ``` où piece3 est la donné au format binaire

6. 
6.1 On envoie les buffermap aux pairs périodiquement
```< have clef buffermap``` 
```> have clef buffermap``` 
6.2 On envoie les buffermap de ces fichiers aux trackers
``` < update seed [clef clef clef] leech [clef13 clef14 clef15]```
``` > ok```

7. Avoir une config


TO USE TAURI
```npm install -g @tauri-apps/cli```
cd Application then ```npm run tauri dev``` or ```npx tauri dev```
export PATH="$PATH:/path/to/tauri/cli"


recherche fichiers avec un formulaire et des conditions
submit -> affiche resultat lignes -> click droit pour commencer le telechargement

upload fichiers a partir du navigateur fichiers

