# Release 0.4 (0.20)

## Bilan
- Prevoir les refactos des systèmes touchés lors de l'ajout d'un nouveau systeme.  
- Améliorer le systeme de rendu des animations.  
- Revoir les controles.  


## Ce qui a bien marché
- Refacto régulières.
- L'ajout des graphismes, sons, musiques, etc. Cela donne plus de vie et de motivation.
- La refacto du cursor / action_infos qui a bien simplifié le fonctionnel.


## Ce qui n'est pas ouf.
- Les nouveaux systemes tendent à deborder hors de leur perimètre sur d'autres systèmes, et une refacto est necessaire après mais etat de semi burn-out à ce moment-là.  
- Les animations se font encore les unes sur les autres, ce qui cachent par exemple le fait que plusieurs coups peuvent être portés. On doit attendre chaque NPC.  
- La sauvegarde est encore cassé. Il faut l'assumer pour le moment.  
- Encore beaucoup de choses en dur. Il est temps d'avoir un vrai systeme de données pour structurer le tout.  
- Les systèmes s'empilent et sont vulnerables car se touchent les uns les autres. Mieux créer de l'independance (Visibilité, IA).  
- Les controles ne sont pas à la hauteur.  
- L'opposition Dual Grid & Logic Grid très penible pour le systeme de rendu de la visibilité.  


