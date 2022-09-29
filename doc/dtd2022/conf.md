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
- Planche de lettres remplie de résine
- Séparateur de mots rempli de résine
- Eclairage

- Découpe laser vs découpe CNC, les problèmes (3mn)
  > EASEL

- Comment couler de la résine (3mn)

- Choix des LED (2mn)
  > Tests d'illumination des LEDs, CMS ou classiques
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
