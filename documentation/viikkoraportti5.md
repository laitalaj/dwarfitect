# Viikkoraportti 5
*3.10. - 16.10.2016*

**3.10.**
Iltapäivällä refaktoroin tietorakenteita ja lisäsin niihin toiminnallisuutta.

Aikaa käytetty 0,5h.

**4.10.**
Lisäsin toiminnallisuutta ja testejä tietorakenteisiin.

Aikaa käytetty 1h.

**8.10.**
Loin erikoistuneen matriisitietorakenteen tulevia exporttaamisia ajatellen. Lisäsin myös toiminnallisuuden
muutenkin lopputulosten tulostamisen tiedostoon tekstimuodossa.

Aikaa käytetty 2h.

**10.10.**
Koska Rustista puuttuu luokka-inheritanssi, käytin de facto kiertotietä tähän ja refaktoroin
structin sisällä olevan Rectin manipulaatioon liittyvät metodit makroon, joka implementoi ne
annetun structin annetulle kentälle. Näin saan samat neliöön liittyvät metodit implementoitua
sekä Gene- että Room-structeille yhdellä rivillä.

Tämän lisäksi aloitin relax-metodin uudelleenkirjoituksen; vanha toiminnallisuus siirsi geenejä
aina samaan suuntaan, minkä seurauksena koko kromosomi liukui vähitellen oikealle alaspäin jolloin
geenien naittaminen "freeseille" geeneille ei koskaan tuottanut hyviä tuloksia.

Siirryin myös pois hassusta i16-integerien käyttämisestä isize-kokoisiin.

Aikaa käytetty 1,5h

**15.10.**
Viimeistelin relaxin uudelleenkirjoituksen, ja sen jälkeen outputteja katsellessani huomasin,
että alkupopulaatio-kromosomien luonti toimii aivan väärin. Korjasin tämän toimimaan siten, miten
olin ajatellut, ja lisäsin vähän satunnaisuutta sekoitukseen.
