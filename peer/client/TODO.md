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