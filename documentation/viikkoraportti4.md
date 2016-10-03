# Viikkoraportti 3
*26.9. - 3.10.2016*

**28.9.**
Tein yksinkertaisen main-funktion, joka laskee tietynlaista populaatiota ja tulostaa välillä fitness-arvoja. Korjasin myös ongelman,
jossa keskikokoisella todennäköisyydellä ohjelma panikoi kandidaatti-binäärihaun jälkeen johtuen siitä, ettei binäärihaku
löytänyt mitään.

Tarkastelin myös mahdollisuuksia tallentaa ratkaisu kuvana, ja sähläsin populaation parhaiden säästämiseen seuraavaan populaatioon
kanssa - yritin aluksi muuttaa populaation Vec<Chromosome>:sta Vec<&Chromosomeksi>, mutta tämä johti elinaikaongelmiin.

Aikaa käytetty noin 3h

**29.9.**
Korjasin lisää bugeja binäärihaussa ja sain populaation parhaat elämään myös seuraavassa populaatiossa; unohdin Vec<&Chromosome>-idean
ja sen sijaan vain kopioin lainattuja kromosomeja seuraavaan populaatioon, pitäytyen Vec<Chromosome>:ssa.

Aikaa käytetty noin 1,5h

**30.9.**
Korjasin fitness-funktion käyttämään kymmenkantaista logaritmia ln:än sijasta, joten nykyään se toimii jopa oikein. Kirjoitin myös
purge-funktion, joka korvaa tietyn osan populaatiosta "alkukantaisilla" jäsenillä. Tällä funktiolla on hyvä saada kehittymisen lopettanut
populaatio taas vauhtiin (ainakin teoriassa, vielä ei toimi kovinkaan hyvin...).

Painin myös omien tietorakenteiden aloittamisen kanssa; heap_api, jota tarvitaan heapin suoraan manipulointiin, ei nähtävästi toimi muussa
kuin nightly-rustissa, joten asensin nightlyn. Nähtävästi kyseinen ominaisuus ei kuitenkaan toimi siinäkään...

Aikaa käytetty noin 3,5h

**2.10.**
Jatkoin nightlyn kanssa tappelemista. Asensin rustup-toolchainin, jonka nightly-versio näköjään ei compilannut kunnolla rand-pakettia ja yritti sen sijaan
käyttää standardikirjanston (epävakaata) random number generaattoria (ja siten valitti siitä, että kyseinen generaattori on epävakaa). Paljon vaivaa yhden
ominaisuuden käyttöönottamiseksi...

Ohessa tein vertaisarvioinnit.

Aikaa käytetty noin 2,5h

**3.10.**
Vaihdoin rustupissa nightlystä stableen ja takaisin nightlyyn, ja nyt kaikki näyttäisi maagisesti toimivan. Sain siis vihdoinkin oman
alkeellisen tietorakenneimplementaation compiloimaan ja testautumaan. Voi pojat, näitä lisää ensi viikolla.

Tämän lisäksi kirjoittelin vähän testejä ja dokumentaatiokommentteja.

Aikaa käytetty noin 1,5h

Aikaa käytetty yhteensä tällä viikolla noin 11h