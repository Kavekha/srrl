last update: 0.20  

# inspiration
FEAR pour la logique de Squat,  
Left For Dead pour la logique "MJ" d'un combat,  
Alien Isolation pour la logique de traque & poursuite,
Thief pour la logique infiltration.    
Très séduit aussi par Infinite Axis / Utility IA plus "logique" pour de l'ECS.
https://www.youtube.com/watch?v=dB7ZSz890cw



# Hide & Seek  
## Status des NPC
Les NPC ont plusieurs status:  
- Unaware: Ne savent pas où est le PJ, ne le cherche pas, font leur vie.  
- Suspicious: Un element leur a donné l'idée que tout ne va pas forcement bien, et ils deviennent attentifs et cherchent un peu au hasard.   
- Tracking: Ils savent qu'il y a quelqu'un, et le cherchent.  
- Hunting: Leur cible est en visu, et ils veulent la tuer. 
Ils peuvent revenir de Suspicious à Unaware, de Hunting à Tracking, mais jamais ne reviennent à Suscpicious / Unaware s'ils ont atteint le Hunting / Tracking.  
>> MVP 0 : Dans un premier temps, on serait toujours en Hunting ou Tracking.  

## Traque
Quand un NPC est Tracking, il cherche activement sa cible.  
Techniquement:  
1. Il regarde autour de lui: pas de cible. Il enregistre sa position comme "lieu sans succès" ainsi que le tour.  
2. Il va vers une zone qu'il n'a pas encore exploré.  
3. S'il n'a plus de zone non-exploré, il revient vers la plus ancienne "sans succès".  

## Communication
Quand un NPC croise un autre NPC, s'ils ont l'abilité de communiquer, ils partagent leurs informations.  
Techniquement:  
1. Les deux NPC doivent être à porté de voix.  
2. Une communication gratuite par tour.  
3. Ils peuvent s'arreter et echanger plus longuement, contre des AP.  
4. Ils se donnent l'information la plus recente qu'ils n'ont pas en commun.  
Dans l'ideal, toute communication NPC se fait à haute voix pour le benefice du joueur.  
>> Hors MVP 0 : Un NPC pourrait Suspicious pourrait rendre Suspicious un autre, et un NPC Tracking de même. Une forme de contagion.  
>> Hors MVP 0 : Les NPC ont des moyens de communication à plus grande echelle, et peuvent communiquer sans être en contact.  

## Association
Quand un NPC en rejoint un autre, ils peuvent decider de poursuivre ensemble ou de se séparer.  
>> Cette décision peut être renforcé s'ils trouvent un cadavre par exemple.  

## Chasse
Quand un NPC chasse le joueur, c'est qu'il l'a eu un visu.  
Dans ce cas-là, on est bien plus précis: le NPC sait où est le PJ.  
On garde toujours en dernier "visuel" le premier point où le NPC a perdu de vue: on sait donc dans quelle direction il a tourné.  
Techniquement:  
1. Le PJ est en visuel du NPC au debut de son tour.  
2. Il se déplace à l'angle d'un couloir, dans une case toujours visible du NPC : le NPC le sait.
3. Il va dans le couloir, la case n'est plus visible du NPC : le NPC recoit l'information que le PJ s'est rendu dans cette case : il l'a vu tourné.  
4. Le PJ poursuit dans le couloir et tourne un peu plus loin: le NPC ne le sait pas car plus dans son visuel depuis l'etape 3.  

# Combat
## Attitudes des NPC
Les NPC ont diverses attitudes qui determinera leur façon de combattre.  
Cela donne un peu de couleur selon que l'adversaire soit une créature, un simple gang ou un veteran par exemple.  
## Stratégies
### Identifier les endroits où le NPC finira son tour à l'abri.
Ces endroits dépendent si leur cible peut attaquer à distance ou non.  
Selon leur courage, les NPC tenteront de faire une attaque puis se cacher, ou prioriseront le fait d'être caché.  
Cette décision peut avoir pour critère la volonté de rester en vie, attendre des renforts ou au contraire empecher le PJ de fuir car proche de la sortie.  
### Prendre en tenaille. 
Les NPC connaissent les lieux: s'ils voient leur Cible à un endroit, ils vont essayer d'en couvrir toutes les sorties.  
Cette stratégie n'est pas incompatible avec le fait de finir son tour à l'abri.  
Il faut par contre une logique de communication: "Par défaut j'allais à cette zone, je vois que tu es là copain NPC, je vais vers une autre zone".  
Dans la même logique, le NPC communique avec les autres NPC les sorties disponibles.  

# Director & pacing.
Le principe de l'IA Director est d'assurer le fun du joueur.  
Le principe est de créer des moments de tension mais aussi de les réduire pour laisser le joueur souffler.  
>> Un peu tot pour le MVP 0, mais on peut imaginer des NPC qui se mettent à faire une ronde, ou à se disputer, ou se trompe de cible à traquer en se courant l'un après l'autre, etc.  
>> Plus tard, on peut même imaginer des décisions autour de l'objectif, qui doit être une metric assez importante, comme par exemple faire fuir la cible ou deplacer un objet.  