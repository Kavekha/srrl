# Generation de missions.

## Long terme
Pour le moment, on souhaite que la mission produise le lieu où elle se déroulera afin d'être à son service.
Sur le long terme, on veut des lieux prégénérés en debut de jeu et que les missions s'y intègrent. Cela permets de revenir dans des lieux connus mais qui ont pu changer suite aux actions du joueur.

## Les bases d'une mission.
En première version, une mission se compose d'un objectif, d'un lieu et d'antagonistes, le tout présenté sous forme d'histoire.
Le lieu se base sur un thème simple (Metro, Catacombe, Egouts, Souterrains) avec differentes règles de construction pour produire des zones interessantes.
Un Layer agit dessus ensuite pour y construire l'objectif déterminé par la mission, notamment le point d'entrée, les sorties et la cible.
Un second Layer place les antagonistes en accord avec le Layer précédent.
Le succès ou l'echec de la mission est vérifié à la mort du perso ou à sa sortie de la map.

## Les objectifs et mecanismes.
Il y a un ou plusieurs objectifs primaires, qui determinent ou non le succès de la mission.
On peut complexifier le moteur de mission en ajoutant des objectifs secondaires, alimenter dans la map via un Layer.
Une autre amélioration serait les objectifs opérationnels: ils ne sont pas explicites pour le joueur, mais les Layers modifient la map pour les mettre en place. On parle ici de porte fermée qui nécessite de récuperer une carte magnetique pour l'ouvrir par exemple. Par nature, ils doivent être facultatifs.

### Objectifs finaux possibles.
- Eliminer une cible ou plusieurs.
- Chasser un groupe de la carte.
- Retrouver un ou plusieurs NPC et le(s) conduire à la sortie.
- Récuperer un objet.
- Déposer un objet.

### Objectifs opérationnels possibles.
- Obtenir une carte d'accès.
- Obtenir une clé.
- Obtenir une lettre d'introduction.
- Court-circuiter une grille.
- Activer un pont.
- Retirer l'eau d'un point de passage.
- Faire exploser un mur / un obstacle.

## Version avancée: en avant les histoires.
La finalité des missions est d'écrire, au fur et à mesure, une histoire pour le monde et les personnages qui y vivent.
Une façon d'ecrire une histoire est d'associer des thèmes proches, similaires, opposés ou se completant. Pour cela on peut associer, à la donnée ou en live, des etiquettes.
Le principe d'une etiquette est d'avoir un mot-clé dont le système se moque mais qu'il peut récupérer à plein d'endroits differents pour construire une cohérence.
Une partie est géré par la data, en associant par exemple l'etiquette Gang a une Faction disponible, et en indiquant que Gang & Biker sont des etiquettes se completant.
L'autre partie peut être gérée directement par le système, en créant à la volée des etiquettes pour mentionner que ces personnages / ce lieu est associé à un evenement particulier par exemple. Il est ainsi possible de reprendre une histoire commencée et la poursuivre en reprennant les objets concernés par ce tag créé pour l'occasion et en regardant dans les autres tags de ces objets des points communs qui permettraient de concevoir une nouvelle histoire.

### Les raisons derrière la mission.
On souhaite donc que le client ait une existence mentionnée dans les missions, et que l'on puisse voir en fil conducteur une motivation de sa part. 
Ces clients doivent aussi pouvoir devenir des cibles - directes ou indirectes - pour appuyer le coté mercenaire.
De même, une ancienne cible peut devenir client dans un autre temps.
Pour cela, on invente un NPC auquel on associerait quelques tags. On en invente un autre qui aurait au moins un tag en commun.
On exploite ensuite une serie de "Relation" expliquant l'opposition entre ces personnages: en compétition, en concurrence, desir de revenche, etc. Quelque chose qui les associe tous les deux de façon conflictuel.
Ce système d'association via des verbes peut se faire à plusieurs niveaux: les lieux, les zones, les objets, les personnages du joueur, les NPC, les factions, les evenements, etc.
L'important est aussi que le Client envoie en mission les personnages contre les agents de son adversaire plutot que l'adversaire lui-même, du moins dans l'immédiat. Cela permets de mentionner cet Adversaire dans le briefing, mais aussi dans les dialogues des agents ou même dans l'histoire et les events qui ont façonnés le lieu.
Le NPC n'a pas forcement besoin d'être hostile: il peut être interressé par un tag particulier. On peut imaginer un tag "Weapon", et que le NPC veuille le controle du marché noir des armes de guerre. Chaque fois qu'un tag "Weapon" est utilisé dans la construction d'une mission, même sans que ce NPC soit client ou adversaire, on peut imaginer que son nom soit mentionné, voire même des rebondissements où un agent adverse trahit son propre camp pour le servir.

#### Déroulement Système : la base.
Un NPC est créé. C'est le Client. 
On lui associe des tags parmi ceux disponibles. Imaginons: "Gang", "Weapon", "Honor", "Metahumain", "collector".
On cherche une faction qui contiendrait l'un de ces tags et qui serait sa cible. Cela peut être donc Meathead (gang), Smuggler Associated (Weapon), Ronin (Honor), Cour des Elfes (Metahumain), Academie de Magie (Collector).
On créé ensuite un nouvel NPC, l'adversaire.
On part du tag commun que l'on attribue à ce NPC, et on en ajoute d'autres. Si le choix fut "Collector", alors ce NPC Adverse a le tag "Collector".
On ajoute à ce nouvel NPC d'autres tags. Supposons: "Military", "AntiMeta", "Loyalty", "Spy".
Il nous faut ensuite une Relation négative entre les deux NPC, parmi une liste disponible: "Client veut humilier Adversaire en raison d'une humiliation passée". Cette Relation n'a pas forcement besoin d'être précise, et peut elle-même s'alimenter toute seule par la suite si des mécanismes le permettent.
On prends ensuite un objectif et on essait de l'associer à un element lié à un tag en rapport avec ceux du Client et de l'Adverse, pour créer un lien. Par exemple l'objectif "Voler un objet" peut concerner un Objet avec le tag "Collector". Cet objet peut avoir d'autres tags aussi : "Artefact", "Elfique", qui pourront servir pour cette intrigue ou d'autres.
On choisi ensuite le lieu où la mission se deroulera, basé sur les tags associés soit au NPC Adverse, soit à l'élement de la mission. Si NPC & Element ont un tag commun, il sera pris.
Cela peut être par exemple un tag "Loyalty", associé à un Sanctuaire souterrain. Par nature ce lieu a probablement d'autres tags, comme "Secte", "Société", "Secret".
Les antagonistes de la mission sont liés à l'un des tags de l'adversaire, comme "AntiMeta" que l'on peut retrouver chez le PolyClub Humanis, qui a d'autres tags "Société", "Fasciste". Une Relation positive peut être mise en place entre l'Adversaire et la Faction. Dans certains cas, cette Faction peut même être partagée entre le Client et l'Adversaire.
Les zones de ce lieu peuvent être associés à des meubles ou fonctions associés aux tags du lieu choisi, ou des tags associés aux antagonistes qui se les sont appropriés.
Enfin il est toujours bon d'avoir un Antagoniste pour la mission, un homme de confiance qui aura un tag associé au groupe choisi ("Société"), un autre à l'Adversaire ("Military"), et un dernier personnel ("Fanboy"). 

#### Exploiter le contenu généré.
A la fin de la mission, nous avons créé plusieurs personnages, factions, lieux, etc. 
La mission qui vient de s'achevée génère aussi un tag qui sera associé au Client, l'Adversaire, l'Antagoniste, le Lieu, l'Element et la faction utilisée par l'Adversaire pour conserver un historique.
Tout cela constitue un certain contenu pour la suite, y compris si l'Antagoniste survit à la mission.
Les missions suivantes peuvent être créées à partir d'un Objectif, d'un Lieu, d'une Faction, d'un NPC, d'un Objet ou même d'une mission passée. Sauf qu'à la difference de la première mission générée, elle ne regardera pas que le tag : elle ira chercher possiblement des élements créés auparavant. On essaiera toutefois de varier les missions, en conservant les tags utilisés en cooldown c'est à dire ne pouvant pas retomber de nouveau tout de suite. Idem pour les differents élements créés.
Ainsi pour une mission plus tard on a besoin d'un nouveau Client, et il existe 3 NPC identifiés: Le Client de la Mission 1, l'Adversaire de la Mission 1 et l'Antagoniste de la Mission 1 s'il a survécu. Mais ça peut être la création d'un nouvel NPC Client avec ses propres tags.
On peut tomber sur l'Antagoniste: Société, Military, Fanboy. 
On lui cherche une faction cible contenant l'un de ces tags: Polyclub Humanis (Société), Commando Chacal (Military), Fans de Maria (Fanboy).
On créé un NPC adversaire: "Fanboy". On en ajoute d'autres: "Dwarf", "Veteran", "Negociator", "Underground".
On indique la Relation Negative entre l'Antagoniste et le Nouvel Adversaire: "Lui a volé quelque chose de précieux" (en lien avec le tag).
On trouve un objectif : "Assassiner quelqu'un". L'Element commun entre les deux etant "Fanboy", ce sera un Chanteur dont Nouvel Adversaire est fan. Ce chanteur a des tags lui aussi: "Drug", "Music".
Le lieu est determiné selon le Nouvel Adversaire ou l'Element: "Underground". Si par exemple on tombe sur Galeries souterraines, celui-ci contient ses propres tags: "Obscurité", "Danger", "Secret".
On determine les Antagonistes par rapport au Nouvel Adversaire: soit la faction du Nouvel Adversaire, soit un tag qui lui appartient à lui ou sa faction. "Negociator" peut nous conduire à plein de corporatistes et leurs gardes du corps, et leur Relation Positive en faire des invités à un concert du Chanteur par le Nouvel Adversaire.
Le Nouvel Antagoniste peut être le garde du corps personnel du Chanteur, avec un tag associé aux Corporatistes ("Greedy"), au Nouvel Adversaire ("Veteran"), et un dernier personnel ("Cocky").
A la fin de cette mission à nouveau nous avons tous les elements concernés tagués avec cette mission, mais aussi de nouveaux NPC:
Nouvel Adversaire (FanBoy, Dwarf, Veteran, Negociator, Underground), Chanteur (Fanboy, drug, Music), Nouvel Antagoniste (Greedy, Veteran, Cocky).
Ils sont éligibles, si toujours vivants, à être des élements d'une autre mission en tant que Client, Adversaire ou Antagoniste.

### Donner de la personnalité aux missions.





# Generation de lieux.

## Base: Architecture sur base d'un concept fonctionnel.
On a un lieu que l'on veut construire comme un metro ou des égouts. Il y a donc une logique dans la façon dont il est construit et comment on circule selon la fonction du lieu.

## Layer : Evenements dans le temps : historique des lieux.
Bien qu'un lieu a une raison fonctionnelle d'exister en théorie (coucou corruption), des évènements peuvent alterer la raison d'être d'origine.
Ce layer produit ou non des évènements à differents stades de l'histoire du lieu ce qui peut provoquer des changements à sa forme.
On commence par la phase de construction : un evènement pourrait dire que finalement les financements sont coupés et la construction du metro a été interrompu. Ou seulement en partie.
Ces évènements sont testés plusieurs fois, et certains peuvent être le passage de phase de construction à phase d'exploitation etc.
Ces évènements peuvent être exterieurs: Attaque terroriste, catastrophe climatique, etc.
C'est important que ces évènements soient associés au lieu en lui-même, c'est à dire son emplacement physique, et non ce qu'il devait être au départ (un metro, des égouts). Cela permets d'avoir un lieu qui devait être un Metro et est finalement devenu, entièrement ou partiellement, un entrepot ou un laboratoire par exemple (Rachat par Mega corpo etc).
C'est ce Layer qui determine quels autres layers vont être joués pour faire "progresser" le Lieu.
Les évènements sont des évènements de construction: il y a eu des travaux, puis le lieu a été ouvert au public, puis il a été abandonné, puis il a été racheté pour construire autre chose.
On note aussi que ces évènements peuvent arrivés après la création de map : les actions du joueur peuvent provoquer un évènement qui altera le lieu et fera parti de son histoire.

## Layer : Zones fonctionnelles au sein du lieu.
La fonction du lieu est assurée par plein d'élements : les égouts ont des purificateurs d'eau, des salles de surveillance, des salles d'outillage pour les ouvriers, etc. 
Ce layer s'assure de placer des zones interressantes en cohérence avec la Base.
Chacune de ces zones peut avoir des conditions de placement (de l'eau pour les purificateurs) qu'elles peuvent eventuellement créées elles-mêmes pour exister (on ajoute alors de l'eau pour placer le Purificateur).

## Layer : Etat des lieux.
Ce layer determine si le lieu est au début de sa construction, en pleine usage, en restauration, à l'abandon.
Cela a pour effet de rajouter des élements propres à la situation (Des zones de chantier) ou de les retirer / vieillir / dégrader.
Ces modifications peuvent se faire au niveau des zones et non au niveau, une partie du metro peut donc être en chantier et l'autre abandonnée.

## Layer : Niveau social.
Ce Layer determine à quelle(s) population(s) ce lieu ou les zones de ce lieu sont déstinés. 
Cela se determine au travers des évènements : les lieux abandonnés sont squatés par les pauvres, les lieux pour riches ont tendance à être mis en place dans une logique séparatiste / surprotection contre les pauvres. Ces lieux peuvent aussi être reservés à une élite ou des citoyens corporatistes précis.

## Layer : Enrichissement gameplay.
Ces layers sont là pour mettre en place aux endroits de la map estimés comme pertinents les élements mettant en valeur les aspects du gameplay.
Cela peut être le placement des portes, de barrils explosifs, de lumière, de cameras, de pièges etc.
Ces élements sont contraints par la cohérence du lieu, de la zone et des antagonistes.

## Layer : Utilisation de contacts ou alliés.
Comme à long terme les personnages sont supposés avoir des contacts pouvant les assister, ces layers sont là pour trouver des opportunités où leur aide serait mise en valeur, ou l'inverse: l'absence d'aide serait pénalisante.

## Layer : Appropriation des lieux par la population.
Parmi les évènements, il y a le fait que des populations fréquentent le lieu. La géneration du lieu doit donc y laisser des traces de vie.
Ces traces de vie sont remplacées par de nouvelles traces au fur et à mesure que la population change: graffiti, publicité, etc.
A la fin, ce layer permets de placer les premiers NPC "vie de tous les jours" du lieu.

## Layer : Appropriation par les factions en cohérence avec la mission.
Comme à la fin il s'agit d'y accomplir des missions et des aventures, on y place les antagonistes à la fin.
On essait de les positionner en accord avec la mission et leurs personnalités: protection d'une personne importante, attaque d'une zone particulière, etc.






