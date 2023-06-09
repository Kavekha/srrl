# Systeme de Coordonnées.

## Scene 2D Bevy:
x va de gauche vers la droite. +x ==> Vers la droite.  
y va du bas vers le haut. +y ==> Vers le haut.
z va de loin vers proche camera (vue perspective).  
0.0, 0.0 est le centre de l'ecran par défaut.  

## UI (Ui bevy?):
Origin est au top left corner.  
Y va vers le bas.
X par de 0.0 à gauche jusqu'au nombre de pixels de l'ecran à droite.  
Y par de 0.0 en haut jusqu'au nombre de pixels de l'ecran en bas.  
Les unités UI sont des pixels logiques, compensés par le DPI Scaling. Yeah.  


## Unités de mesure & positionnement
La Camera considère 1.0 pixel comme une unité monde.    
Elle suit le Player dans le jeu.  
Elle est à 0.0,0.0 par défaut.  

Ascii.png contient 16 rangées de 16 colonnes.  
Ils sont en 8x8 pixels.
Le Ascii Loader leur donne un tile_size de 9.0 x 9.0.  

Le Ascii text et Ascii Sprite sont créés avec un Scale par défaut de 1.0 x 1.0 x 1.0.  
Dans un environnement 2D, Scale.0 a les valeurs x (hauteur), y (largeur), z (near camera).  
Avec le Sprite Ascii.0, une valeur differente de 1.0 permets d'aggrandir le pixel en plusieurs pixels.  
Cela permets de faire des traits (Comme le cadre des boutons du NineSlice) ou des barres de vie.  

Le Ascii text a pour point de départ le "Left Center".  
Chaque lettre + espace a une taille de 0.9 x CHAR_SIZE.  

Le Player est créé avec son image pour taille. 
Sa vitesse de deplacement par defaut est de 6.0 * TILE_SIZE par tick, soit 6 x TILE_SIZE unités / seconde.  

La Map est créée sur la base d'une Grid avec des integers x et y.  
Le positionnement dans le monde se fait à x * TILE_SIZE et -y * TILE_SIZE.  
Une grille de [0..10][0..10] placera donc la grille [0][0] à 0.0,0.0, la grille [0][1] à 0.0,-(1 * TILE_SIZE) etc.  

# Deplacement
Très contre-intuitif pour le moment.  
Je suis à 38,22 (grid unit).  
Je me deplace vers le bas : +0, -1y.  
J'arrive à 38, 23.  
Pour descendre, je vais donc vers le negatif (Monde), ce qui se traduit par une augmentation sur la map (Grid).  
Je me deplace vers la gauche : -1, +0.
J'arrive à 37, 23.
Pour aller à gauche je vais donc vers le negatif (Monde), ce qui se traduit par une réduction sur la map (Grid).
BAS / HAUT sont donc inversés entre Grid et Monde.  


# Events / loop
## Deroulement
StartGame -> State:PlayerInput  
Recoit une PlayerAction -> State:TurnUpdate (Rempli la Queue si 1° fois) -> Declenche un Tick.  
Tick déclenche process_action_queue -> Récupère l'action du joueur et la teste.  
==> Si resolue, mets les actions qui en decoulent potentiellement dans la Pending Action.  
==> Si echec, retourne InvalidPlayerActionEvent => On revient à PlayerInput pour qu'il prenne une autre décision.  
send NextActor Event (Les NPC)  -> NPC font leur planning pendant le cycle.  
Nouveau tick -> Active process action queue -> Prends les actions de l'acteur, les trie par priorité, les verifie.  
==> Si resolue, mets les actions qui en decoulent potentiellement dans la Pending Action et s'arrête.  
==> Si echec, passe à l'action possible suivante.  
send NextActor Event (Les NPC)  -> NPC font leur planning pendant le cycle. -> Nouveau tick jusqu'à ce que Pending & Actor vide.  
Si tout vide: ActionsCompleteEvent -> State:PlayerInput.  

## Note
On ne distingue pas les actions logiques des actions graphiques, elles vont dans la même queue.  
On a par contre plusieurs etapes: D'abord un evenement logique, suivi par un evenement animation, puis un nouveau tick.  

## Ameliorations à considérer
Mieux séparer l'animation du logic : une serie d'animations bloque le logic si on veut qu'elle se joue jusqu'à la fin.  
De même, les animations se jouent les unes à la suite des autres. Si on veut les "controler", on allonge le temps avant que le joueur retrouve la main.  
Peut être qu'une nouvelle action Joueur doit "cut" les animations à la séquence de fin, ou les recalculer pour être plus à jour.  
On aimerait aussi pouvoir distinguer "animation basique" (les mobs se deplacent) et "animation importante" (Les mobs me tapent): les premières peuvent être jouées ensemble, les secondes doivent s'alterner.  





