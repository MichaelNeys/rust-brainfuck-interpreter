[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/yZP11lHz)
# BrainFuck

In deze taak moet je een [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter schrijven in Rust.

## Inleiding tot BrainFuck

Brainfuck is een minimalistische programmeertaal bestaande uit slechts acht commando's. Een Brainfuck programma werkt op een oneindig lange "tape" van geheugencellen, waarbij elke cel aanvankelijk de waarde 0 bevat. Elke cel kan een waarde van 0 tot 255 bevatten (waarden die hoger of lager zouden worden "wrappen" rond modulo 256). Er is een geheugenwijzer die bijhoudt welke cel momenteel actief is. Een brainfuck programma werkt heel gelijkaardig aan een Turing machine.

Hieronder een samenvatting van de acht commando's:

| Commando | Betekenis                                                             |
| :------: | :-------------------------------------------------------------------- |
|   `>`    | Verhoog de geheugenwijzer met 1.                                      |
|   `<`    | Verlaag de geheugenwijzer met 1.                                      |
|   `+`    | Verhoog de huidige cel met 1.                                         |
|   `-`    | Verlaag de huidige cel met 1.                                         |
|   `.`    | Print het ASCII-teken overeenkomend met de waarde van de huidige cel. |
|   `,`    | Lees één ASCII-teken in naar de huidige cel (EOF = 0).                |
|   `[`    | Als de huidige cel 0 is, spring verder naar het bijbehorende `]`.     |
|   `]`    | Als de huidige cel niet 0 is, spring terug naar het bijbehorende `[`. |

Alle andere tekens in een Brainfuck programma worden genegeerd.

De volgende website kan gebruikt worden om een Brainfuck programma stap-voor-stap uit te voeren: [Brainfuck Visualizer](https://ashupk.github.io/Brainfuck/brainfuck-visualizer-master/index.html).

### Hello, World!

Bijvoorbeeld, hieronder is een "Hello World!" Brainfuck programma:

```brainfuck
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
+++++++++++++++++++++++++++++.
+++++++.
.
+++.
>++++++++++++++++++++++++++++++++++++++++++++.
------------.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++.
++++++++++++++++++++++++.
+++.
------.
--------.
>+++++++++++++++++++++++++++++++++.
```

Om de leesbaarheid te verhogen, hebben we de code verdeeld in verschillende regels, waarbij elke regel één letter uitprint. De eerste regel bevat 72 plustekens, wat dus de waarde van de cel van 0 naar 72 verhoogt. 72 is de ASCII waarde van de letter H. De `.` print dit teken dan uit. Gebruik [deze website](https://ashupk.github.io/Brainfuck/brainfuck-visualizer-master/index.html#KysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrLgorKysrKysrKysrKysrKysrKysrKysrKysrKysrKy4KKysrKysrKy4KLgorKysuCj4rKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKy4KLS0tLS0tLS0tLS0tLgorKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrLgorKysrKysrKysrKysrKysrKysrKysrKysuCisrKy4KLS0tLS0tLgotLS0tLS0tLS4KPisrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKy4=) om het programma commanda-voor-commando uit te voeren.

### Repeat

Het volgende programma leest 1 ASCII-getal $X$ in, en print dan $X$ keer een uitroepteken `!` uit.

```brainfuck
+++++++++++++++++++++++++++++++++                    # Cel 0 = '!' (33)
>++++++++++++++++++++++++++++++++++++++++++++++++    # Cel 1 = '0' (ASCII 48)
>,                                                   # Lees invoer (ASCII getal) naar cel 2
<[->-<]                                              # Trek waarde '0' (ASCII 48) af van cel 2
>[-<<.>>]                                            # Gebruik waarde in cel 2 om cel 0 zoveel keer te printen
```

Bijvoorbeeld, voeren we het programma uit met als invoer "5", dan print het programma vijf uitroeptekens uit:
`!!!!!`.

Opnieuw kan kan dit programma stap-voor-stap uitgevoerd worden met [deze website](https://ashupk.github.io/Brainfuck/brainfuck-visualizer-master/index.html#KysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrCj4rKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysKPiwKPFstPi08XQo+Wy08PC4+Pl0).

## Opdracht

Schrijf een Brainfuck interpreter in Rust. Ze accepteert twee positionele argumenten:

- Een bestand met een Brainfuck-programma.
- Een bestand met een invoerstring.

Het tweede argument is optioneel. Als het niet wordt opgegeven, zal de interpreter de invoer lezen van de standaardinvoer. De interpreter moet de uitvoer naar de standaarduitvoer sturen.

Ook de volgende flags moeten ondersteund worden:

- `--help` of `-h`: Print een helpbericht met uitleg over de argumenten.
- `--version` of `-V`: Print de versie van de interpreter.
- `--debug-dump` of `-d`: Print de geoptimaliseerde instructies naar de erroruitvoer (stderr / `eprintln!`), één instructie per regel. Later meer hierover.

Het is verplicht [`clap`](https://crates.io/crates/clap) te gebruiken om de command line interface te maken.

### Voorbeeld

Interpreter zonder invoerbestand:

```bash
$ cd bf
$ cargo run --release -- programs/is_prime.bf
Enter a number below 10000:
97
97 is prime.
```

Interpreter met invoerbestand (bijvoorbeeld `input.txt`):

```bash
$ cargo run --release -- programs/repeat.bf input.txt
!!!!!
```

**Tip:** Voor complexe Brainfuck-programma's is het aangewezen om het programma met `--release` uit te voeren. Dit kan een groot verschil maken in de snelheid van jouw programma.

### Crates

Zoals eerder vermeld is het gebruik van [`clap`](https://crates.io/crates/clap) verplicht. Er mag ook gebruik worden gemaakt van [deze crates](https://github.com/rust-lang/rust-playground/blob/2c09240768e394437f0bba418ec8a2e6fd0e8d2c/compiler/base/Cargo.toml). Dit zijn de standaard crates ter beschikking op [play.rust-lang.org](https://play.rust-lang.org/). Een paar handige crates zijn `thiserror`, `anyhow` en `regex`. Buiten `clap` is het gebruik van crates optioneel. Ook de standaard bibliotheek (`std`) mag natuurlijk gebruikt worden.

## Optimalisatie

Een naïeve interpreter maakt een 1-op-1 mapping maken tussen Brainfuck-instructies en Rust-instructies. Dit kan er bijvoorbeeld als volgt uitzien:

```rust
pub enum {
    Right,     // >
    Left,      // <
    Add,       // +
    Sub,       // -
    Read,      // ,
    Write,     // .
    StartLoop, // [
    EndLoop,   // ]
}
```

Echter, dit zal een erg inefficiënte implementatie zijn, en je interpreter zal heel traag werken. We kunnen echter een uitgebreidere instructieset maken waarin sommige veelvoorkomende patronen in Brainfuck-programma's samenvoegd worden tot één enkele "uitgebreide" instructie. Dit maakt de interpreter typisch sneller.

Hieronder een lijst van enkele veelvoorkomende patronen die we kunnen samenvoegen tot een enkele instructie, opgedeeld in twee categorieën: deduplicatie en ontlussing. Bedoeling is dat je deze ook in jouw interpreter implementeert. Je bent niet verplicht de optimalisaties in dezelfde volgorde als hier opgelijst te implementeren.

### Deduplicatie

Deduplicatie is het proces waarbij we opeenvolgende instructies van hetzelfde type gaan samenvoegen of elimineren.

- We spreken over **homogene deduplicatie** wanneer we opeenvolgende instructies van hetzelfde type samenvoegen. Bijvoorbeeld, de commando's `++++++` kunnen worden omgezet naar één instructie die de waarde met 6 verhoogt (bv. `AddConst(6)`).
- **Heterogene deduplicatie** is wanneer we instructies van verschillende types samenvoegen die elkaar opheffen. Bijvoorbeeld, de commando's `-++---` kunnen worden omgezet naar één enkele instructie die de waarde met 2 verkleint (bv. `SubConst(2)`). Soms is er zelfs geen enkele instructie meer nodig, zoals in het geval van `++--`.

Deduplicatie kan niet alleen worden toegepast op `+` en `-`, maar ook op `>` en `<`. Bijvoorbeeld, de instructies `>>><<>` kunnen worden omgezet naar één enkele instructie die de geheugenwijzer met 2 verhoogt (bv. `MoveRight(2)`).

### Ontlussing

Een optimalisatietechniek die een veel grotere impact heeft op de snelheid van de interpreter is het ontlussen van lussen. Hierbij gaan we lussen in het programma herkennen en deze omzetten naar een enkele instructie. Hieronder zijn een aantal veelvoorkomende patronen die we kunnen samenvoegen tot een enkele instructie.

- De **zoeklus** is exact `[>]` of `[<]`. Ze zal de cursor verplaatsen naar de eerste lege cel, rechts of links. Deze lussen kunnen worden vervangen door een enkele instructie die de cursor verplaatst naar de eerste lege cel, bv. `SearchForwards` of `SearchBackwards`.
- De **resetlus** is exact `[-]`. Ze reset altijd de waarde van de huidige cel naar 0. We kunnen dit altijd vervangen door een enkele instructie die de waarde van de huidige cel op 0 zet, bv. `SetConst(0)`.
- De **kopieerlus** heeft de vorm `[-<<+>>]`. Dit betekent dat de waarde van de huidige cel wordt verplaatst naar de cel 2 plaatsen naar links. Deze hele lus kan worden vervangen door een of meerdere instructies, bv. `Add { offset: -2 }` en `SetConst(0)`.
- De **meervoudige kopieerlus** heeft de vorm `[-<<+<+>>>]`. Dit betekent dat de waarde van de huidige cel wordt verplaatst naar de cellen 2 en 3 plaatsen naar links. Deze hele lus kan worden vervangen door twee instructies, bv. `Add { offset: -2 }` en `Add { offset: -3 }`, en `SetConst(0)`.

Hoewel het zoeken naar en vinden van resetlussen en zoeklussen relatief eenvoudig is, is het vinden van kopieerlus en meervoudige kopieerlus een stuk moeilijker, omdat ze er steeds net iets anders uitzien. In het algemeen spreken we over een kopieerlus als de lus enkel `+`, `-`, `>` en `<` instructies bevat, we even ver naar links als rechts verplaatsen, en we cel op de startpositie met exact 1 verlagen.

De meervoudige kopieerlus is de meest impactvolle optimalisatie, als we een Brainfuck programma sneller willen uitvoeren, maar ze is ook de moeilijkste om te implementeren.

## Debug-uitvoer

Om de instructies te kunnen bekijken, moet de interpreter ook een `--debug-dump` of `-d` vlag ondersteunen. Dit schrijf een overzicht van alle instructies naar de erroruitvoer (stderr), één instructie per regel. Dit kan helpen bij het debuggen van de interpreter, en het begrijpen van de optimalisaties die zijn toegepast. Dit kan er als volgt uitzien:

```bash
$ cargo run --release -- -d programs/helloworld.bf
AddConst(48)
MoveRight(1)
AddConst(33)
MoveRight(1)
Read
MoveLeft(1)
Sub { offset: 1 }
Sub { offset: 0 }
MoveRight(1)
StartLoop
SubConst(1)
MoveLeft(2)
Write
MoveRight(2)
EndLoop
Total number of instructions: 15
---------
Hello, World!
```

Je bent volledig vrij in te kiezen welke instructies er nodig zijn om een Brainfuck programma uit te voeren, en hoe deze eruit zien. Het is dus mogelijk (en waarschijnlijk) dat jouw debug-uitvoer er heel anders uitziet dan de bovenstaande. Print wel steeds één leesbare instructie per regel.

Er mag ook extra informatie in de debug-uitvoer staan, zoals het aantal instructies dat het programma bevat. Zorgt wel dat deze informatie het niet moeilijker maakt om de instructies te lezen.

## Indienen

Plaats de volgende elementen in de root van de repository:

- `/bf/` is een folder met een complete Brainfuck interpreter in Rust. 
- `/REPORT.md` bevat een korte uitleg over de implementatie en de gemaakte keuzes. Indien bepaalde onderdelen niet volledig geïmplementeerd zijn, leg uit waarom. Leg uit welke moeilijkheden u bent tegengekomen en of en hoe u deze hebt opgelost. Geef ook aan hoe lang er aan de opgave en de verschillende delen van de opgave gewerkt is.

**Voeg ook uw naam, voornaam en studentennummer toe aan `REPORT.md`! Anders kan de opgave niet worden geëvalueerd.**

Zorg ervoor dat het programma uitvoerbaar is met:

```bash
cargo run --release -- --help
```

Het is toegestaan om andere CLI-flags toe te voegen, en optimisaties te implementeren die niet in deze opgave staan. Zorgt wel dat de functionaliteit die in deze opgave gevraagd wordt, ook werkt.


Belangrijk: test, eens je je finale commit gemaakt hebt dat je geen bestanden vergeten bent in je repository op te nemen door op een nieuwe locatie je repository te clonen en bovenstaand commando uit te voeren. Compileert en runt alles zoals verwacht?


## Evaluatie

Je wordt verwacht deze opgave in groepen van twee te maken. Hierbij is het gebruik van generative AI om (delen van) de code te schrijven niet toegelaten. 

Deze opgave wordt beoordeeld op **correctheid**, **codekwaliteit**, en een **mondelinge toelichting**. We starten met 0 punten, en er kunnen maximaal 10 punten verdiend worden:
- Correctheid draagt bij tot de punten.
- Een gebrek aan codekwaliteit kost punten. Je kan nooit meer dan 3 punten verliezen op codekwaliteit.
- In de mondelinge toelichting moet je kort de werking van je eigen code toelichten waarbij er vanuit het onderwijsteam ook vragen gesteld worden. Je toont hierdoor aan dat je de ingediende code zelf geschreven hebt en dus beheerst. Indien je dit niet overtuigend kan doen, en er hierdoor dus twijfel ontstaat of je de code zelf geschreven hebt, wordt dit als een vorm van plagiaat beschouwd en conform het examenreglement aan de examencommissie gerapporteerd. De examencommissie beslist over de verdere afhandeling en sancties. Je kan maximaal 3 punten verliezen op de mondelinge toelichting.
- Er kunnen daarnaast ook vragen over Rust zelf gesteld worden tijdens de mondeling toelichting. Deze vragen zijn bedoeld om te controleren of je de features van Rust die je in je project hebt moeten toepassen (de taal zelf dus) ook beheerst, en niet alleen de code die je geschreven hebt. Je kan maximaal 3 punten verliezen op deze vragen.
  - Indien je de taal helemaal niet beheerst en deze vragen niet kan beantwoorden krijg je 3 minpunten.
  - Indien je de taal niet goed beheerst en deze vragen slechts gedeeltelijk kan beantwoorden krijg je 1.5 minpunten.
  - Indien je de taal goed beheerst en deze vragen goed kan beantwoorden krijg je geen minpunten.

### Correctheid (max. 10 punten)

De implementatie moet correct werken zoals beschreven in de opgave. Een inzending die niet compileert is per definitie incorrect en geeft een score van 0/10.

- Basiswerking: 4 punten te verdienen, onderverdeeld als volgt:
  - **+1 punt**: Een CLI met `clap` die de juiste argumenten accepteert.
  - **+1 punt**: De interpreter werkt correct voor brainfuckprogramma's die geen `,` commando bevatten en dus zelf geen invoer lezen.
  - **+0,5 punten**: De interpreter werkt correct voor alle programma's; invoer nodig voor `,` commando's wordt uit een bestand lezen.
  - **+0,5 punten**: De interpreter werkt correct voor alle programma's; invoer nodig voor `,` commando's wordt van de standaardinvoer gelezen.
  - **+1 punt**: De interpreter print een foutmelding uit wanneer het programma een `[` of `]` tegenkomt zonder dat er een bijbehorende `]` of `[` is.
- Optimalisatie: 6 punten te verdienen, onderverdeeld als volgt:
  - **+0,5 punten**: Homogene deduplicatie van `+` en `-` instructies.
  - **+0,5 punt**: Heterogene deduplicatie van `+` en `-` instructies.
  - **+0,5 punten**: Homogene deduplicatie van `>` en `<` instructies.
  - **+0,5 punten**: Heterogene deduplicatie van `>` en `<` instructies.
  - **+1 punt**: Ontlussing van resetlussen `[-]`.
  - **+1 punt**: Ontlussing van kopieerlussen (bv. `[-<<+>>]`).
  - **+1 punt**: Ontlussing van meervoudige kopieerlussen (bv. `[-<<+<+>>>]`).
  - **+1 punt**: Ontlussing van de zoeklussen `[>]` en `[<]`.

**Let op:** Het uitprinten van de debug-uitvoer is een vereiste om punten te krijgen voor optimalisaties. Vergeet dit dus zeker niet!

### Codekwaliteit (max. 3 minpunten)

Om een maximaal aantal punten te behalen, moet de code goed gestructureerd zijn, en gebruik maken van goede programmeerpraktijken. Schrijf duidelijke, beknopte, en becommentarieerde code.

| Punten      | Beschrijving                                                                                                                                                             |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0 minpunten | De code is zeer duidelijk, goed gestructureerd, en gedocumenteerd. De belangrijkste functies hebben enkele testcases.                                                    |
| 1 minpunt   | Er zijn kleine problemen met de leesbaarheid van de code, maar met slechts een kleine impact op de leesbaarheid, of er ontbreken eventueel enkele belangrijke testcases. |
| 2 minpunten | De code is moeilijk te lezen en te begrijpen, maar met moeite kan ze nog gevolgd worden; er is een volledig gebrek aan testcases of cargo check of cargo clippy geven een warning.                                     |
| 3 minpunten | De code is onleesbaar, onbegrijpelijk en bevat geen testcases.                                                                                                           |
