## TODO 
- choose_file : boucle recursive comme dans menu.rs qui demande que l'input soit un nombre compris entre 1 et nbr de fichiers téléchargables




## Intro 
    - port, ip of tracker and peer in config.ini
    - port of peer can be changed in command line via -p
    - There is LOG mode where messages are printed with logged function 

LOG: 
## Presentation UI
Menu principale 
1. Search
2. Upload
3. Download

# Search
Enter file name (hit enter to see all) : 
Enter 
LOG:look
Voici la liste des fichiers disponibles : 
1. baba.text
2. test_taille
Menu principale 
1. Search
2. Upload
3. Download

# Download
Select file to download : 
(1 *enter)
Downloading baba.text from IP:PORT IP2:PORT2
Menu principale 
1. Search
2. Upload
3. Download

## Upload
Enter file name to upload : 
LOG: `announce listen $Port seed [$Filename1 $Length1 $PieceSize1 $Key1 $Filename2 $Length2 $PieceSize2 $Key2 …] leech []`

# Architecture download

filename    | key | length | piece_size | (ip,port)
test_taille | 1   |        |            | (127.0.0.1, 1234), (127.0.0.1, 1235)     
test_taille | 2   |        |            |
baba.text   | 3

1. **list** -> (filename,key) | permet de stocker les fichiers disponibles avec leur clef
csv file with all files available with keys 


peer(ip,port,buffermap,key)
file(filename,key,length,piece_size)

2. **getfile clef** -> [(ip1,port1), ... (ipn,portn)] | permet de stocker les pairs qui possèdent le fichier

3. 
pour tout peer qui a le fichier : 
    envoie **interessed clef**
    recoit **have ** -> (clef, buffermap) | pour chaque 

il trouve les paquets les plus rares (les moins présents dans les buffermap) et les télécharge
4. 
    pour chaque paquet à télécharger :
        envoie **getpieces clef [3 5 7 8 9]**
        recoit **data clef [3:%piece3 5:%piece5] ** où piece3 est la donné au format binaire

6 périodiquement
    envoie **have clef buffermap** 
    envoie **update seed [clef clef clef] leech [clef13 clef14 clef15]**

search section list de fichiers -> download si download prend void il affiche le resultat de look (tout les fichiers disponibles)


TODO :
1 Un formulaire pour rechercher des fichiers avec des conditions
Submit -> affiche resultat lignes -> click droit pour commencer le telechargement

2 Upload fichiers a partir du navigateur fichiers

3 Remove connect disconnect buttons

4 change user input functions from rust -> js