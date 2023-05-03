# Introduction

## Goal
Le but premier est de developper un petit jeu en apprennant Rust, puis d'iterer dessus pour l'enrichir.

## Choix du jeu
ShadowRun est un jeu de rôle papier dans un univers futuriste cyberpunk où la magie serait revenue: elfes, orks, nains, dragons, trolls existent dans ce monde.
Les joueurs y jouent des mercenaires accomplissant des missions ("runs") officieuses pour le compte des Corporations ou autres organisations.
Les règles sont assez violentes et la mort vient vite, ce qui corresponds bien à la philosophie d'un rogue-like.

## boucle de gameplay

La description la plus sommaire d'une partie de ShadowRun serait la suivante:
1. les personnages recoivent une mission.
2. Ils l'accomplissent.
3. Ils reviennent se faire payer.

A partir de là, plusieurs élements peuvent l'enrichir:
1. Les personnages doivent avoir un contact en mesure de leur fournir du travail.
2. Ce travail est apporté par un Mr Johnson représentant un client.
3. L'objectif affiché de ce travail n'est pas forcement l'objectif réel du Johnson ou de son client.
4. Il est de bon ton de s'informer sur le client ou le Johnson avant de le rencontrer.
5. Negociation du paiement de la mission, parfois avec des avantages en nature (Equipement special, dette, etc)
6. La mission contient généralement un aspect enquête, ou à minima un peu de préparatifs pour ne pas se jetter dans la gueule du loup.
7. Durant la mission, des évènements peuvent compliquer l'affaire.
8. Accomplir la mission ne suffit pas, il faut s'en sortir vivant.
9. Une fois la mission finie, le client ou le Johnson n'a pas forcement envie de payer ou de laisser des traces.
10. Une fois la mission finie, les personnages peuvent vouloir en tirer d'autres benefices et faire du chantage ou vendre des infos au plus offrant.
11. Les personnages peuvent aussi ne pas vouloir accomplir la mission pour leur client, mais en faire profiter un autre.
12. Les opposants peuvent vouloir se venger, surtout si la mission a été accomplie de façon trop visible.

En vision "big picture", il s'agirait autant d'accomplir des missions que de se prémunir contre les retours de flamme des missions déjà accomplies.


