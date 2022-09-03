# Usinage

Probleme initial, probleme de profondeur de l'axe Z. obligé de percer a 9mm pour un morceau de bois de 3mm.

Il faut utiliser le compte de Marvin, "machine Inspector" et utiliser la console pour changer les paramètres

## default value sur EASEL
axe   $100=26.660 
axe   $101=26.660
axe Z $102=46.660

## new value 
axe  $100=26.660
axe  $101=26.660
axe Z $102=200 , erreur de profondeur , 150 pas assez.

D'apres la documentation :
- https://inventables.gitbook.io/x-carve-assembly
- https://inventables.gitbook.io/x-carve-assembly/calibration-and-maintenance
- https://discuss.inventables.com/t/grbl-default-settings/47158/6
  
## a tester
$100 = 40.054 (x, step/mm)
$101 = 40.054 (y, step/mm)

The default for $102 is 188.947 per GRBL ((Microstepping * Steps/rev) / MM_Per_rev_Z = (2 * 200) / 2.117 = 188.947)

## valeur validé :
$102 = 188.443 (z, step/mm)
# Gugi.zip 
la police utilisé pour l'horloge

# FINALhorlogeSVG.zip
l'export du projet easel

# compte easel 
mot de passe dans le excel Onglet:charlotte

