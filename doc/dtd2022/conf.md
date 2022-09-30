title: Horloge à mots au Secteur 3
author:
  name: Gil, Charlotte, Sebastien, Xavier
controls: false
theme: ./orange-theme
---
# Une Horloge à mots
## Les jeudis de l'electronique au secteur 3
---
# Introduction
## (5mn): Seb
---
### C'est quoi le secteur 3 et la code room ? (3mn)

- Infos utiles, qui peut venir, quels jours...
- Quelques vieux projets:

---
### Le projet : l'horloge à mots (2mn)
- Présentation, fonctionnement, tarifs commerciaux

<center>![horloge](images/horloge.png) ![horloge gold](images/horloge-gold.png)</center>

- Nos objectifs : faible coût, composants simples
---
# Composants et design de la board
## (15mn) - Xav +Florent
---
### Préambule : tout ce qu'on a souhaité écarter (2mn)
- Les rubans de LEDs adressables
- Connexion Wi-Fi, mise à jour par NTP
---
### La problématique de base : comment piloter XX leds ? (5mn)
  - Quand XX = 1 : 
     - courant
     - voltage d'une LED
     - contraintes d'un microcontrôleur
  - Les différents types de multiplexage
    - nombre de GPIO nécessaires, 
  - choix du µc: 
    - la pénurie
    - les stm32 et atmega8 (alimentation, )
---
### Comment interagir avec l'objet ? (3mn)
  - Nos idées d'interface utilisateur (PIR, boutons, mise sous tension à heure fixe...)
---
### Le schéma de la board (5mn)
  ![le schema](images/schema_avr.png)
  - explications de chaque élément 

---
### Réalisation du circuit imprimé avec EasyEDA (5mn)
  ![pcb editor](images/pcb_avr.png)
  - Facilité de réalisation, le routage auto (démo live ?), les prix
---
# Partie afficheur  (13mn) - Charlotte Gil 
---
### Principe retenu pour l'assemblage en multi-couches : (5mn)
**3 Planches** 

- La première pour les lettres

  <img src="images/resine-lettres.png" alt="planche_lettres" style="zoom:50%;" />

---

- La seconde pour les mots

  

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-mots.png" alt="planche_mots" style="zoom:50%;" />

---

- La troisième pour les leds


<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-leds.png" alt="planche_leds" style="zoom:50%;" />

---

**La résine**

[Résine Epodex : https://www.epodex.com/fr/produit/pro-system](https://www.epodex.com/fr/produit/pro-system/)

![resine_epodex](/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-epodex.png)

---

- Découpe laser vs découpe CNC, les problèmes (3mn)
  > EASEL

  

---

Comment couler de la résine (3mn)

**Etape 1 : Mettre du scotch pour contenir la résine**

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-scotch.jpg" alt="resine-scotch" width="600" />

**Etape 2 : Mettre la couche des mots**

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-2PremieresCouches.jpg" alt="resine-2PremieresCouches" width="600" />

---

**Etape 3 : Préparer la résine.**

- Respecter les dosages (2 pour 1)

  <img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-Preparation.jpg" alt="resine-Preparation" width="200" />

- Bien remuer (5mn)

- Faire ça dans un environnement chaud 20°C

---

**Etape 4 : Couler la résine.**

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-onVerseLaResine.jpg" alt="resine-onVerseLaResine" width="600" />

**Etape 5 : Positionner les Leds**

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-sechageDesLeds.jpg" alt="resine-sechageDesLeds" width="600" />

---

**Test des différents types de LED**

- LED traversantes (30°-60°)

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-ledsTranversantes.jpg" alt="resine-ledsTranversantes" width="350" />

- LEDs cms (120°)

<img src="/home/gil/Projets/horloge-a-mots/doc/dtd2022/images/resine-ledCms.jpg" alt="resine-ledCms" width="350" />



---
# Développement (13mn)
---
- L'environnement PlatformIO (5mn) - Florent & Seb
  > Pour ceux qui ne connaissent pas et sont restés sur Arduino
  
- Description du code, conception détaillée (3mn) - Xav + Seb
- Comment on (essaie (péniblement) de) (on a brillament su) faire en Rust (3mn) - Xav + ?
---
# Conclusion
## Envoyez des sioux ! (à l'arc) !
