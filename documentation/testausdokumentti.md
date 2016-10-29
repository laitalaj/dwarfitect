# Testausdokumentti

## Toteutus
Tällä hetkellä ohjelman toimintaa on testattu pääasiassa yksikkötesteillä.
Näillä testeillä tarkastellaan, toimiiko yksittäiset toiminnallisuudet odotusten
mukaisesti.

Myös jonkin verran manuaalista testausta on suoritettu silmämääräisen sekä
intuitiivisen tulosten tarkastelun muodossa.

## Yksikkötestit
Yksikkötestit löytyvät Rustille idiomaattisesti samoista tiedostoista kuin
missä itse toiminnallisuus on, alimoduulista tests. Kirjoitushetkellä yksikkötestejä
on 22 kappaletta, ja mikään niistä ei epäonnistu. Tavoitteena on testata suurin osa
toiminnallisuudesta yksikkötesteillä.

## Manuaalinen testaus
Tällä hetkellä ohjelman suoritus pyöräyttää käytännössä yhden kierroksen manuaalista
testausta. Ohjelma kertoo, kuinka monta sukupolvea on prosessoitu, mikä on parhaan
kandidaatin fitness-arvo, pinta-ala sekä geenien kokonaispinta-ala. Ohjelma myös tallentaa
tekstitiedostoihin alkupopulaation parhaan yksilön sekä loppupopulaation priimajäsenen.
Tämän avulla on mahdollista arvioida ohjelman kokonaistoimintaa.

## Testien toistaminen
Yksikkötestit voidaan ajaa esim. Rustin paketinhallintaohjelman kautta komennolla cargo test.
Benchmark-testit saa ajettua komennolla cargo bench.