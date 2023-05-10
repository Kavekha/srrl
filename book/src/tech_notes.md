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


# Unités de mesure & positionnement
La Camera affiche 500.0 "unitées monde".  
Elle suit le Player dans le jeu.  
Elle est à 0.0,0.0 sur le Menu principal.  

Ascii.png contient 16 rangées de 16 colonnes.  
Ils sont en 8x8 pixels.
Le Ascii Loader leur donne un tile_size de 9.0 x 9.0.  

Le Ascii text et Ascii Sprite sont créés avec un Scale par défaut de 1.0 x 1.0 x 1.0.  
Dans un environnement 2D, Scale.0 a les valeurs x (hauteur), y (largeur), z (near camera).  
Avec le Sprite Ascii.0, une valeur differente de 1.0 permets d'aggrandir le pixel en plusieurs pixels.  
Cela permets de faire des traits (Comme le cadre des boutons du NineSlice) ou des barres de vie.  

Le Ascii text a pour point de départ le "Left Center".  
Chaque lettre + espace a une taille de 0.9 x TILE_SIZE.  

Le Player est créé comme un Ascii Sprite: 0.9.  
Son Positionnement est à 2.0 * TILE_SIZE, -2.0 * TILE_SIZE et une distance de 900.0.  
Sa vitesse de deplacement par defaut est de 6.0 * TILE_SIZE par tick, soit 0.6 unités / tick.  

La Map est créée sur la base d'une Grid avec des integers x et y.  
Le positionnement se fait à x * TILE_SIZE et -y * TILE_SIZE.  
Une grille de [0..10][0..10] placera donc la grille [0][0] à 0.0,0.0, la grille [0][1] à 0.0,-0.9 etc.  
Cela signifie qu'un personnage à la position 12.0,14.0 est à la grille [int(12.0 / TILE_SIZE)][int(14.0 / TILE_SIZE)] soit [13][15], 


# Tableau
|| Element | World Unit | Modified by TILE_SIZE? | Final Result ||
| TILE_SIZE | 0.1 | N/A | 0.1 |
| Vue Camera | 500.0 | N/A | 500.0 |
| AsciiSheet Tile 8x8 | 0.9 | N/A | 0.9 |
| Ascii Text + Space | 0.9 | Oui | 0.09 |
| Ascii Sprite | 0.9 | Non | 0.9 |












