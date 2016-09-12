# Määrittelydokumentti

## Ongelma

Tavoitteena on luoda ohjelma, joka toteuttaa heuristisen sijoitteluoptimisaatioalgoritmin määritellyn muotoisille paloille ruudukkoon.
"Optimaalisuus", jota tavoitellaan, on ennen kaikkea palojen välisen matkan- ja toisaalta hukkatilan minimaalisuus. Ohjelma myös yhdistää
palat toisiinsa poluilla niin vaadittaessa. Idea sai lähtönsä Dwarf Fortress-pelin linnoituksen rakenteen suunnittelusta - mahdollisimman
hyvässä linnoituksessa kääpiöt asuisivat mahdollisimman lähellä työ- ja ruokapaikkoja. Käytännössä kyse on siis facility layout-tyyppisen
ongelman erityistapauksesta.

## Työn laajuus
Työn laajuus on 4op.

## Ohjelmointikieli
Suurin osa toiminnallisuudesta (itse algoritmit ja tietorakenteet sekä niitä käyttävät funktiot) ohjelmoidaan Rust-kielellä. Ajan ja 
kurssin vaatimusten salliessa saatan lopussa tehdä ohjelmalle graafisen käyttöliittymän Pythonilla.

## Toteutetut algoritmit ja tietorakenteet
Ratkaisuna tähän NP-täydelliseksi [1] todettuun sijoitteluongelmaan aion käyttää geneettistä algoritmia, joka luo ja yhdistelee mahdollisia ratkaisuja
ja löytää näin riittävän hyvän ratkaisun tarpeeksi pienessä ajassa. Innoitusta ratkaisumalliin olen hakenut esimerkiksi artikkeleista [2] ja [3].
Kun sijoittelu on valmis, lasken riittävän hyvät polutukset käyttäen A*-polunlöytämistä mukailevaa menetelmää. Mahdollistan tämän jättämällä tyhjää
tilaa palojen väliin sijoitteluvaiheessa, joten tämän jälkeen vielä tiivistän ylimääräisen tyhjän tilan.

Tarvittavavia tietorakenteita ovat esimerkiksi geneettisen algoritmin kromosomeille tehokas struct-tyyppinen rakenne sekä ainakin valmiille sijoittelulle
ruudukko, josta selviää että mikä pala on missä kohtaa. Muitakin tietorakenteita tulee luultavasti tarvitsemaan - niitä implementoin sitten niiden tullessa
vastaan.

### Aika- ja tilavaativuudet
Geneettisen algoritmin aikavaativuudeksi tavoitteena on luokka O(p*i*n^2), missä p on populaation koko, i on iteraatioiden määrä ja n huoneiden määrä.
Tilavaativuuden olettaisin pysyvän tässä vaiheessa luokassa O(p*n).

Polkujen löytämisen aikavaativuus tulee olemaan - normaalin A*-toiminnalisuuden tapaan - O(e), missä e on polkuruutujen kokonaismäärä.
Tilavaativuus tässä vaiheessa on luultavasti jo luokkaa O(w^2), missä w on kaikkien huoneiden suurempien ulottuvuuksien (leveys tai korkeus)
summa.

Käyttämättömän polkutilan tiivistäminen onnistuu ahneesti käymällä kaikki palat järjestyksessä lähimmästä kaukaisimpaan ajassa O(n). Tilavaativuus
on tällöinkin O(w^2).

## Lähteet
[1]: Facility layout problem, Wikipedia, luettu 12.9.2016. https://en.wikipedia.org/wiki/Facility_location_problem
[2]: Tasadduq I.A., Imam M.H., Ahmad A-R, "A novel metasearch algorithm for facility layout optimization", http://www.usc.edu/dept/ise/caie/Checked%20Papers%20[ruhi%2012th%20sept]/word%20format%20papers/REGISTRATION%20PAID%20PAPERS%20FOR%20PROCEEDINGS/pdf/282%2013%20A%20NOVEL%20METASEARCH%20ALGORITHM%20FOR%20FACILITY%20LAYOUT%20OPTIMIZATION.pdf
[3]: Balakrishnan J., Cheng C-H, Wong K-F, "FACOPT: a user friendly FACility layout OPTimization system", Computers & Operations Research Volume 30, Issue 11, September 2003, Pages 1625–1641