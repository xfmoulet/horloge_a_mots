# Horloge à Mots
Par Charlotte, Gil, Sebastien, Florent, François-Xavier, Xavier-Frédéric...

## Liens

- Gitlab: https://gitlab.tech.orange/coderoom-atalante/jeudis-electronique/horloge-a-mots
- EasyEDA: https://easyeda.com/editor#id=7d74d455c30843a59e121c8b445ea3cb|077fc49e57d04595857739c424e2151a
- Easel: https://easel.inventables.com/projects/dvpuCEvz0pOS_8ivUtRTIw
- Datasheet ATMega8A : https://ww1.microchip.com/downloads/en/DeviceDoc/en590320.pdf

## Schemas 

![board](doc/dtd2022/images/board3d.png)
![schema](doc/dtd2022/images/schema_avr.png)

## Calcul Resine
pour une planche de mot de 36*36 => calcul-volume-resine.xlsx
environ 55 cl

## Matrice de LED

- lignes et colonnes
- 11 lignes, ~3 mots par ligne ; environ 5 LED par ligne
- si on veut simplifier le câblage : 2 lignes en une (2x3 = 6) et ~ 6 LEDs 
-> OK avec 6x6 LED 

"Schéma": 
```
     c1/4  c2/5 c3/6
      | |   | |  | |
L1  --x-|---x-|--x-|--\
      | x---|-x--|-x--/
      | |   | |  | |
L2  --x-|---x-|--x-|--\
      | x---|-x--|-x--/
      | |   | |  | |
L3  --x-|---x-|--x-|--\
      | x---|-x--|-x--/
...
```
- on reste un peu plus longtemps en PWM sur les LEDs doubles / triples pour compenser la répartition de courant
- selon la datasheet § "5.3.14 IO Ports Characteristics", page 62 "Output driving current" : 

> The GPIOs (general purpose input/outputs) can sink or source up to ±6 mA, and up to
±15 mA with relaxed VOL/VOH. 

- Les mesures que nous avons réalisées avec une LED blanche indiquée 2.4V + arduino (5v): 

resistance | courant LED | voltage LED
-----------|-------------|-------------
100 Ω | 15 mA | 2.9V
150 Ω | 11 mA | 2.8V

Donc le voltage reste ~constant et on module le courant avec la résistance.

Avec un multiplexage de 1/36 (et une résistance légèrement plus faible), la LED reste bien visible.


