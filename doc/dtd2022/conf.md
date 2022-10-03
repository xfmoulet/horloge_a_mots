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
---
### C'est quoi le secteur 3 et la code room ? (3mn)

- Infos utiles :
  - ouvert à tous !
  - Activités (coding dojos, jeudi électronique, rencontre agile)

- Quelques vieux projets:
  - Borne d'arcade
  - TomTom
  - Afficheur led rotatif
  - Développement d'un jeu GameBoy

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
- Arduino, C
---
### Ce qu'on doit conserver
- Afficher des LEDs
- Garder l'heure juste
- (éventuellement) régler l'heure
---
### La problématique de base : comment piloter XX leds ? (5mn)
  - Quand XX = 1 : 
     - courant: dans la spec
     - voltage d'une LED: dans la datasheet de la LED, ~3V pour une LED blanche. Qq mesures pour contrôler
     - contraintes d'un microcontrôleur: IO qq dizaines mA OU circuit plus complexe
     ![LED datasheet](images/led_specs.png)
---
### Piloter XX=30 LEDs : 1/3
  - 1 pin / LED : 30 IO
    - MCU plus chers, plus complexes à souder
    - ![lqfp64](images/lqfp64.png)
    - pas possible de tout allumer en même temps sans *driver* 30\*30mA = 1 A !
  - Circuit supplémentaires 
    - 74HC595 : série -> parallèle
    - 2 pattes pour piloter toutes les LED (chaînables)
    - 8 LED / circuit : 4 circuits x 10cts ! 
    - Plus complexe a souder / réaliser plus complexe a réaliser.
---
### Piloter 30 LEDs : 2/3
  ![multiplex](images/led-matrix.png)
- Multiplexage ligne/colonne: (n/2)²
- De N=30 à 2\*√N = 12 ! : 
  - MCU à 20 pattes convient
- 1 LED allumée à la fois
  - joue sur la persistence rétinienne
  - Contrôle du courant 
  - Assez rapide pour ne pas clignoter

---
### Piloter 30 LEDs : 3/3
![charlieplex](images/charlieplex.png)
- CharliePlex: n²-n (\~2001)
- 2 LED tête bêche, emploie le fait d'éteindre une IO    
- 6 pattes => 30 pins ! 
- beaucoup plus complexe, pas nécessaire
---
### Garder l'heure

- quartz + circuits dédiés logiques
    - CD 4060: driver quartz + prédiviseur puissance de 2
    - quartz pas chers: 32kHz = 32768Hz, 1s sans division
    - MAIS division par 3 et 5 complexes (60=5\*3\*2²)
      - depuis les babyloniens, difficile de redéfinir la minute, l'heure
      - tentative en 1793: 10h, minute décimale, seconde décimale      
    - plusieurs puces
- microcontrôleur : CPU + mémoire intégrée
  - oscillateur interne précis à 1% : 14min/jour ...
  - base de temps: Quartz 32kHz, 50Hz secteur 
  - besoin d'une horloge stable (temps, quartz) + 1 horloge "rapide" (CPU)

---
### Choix du microncontrôleur

| Quoi | Caractéristiques | Besoin |
|---|-----|----|
| CPU | 4-32 bits, 1-250 MHZ, ARM/AVR/Autre? ✖ ➗ ? | compilateur, 8/32 bits, 1MHz OK |
| RAM interne | 0 (!) à 1MB | code actuel: 1 octet |
| Flash interne | de 0k (OTP), 2Ko à 2Mo | 4Ko |
| Périphériques | 1-10+ Timers, n UART, SPI, I2C, CAN ... | Timer Quartz 32KHz |
| IO | Nb pattes (boîtier), mA | 12 GPIO, qq dizaines mA ! |
| Horloges | PLL, Osc interne / Quartz, ... | Osc. Int High speed ou PLL |
| Power | 3v3, 1v2, 5v ? mA: Low power ? Sleep ? | 5V plus simple, sinon 3v3|
| Programmateur | USB, UART, SWD/SPI, autre high power .. | USB, UART |


---
### Choix d'un constructeur
- Exemple de constructeurs
  - Padauk: 1k OTP, 64oRAM, 3cts MAIS programmateur cher, langage pseudo-C 
  - AVR Atmega328P (arduino): 8bits connu, fiable, pas cher*
  - AVR Attiny : Idem, moins de mémoire, périphériques
  - STM32: de 0.5€ -> qq dizaines d'euros, de 4 à 150 IOs, de 32 à 550MHz, 8k -> 2Mo flash ... 
  - Microchip PIC, NXP (philips), STM8, TI, Cypress, Renesas, 8051 ...
- Notre choix
  - STM32 : 
    - très large famille (64MHz, crystal, 32/8ko, 1€, 20pin)
    - simple à programmer (ARM, programmateurs très répandus, USB)
    - MCU disponibles peu chers 
    - version 1 de la board
  - MAIS AVR: 5V (alim directe par USB 5V), IO plus puissantes  
---
### Choix du microncontrôleur: choix de la famille dans la gamme constructeur
![famille](images/stm32-1.png)
---
### Choix du microncontrôleur: choix du modèle
![modele1](images/stm32-2.png)
![modele2](images/stm32-3.png)
---
# MAIS
## Choix beaucoup plus simple !
---
### Choix sur un site connu: Atmega328P
- atmega328P (arduino), rechercher ...

![rechercher mouser](images/achat328-1.png)
- choix packages 
- click sur "in stock"
---
### Choix sur un site connu : atmega328P
![rechercher mouser](images/achat328-2.png)
---
### Choix sur un site connu : atmega328P
- Autre site ! 

![rechercher mouser](images/achat328-3.png)
---
### Algorithme V2
- Choisir un constructeur
- Sélectionner "en stock"
- Prendre le moins cher
- S'adapter
---
### Algorithme V2

![rechercher lcsc](images/achat328-4.png)

- * note: les Attiny, moins puissants, ne sont pas moins chers 
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
### Au final
![3D schema](images/board3d.png)
- LCPCB (china): 2 semaines, 10€ pour 15 boards
---
### Au final
![boards finies](images/boards.jpg)
---
# Partie afficheur  (13mn) - Charlotte Gil 
---
### Principe retenu pour l'assemblage en multi-couches : (5mn)
**3 Planches** 

- La première pour les lettres

![planche lettres](images/resine-lettres.png)
  
---

- La seconde pour les mots

  
![planche mots](images/resine-mots.png)

---

- La troisième pour les leds

![planche leds](images/resine-leds.png)

---

**La résine**

[Résine Epodex : https://www.epodex.com/fr/produit/pro-system](https://www.epodex.com/fr/produit/pro-system/)

![resine_epodex](images/resine-epodex.png)

---

- Découpe laser vs découpe CNC, les problèmes (3mn)
  > EASEL

  

---

Comment couler de la résine (3mn)

**Etape 1 : Mettre du scotch pour contenir la résine**

<img src="images/resine-scotch.jpg" alt="resine-scotch" width="600" />

**Etape 2 : Mettre la couche des mots**

<img src="images/resine-2PremieresCouches.jpg" alt="resine-2PremieresCouches" width="600" />

---

**Etape 3 : Préparer la résine.**

- Respecter les dosages (2 pour 1)

  <img src="images/resine-Preparation.jpg" alt="resine-Preparation" width="200" />

- Bien remuer (5mn)

- Faire ça dans un environnement chaud 20°C

---

**Etape 4 : Couler la résine.**

<img src="images/resine-onVerseLaResine.jpg" alt="resine-onVerseLaResine" width="600" />

**Etape 5 : Positionner les Leds**

<img src="images/resine-sechageDesLeds.jpg" alt="resine-sechageDesLeds" width="600" />

---

**Test des différents types de LED**

- LED traversantes (30°-60°)

<img src="images/resine-ledsTranversantes.jpg" alt="resine-ledsTranversantes" width="350" />

- LEDs cms (120°)

<img src="images/resine-ledCms.jpg" alt="resine-ledCms" width="350" />

---
# Développement (13mn)
---
### Environnement PlatformIO (5mn) - Florent & Seb
> Pour ceux qui ne connaissent pas et sont restés sur Arduino 
  
  - Plate-forme pour le développement embarqué, IoT, Arduino, CMSIS, ESP-IDF, FreeRTOS
  - Intégration à VSCode sous forme de plugin
  - Installation et gestion facilitée pour les boards
  - Installation et gestion facilitée des librairies
  - (Non testé) des possibilités avancées de debug directement sur la board
  - Possibilité d'utiliser un vrai IDE
---
  <div class="largeImage">![platormIO home](images/platformio_home.png)<div>
---
  <div class="largeImage">![platormIO ini](images/platformio_ini.png)<div>
---
  
- Description du code, conception détaillée (3mn) - Xav + Seb
---
# Code de l'horloge
---
### Compter l'heure (précisément)
- un *timer* 32kHz
   - un timer HW : un compteur de N à zero, et recommence
   - compter 1 minute = 60 secondes = 1966080 clocks
   - compteur ~~32~~ 8 bits 
   - sans division (ou peu)
- 32kHz = 32768 Hz ! 
  - pré-diviser par 1024 en HW -> 32Hz
  - 1 minute = 32768/1024 * 60 = 1920 ticks > 255 :(
  - Rollover : diviseur entier de 1920 <255 : 6 (\~0.19s), 10, 15, 30, **32** (1s), 60, 64, 128, 192 (6s.)
- Algo
  - lire ticks timer "fréquemment" :
   - si `ticks` plus petit que valeur précédente: +1 tour
  - alternative : interruption sur rollover
  - si nb tours > rollover, augmenter secondes, minutes, heures ...
---
### Choisir les LED à allumer (et le faire)
- Choisir les LED
  - à partir d'une minute donnée, choisir les LED "minutes" à allumer
  - ex: 35 -> "moins" vingt" "cinq" (et +1 heure) (attention à 23h) 
  - ex: 15 -> "et" "quart" 
  - Génération de tableau en Flash
   - Heures et minutes indépendantes: 60x12=720 vs 60+12=72
  - Génère un tableau de LEDs à allumer parmi N max
  - une LED -> une ligne, une colonne (un tableau, ou directement précalc)
- multiplex
  - boucler rapidement sur les LED du tableau
  - certaines LEDs sont doubles: rester plus longtemps dessus -> ds le tableau  
- fonctions "avancées": réglage de l'heure, détection si quelqu'un passe, ...
  - TODO !
  - Par exemple appuyer sur le bouton reset à un moment précis (par exemple 13h37 !)
---
# Rust 🦀
## Comment on (essaie (péniblement) de) (on a brillament su) faire en Rust
---
### Rust sur microcontrôleur 
- Tout est fortement typé / sécure
  - Move semantics
  - Borrow checker
  - Mutex sur matériel
- Mais se compile en qq octets (Cf. make info) : 1o de RAM
- Option<LED>[10] en flash
- Prégénération de code en rust avec build.rs
- Crate `embedded_hal`
- ARM32 mieux que AVR (dispo)
---
# Conclusion
## Envoyez des sioux ! (à l'arc) !
