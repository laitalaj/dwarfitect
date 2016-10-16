# Toteutusdokumentti

## Ohjelman yleisrakenne
Ohjelma kääntyy sekä kirjastoksi että suoritettavaksi
binääriksi. Suurin osa toiminnallisuudesta löytyy kirjastosta.

Ohjelma koostuu neljästä suuremmasta moduulista:
collections, genetics, io ja mapping. 

Rustille idiomaattisesti yksikkötestaus löytyy samasta tiedostosta
kuin itse toiminnallisuus. Tulen luultavasti tulevaisuudessa luomaan
vielä tests-moduulin integraatiotestejä varten.

### Collections-moduuli 
Collections-moduuli sisältää itsekirjoittamani tietorakenteet,
ResizableMemory, Vector sekä Matrix, sekä näiden toiminnallisuuden.

### Genetics-moduuli
Genetics-moduuli on ohjelman toiminnan kannalta tärkein moduuli;
itse geneettisen algoritmin pyöriminen löytyy täältä. Moduuli
on jaettu kahteen osaan: breeding ja genes. Genes sisältää geenit ja
kromosomit, tietueet joiden manipulointiin kaikki taikuus perustuu.
Breeding sisältää isoa kasaa kromosomeja manipuloivia funktioita.

### IO-moduuli
IO-moduuli sisältää syötteen ja tulostuksen käsittelyn. Toistaiseksi
moduulista löytyy vain alimoduuli output ja sieltä funktio save, joka
tallentaa merkkimatriisin tekstitiedostona.

### Mapping-moduuli
Mapping-moduuli sisältää perusgeometriaa alimoduulissa shapes, sekä
parhaimmaksi todetusta kromosomista tehdyn "kartan" manipulointia
alimoduulissa rooms.


## Saavutetut aika- ja tilavaativuudet
TBD

## Puutteet ja parannusehdotukset
TBD

## Lähteet
TBD