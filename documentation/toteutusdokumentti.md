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
Analysoin tässä lähinnä ohjelman tärkeimmän toiminnallisuuden eli
geneettisen algoritmin aika- ja tilavaativuutta.

### Aikavaativuus
Merkinnöissä n on populaation koko, m geenien määrä kromosomissa ja g 
sukupolvien määrä.
Yhden sukupolven aikana...
+ ...populaatio kopieoidaan. Tähän kuluu aikaa O(n) (jokainen jäsen kopioidaan
yksi kerrallaan)
+ ...populaatio järjestetään sen sopivuuden perusteella. Tähän käytetään
Rustin slicen vakiojärjestämistä, joka toimii ajassa O(n log n).
+ ...populaatio muutetaan kandidaateiksi. Tätä varten populaatio käydään
kahteen kertaan läpi alusta loppuun ja muut operaatiot ovat vakioaikaisia, 
joten aikavaativuus on O(n).
+ ...tietty prosenttiosuus populaatiosta siirretään uuteen populaatioon.
Tässä vakioaikaisia operaatioita täytyy tehdä an kappaletta, missä 0<=a<=1. 
Aikavaativuus on siis O(n).
+ ...loppupopulaatio täytetään lapsilla parittamalla vanhan populaation
jäseniä. Tämän yhteydessä
..+ etsitään binäärihaulla kaksi kandidaattia satunnaisgeneroidun luvun 
perusteella. Aikavaativuus O(log n).
..+ paritetaan kaksi kromosomia toisiinsa. Tällöin
....+ käydään geenit läpi ja lisätään niitä lapsiin; aikavaativuus O(m)
....+ luodaan kaksi uutta kromosomia. Luomisen yhteydessä relaksoidaan
kromosomit, eli varmistetaan ettei mikään geeni törmää toiseen. Tässä
geeniä aluksi siirretään, kunnes se ei enää törmää mihinkään jo "jäädytettyyn"
geeniin, ja sitten se jäädytetään. Pahimman tapauksen aikavaativuus on
siis luokkaa O(m^2).
..+ -> Siispä parittelun aikavaativuus on O(m^2)
..+ parittelun tulokset mutatoidaan kokeilemalla, mutatoituuko jokin
geeneistä käyttäen muuten vakioaikaisia operaatioita. Aikavaativuus
on siis O(m).
+ -> Tätä toistetaan (n - an)/2 kertaa, missä 0<=a<=1, eli O(n) kertaa.
"lapsillatäyttösilmukan" sisäosien aikavaativuus on O(log n + m^2), joten
lapsillatäyttösilmukan kokonaisaikavaativuus on O(n log n + nm^2)

Näistä huomataan, että yhden sukupolven operaatioiden aikavaativuus on sama
kuin sen vaativimman operaation ("lapsillatäyttösilmukan") aikavaativuus eli
O(n log n + nm^2). Sukupolvia iteroidaan g kertaa, joten kokonaisaikavaativuus
geneettiselle algoritmille on O(gn log n + gnm^2). Saavutettu aikavaativuus on siis
hieman suurempi kuin määrittelydokumentissa arvioitu.

### Tilavaativuus

Merkinnöissä n on populaation koko, m geenien määrä kromosomissa ja g 
sukupolvien määrä.
Yksi sukupolvi vie tilaa O(nm) verran. Yhden sukupolven aikana...
+ ...populaatio kopioidaan. Tämä vaatii O(nm) lisää tilaa.
+ ...populaatio järjestetään sen sopivuuden perusteella. Tähmä tapahtuu
tilassa O(log n).
+ ...populaatio muutetaan kandidaateiksi. Tämän tilavaativuus on O(n)
+ ...tietty prosenttiosuus populaatiosta siirretään uuteen populaatioon.
Tässä kopioidaan an kappaletta kromosomeja, missä 0<=a<=1. 
Tilaa vaaditaan siis taas  O(nm).
+ ...loppupopulaatio täytetään lapsilla parittamalla vanhan populaation
jäseniä. Tämän yhteydessä
..+ etsitään binäärihaulla kaksi kandidaattia satunnaisgeneroidun luvun 
perusteella. Tilavaativuus vakio.
..+ paritetaan kaksi kromosomia toisiinsa. Tällöin
....+ käydään geenit läpi ja lisätään niitä lapsiin; Tilavaativuus O(m)
....+ luodaan kaksi uutta kromosomia. Luomisen yhteydessä relaksoidaan
kromosomit, eli varmistetaan ettei mikään geeni törmää toiseen. Tämän
tilavaativuus on O(m), ja luomisen kokonaisuudessaan tilavaativuus on myös
O(m).
..+ -> Siispä parittelun aikavaativuus on O(m)
..+ parittelun tulokset mutatoidaan. Tilavaativuus on O(m).
+ -> Lapsia tehdään n - an kappaletta, missä 0 <= a <= 1, joten uusien lapsien
tilavaativuus on O(nm).

Kokonaisuudessaan siis tilavaativuus on luokkaa O(nm), mikä on määrittelydokumentissa
arvioidun mukainen.



## Puutteet ja parannusehdotukset
Ajan puutteen takia jäivät määrittelydokumentissa mainitut A*-algoritmi sekä
palojen tiivistys tekemättä. Ne olisivat seuraavat lisättävät toiminnallisuudet.

## Lähteet
Omien tietorakenteiden tekemisessä auttoi suuresti
Example: Implementing Vec - The Rustonomicon, https://doc.rust-lang.org/nomicon/vec.html