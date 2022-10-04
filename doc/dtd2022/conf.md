title: Horloge à mots au Secteur 3
author:
  name: Florent, François-Xavier, Gil, Charlotte, Seb, Xavier-Frédéric
controls: false
theme: ./orange-theme

---

# Une Horloge à mots
## Les jeudis de l'electronique au secteur 3

---

### C'est quoi le secteur 3 et la code room ?

- Infos utiles :
  - Ouvert à tous !
  - Activités (coding dojos, jeudi électronique, rencontre agile)
  - Le programme : https://plazza.orange.com/groups/tiers-lieu-rennes

- Quelques vieux projets:
  - Borne d'arcade
  - TomTom
  - Afficheur led rotatif
  - Développement d'un jeu GameBoy
  - linky
  - Horlogeobus

---

### Le projet : l'horloge à mots

<center><img src="images/horloge.png" width="500"><img src="images/horloge-gold.png" width="500"></center>

- Nos objectifs : faible coût, composants simples

---

# Composants et design de la board

<div class="timer">Timer: 40mn</div>

---

### Préambule : tout ce qu'on a souhaité écarter

- Les rubans de LEDs adressables 
  
  <img src="images/ruban-leds.jpg" width="300">
  <img style="float: right;" src="images/esp8266.jpg" width="500">

  
- Connexion Wi-Fi, mise à jour par NTP
- Arduino, C

  <img src="images/arduino.jpg" width="300">

---

### Ce qu'on doit conserver

- Allumer des LEDs
- Garder l'heure juste
- Régler l'heure
  - éventuellement
  - comment ?

---

### La problématique de base : comment piloter une LED ?

  - **Courant** : dans la datasheet, ~20mA pour une LED blanche 
  - **Tension** aux bornes d'une LED : dans la datasheet, ~3V (quelques mesures pour contrôler)

  ![LED datasheet](images/led_specs.png)

  - Contraintes d'un microcontrôleur : quelques dizaines de mA par GPIO 
  - Sinon, il faut un circuit plus complexe

---

### Piloter 30 LEDs : 1/3

  - 1 pin / LED : 30 IO
    - MCU plus chers, plus complexes à souder
    <img alt="lqfp64" src="images/lqfp64.png"  style="float: right;">
    - pas possible de tout allumer en même temps sans *driver* 30\*30mA = 1 A !

  - Circuit supplémentaires 
    - 74HC595 : registre à décalage
    <img alt="74hc595" src="images/74hc595.png"  style="float: right;">
    - 2 pattes pour piloter toutes les LED (chaînables)
    - 8 LED / circuit : 4 circuits x 10cts ! 
    - Plus complexe a souder / réaliser plus complexe a réaliser.

---

### Piloter 30 LEDs : 2/3

  ![multiplex](images/led-matrix.png)

- Multiplexage ligne/colonne : (Nb<sub>GPIO</sub>/2)² LEDs

- Pour 30 LEDs, il faut 2\*√30 = 12 GPIOs :
  - Un MCU à 20 pattes convient

- 1 LED allumée à la fois
  - Joue sur la persistence rétinienne
  - Contrôle du courant 
  - Assez rapide pour ne pas clignoter

---

### Piloter 30 LEDs : 3/3

![charlieplex](images/charlieplex-tr.png)

- CharliePlex: Nb<sub>GPIO</sub>² - Nb<sub>GPIO</sub> (\~2001)
- 2 LED tête bêche, emploie le fait d'éteindre une IO
- 6 GPIOs => 30 LEDs ! 

----

### Piloter 30 LEDs : 3/3

![charlieplex](images/charlieplex-2.png)

- Exemple pour allumer la LED1 :
  - X1 à Vcc
  - X2 à la masse
  - X3 déconnecté
- Beaucoup plus complexe, pas nécessaire

---

### Garder l'heure

- Avec un quartz et des circuits dédiés logiques
    - CD 4060: driver quartz + prédiviseur puissance de 2
      <img alt="CD4060" src="images/CD4060-pinout.png"  style="float: right;">
    - quartz pas chers: 32kHz = 32768Hz, 1s sans division
    - MAIS division par 3 et 5 complexes (60=5\*3\*2²)
      - depuis les babyloniens, difficile de redéfinir la minute, l'heure
      - tentative en 1793: 10h, minute décimale, seconde décimale      
    - plusieurs puces


- Avec un microcontrôleur : CPU + mémoire intégrée
  - oscillateur interne précis à 1% : 14min/jour ...
  - base de temps: Quartz 32kHz, 50Hz secteur 
  - besoin d'une horloge stable (temps, quartz) + 1 horloge "rapide" (CPU)

---

### Choix du micro-contrôleur

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

### Choix du micro-contrôleur : choix de la famille dans la gamme constructeur

<center>![famille](images/stm32-1.png)</center>

---

### Choix du micro-contrôleur : choix du modèle

<center>![modele1](images/stm32-2.png)
![modele2](images/stm32-3.png)</center>

---

# MAIS
## Choix beaucoup plus simple !

---

### Choix sur un site connu : Atmega328P

- atmega328P (arduino), rechercher ...
<center>![rechercher mouser](images/achat328-1.png)</center>
- choix packages 
- click sur "in stock"

---

### Choix sur un site connu : atmega328P
<center>![rechercher mouser](images/achat328-2.png)</center>

---

### Choix sur un site connu : atmega328P
- Autre site ! 

<center>![rechercher mouser](images/achat328-3.png)</center>

---

### Algorithme V2

- Choisir un constructeur
- Sélectionner "en stock"
- Prendre le moins cher
- S'adapter

<center>![Improvise, adapt, overcome](images/improvise-adapt-overcome.png)</center>

---

### Algorithme V2

![rechercher lcsc](images/achat328-4.png)

- * note: les Attiny, moins puissants, ne sont pas moins chers 

---

### Comment régler l'heure ?

  - Avec un ou plusieurs boutons ?
    <img alt="button" src="images/push-button.jpg"  style="float: right; width: 300px;">
    - Des GPIO en plus 😒

  - Avec un détecteur infrarouge ?
    <img alt="PIR" src="images/pir-sensor.jpeg"  style="float: right; width: 400px;">
    - Une ergonomie douteuse 🤨

  - Aucune entrée utilisateur
    - Facile à coder !
    - Pas de bug 🥰


---

### Le schéma de la board

  <img alt="Schéma" src="images/schema_avr.png" width="80%">

---

### Réalisation du circuit imprimé avec EasyEDA
  <center>![pcb editor](images/pcb_avr.png)</center>
  - Facile à réaliser, routage automatique

---

### Au final

<center>![3D schema](images/board3d.png)</center>

- LCPCB (china): 2 semaines, 10€ pour 15 boards

---

### Au final

<center><img alt="Boards finies" src="images/boards.jpg" width="80%"></center>

---

# L'afficheur

<div class="timer">Timer: 25mn</div>


---

### Principe retenu pour l'assemblage en multi-couches

**3 Planches** 

<div style="display:grid; grid-template-columns: repeat(2, 1fr);">
<span>- La première pour les lettres</span>
<span>- La seconde pour les mots</span>
![planche lettres](images/resine-lettres.png)
![planche mots](images/resine-mots.png)
</div>

---

### Principe retenu pour l'assemblage en multi-couches

<span>- La troisième pour les leds</span>
<center>![planche leds](images/resine-leds.png)</center>

---

### Découpe laser vs découpe CNC, les problèmes
  > EASEL

  <center>![decoupe CNC](images/decoupe.jpg)</center>

---

### La résine

[Résine Epodex : https://www.epodex.com/fr/produit/pro-system](https://www.epodex.com/fr/produit/pro-system/)

![resine_epodex](images/resine-epodex.png)

---

### Comment couler de la résine

**Etape 1 : Mettre du scotch pour contenir la résine**

<img src="images/resine-scotch.jpg" alt="resine-scotch" width="600" />

**Etape 2 : Mettre la couche des mots**

<img src="images/resine-2PremieresCouches.jpg" alt="resine-2PremieresCouches" width="600" />

---

### Comment couler de la résine

**Etape 3 : Préparer la résine.**

- Respecter les dosages (2 pour 1)

  <img src="images/resine-Preparation.jpg" alt="resine-Preparation" width="200" />

- Bien remuer (5mn)

- Faire ça dans un environnement chaud 20°C

---

### Comment couler de la résine

**Etape 4 : Couler la résine.**

<img src="images/resine-onVerseLaResine.jpg" alt="resine-onVerseLaResine" width="600" />

**Etape 5 : Positionner les Leds**

<img src="images/resine-sechageDesLeds.jpg" alt="resine-sechageDesLeds" width="600" />

---

### Test des différents types de LED

- LED traversantes (30°-60°)

<img src="images/resine-ledsTranversantes.jpg" alt="resine-ledsTranversantes" width="350" />

- LEDs cms (120°)

<img src="images/resine-ledCms.jpg" alt="resine-ledCms" width="350" />

---

# Développement

<div class="timer">Timer: 12mn</div>

---

### Environnement PlatformIO - Florent & Seb

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
  - ex: 35 -> "moins" "vingt" "cinq" (et +1 heure) (attention à 23h) 
  - ex: 15 -> "et" "quart" 
  - Génération de tableau en Flash
   - Heures et minutes indépendantes: 60x12=720 vs 60+12=72
  - Génère un tableau de LEDs à allumer parmi N max
  - une LED -> une ligne, une colonne (un tableau, ou directement précalculé)
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

**Un screenshot du code peut-être ?**

- Tout est fortement typé / sécure
  - Move semantics
  - Borrow checker
  - Mutex sur matériel
- Mais se compile en qq octets (Cf. make info) : 1o de RAM
- Option&lt;LED&gt;[10] en flash
- Prégénération de code en rust avec build.rs
- Crate `embedded_hal`
- ARM32 mieux que AVR (dispo)
- https://gitlab.tech.orange/coderoom-atalante/coding-dojos/embedded-rust

---

# Conclusion
## Envoyez des sioux ! (à l'arc) !

<center>![fablab image](images/fablab.JPG)</center>