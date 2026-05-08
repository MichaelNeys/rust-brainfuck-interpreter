# Verslag

| Naam   | Voornaam | Studentnummer |
| :----- | :------- | ------------: |
| Lehaen | Joost| 2468995 |
| Neys   | Michael | 2467626 |

# Programa overview
Het hele programma werkt in 4 fases

## 1. Het inlezen van user input via Clap
De Args struct bevat alle mogelijke input parameters voor het programma en derived ClapParser. 
Via `Args::parse()` worden dan de parameters ingelezen.
Aan de hand van args wordt het brainfuck programma ingelezen.

## 2. Het parsen van het brainfuck programma
De parse fase is zeer kort en parsed alles letterlijk per instructie naar Instruction enumerators.

## 3. Het optimaliseren van het brainfuck programma
De optimalisatie fase krijgt een lijst van instructies en past een aantal optimalisaties toe. 
De optimalisaties worden in een bepaalde volgorde toegepast zodat ze voorbereidend werk kunnen doen voor de volgende en elkaar niet in de weg zitten. Bv. Eerst moves en adds mergen voordat we gaan delopen
We doen dit in een loop die stopt vanaf het moment dat de optimalisaties geen effect meer hebben gehad op de lijst van instructies.
Als laatste wordt dit in een InstructionList object gezet. Als dit object aangemaakt word, word ook automatisch een jump table gegenereert voor '\[' en '\]' instructies

## 4. Het uitvoeren van het brainfuck programma
Na het parsen en optimaliseren van het brainfuck programma is uitvoeren straight forward. De executor gebruikt een InstructionList en MemoryTape om de instructies op uit te voeren. 
Op beide datastructuren zijn equivalente functies geimplementeerd als de Instruction enumerators. 
Sommige instructies gebruiken combinaties van functies om hun doel te bereiken bv. `JumpToRight`.

## MemoryTape
Omdat er geen restricties opgelegd zijn voor de pointer op de geheugen lijst, ondersteunt MemoryTape zowel positieve als negatieve indices.
We doen dit door aan 2 Vec objecten te gebruiken, 1 voor negatieve en 1 voor positieve indices. 
Intern worden de pointers vertaald naar hun juiste array en index combinatie.

# Optimalisaties

## Verplicht

### Deduplicatie
We hebben volledige heterogene deduplicatie geimplementeerd inclusief het verwijderen van instructies die geen effect zullen hebben op het uiteindelijke resultaat.
We hebben dit toegepast op de verplichte Move en Add instructies.
De optimizer gebruikt voor beide instructies een fold op de instructions slice. Hierbij worden 2 dezelfde instructies hun count opgeteld bij elkaar.
Bij Add wordt dit enkel gedaan als de offset gelijk is.

### Ontlussing
Alle verplichte vormen van ontlussing zijn geimplementeerd: zoeklus, resetlus en alle vormen van kopieerlussen.

Bij het onlussen van zoeklussen en resetlussen wordt in een fold letterlijk gematched op 3 opeenvolgende instructies.
Als die instructies overeenkomen met een zoeklus of resetlus worden die instructies vervangen door 1 Instructie enumerator.

Bij het ontlussen van Kopieerlussen gaat het iets ingewikkelder. We overlopen elke instructie 1 voor 1. 
Als we een `JumpToRight` instructie tegenkomen zien we dit als een entrypoint naar een mogelijke kopieerlus.
Dan beginnen we met tellen tot we een `JumpToLeft` tegenkomen. We tellen de volgende dingen.
* de totale verplaatsing
* hoeveel de begin cell veranderd wordt
We houden ook een lijst bij van alle posities waar naar gecopieerd moet worden (Dit is bij elke set van + of - die geen totale verplaatsing van 0 hebben)

Als we op het einde een totale verplaatsing van 0 hebben en de begin cell met -1 is veranderd dan vervangen we alle instructies die we tegen zijn gekomen met een `Copy` instructie met de juiste parameters.
We voegen daarna ook nog een Reset toe om de huidige cell te resetten.

## Extra
We hebben ook nog een aantal kleinere optimalisaties toegepast die niet verplicht waren

### ChunkCopies
We hebben gemerkt dat een veel voorkomend patroon in brainfuck het volgende is: Copier een cel met een offset, verplaats 1 en copieer nog eens met dezelfde offset etc.
We hebben dit geoptimaliseerd naar een `ChunkCopy` dat all deze instructies in 1 keer uitvoert. 
Achterliggend gebruikt het nog wel dezelfde logica en functies die normaal zouden aangeroepen worden.
Dit komt doordat elke copy technisch gezien veranderd kan worden door de vorige copy.
Het enige verschil is dat we de meerder moves van 1 kunnen vervangen door 1 collectieve move op het einde. (Deze kan dan weer gededupliceerd worden in een volgende pass.)
Bovendien moeten de instructies ook niet elke keer door de match statement en kan alles met een for-loop uitgevoerd woden.

De optimalisatie is geimplementeerd als volgt.
We gaan over alle instructies heen. 
Als we een copy tegenkomen gaan we in stappen van 3 beginnen springen door de instructies heen.
zo lang Copy-Move-Reset blokken zich blijven herhalen verhogen we de CopyChunk lengte.
Als we bij een Copy-Move blok komen, zonder Reset, is dit de laatste en moeten we nog een extra lengte toevoegen en kunnen we dan een ChunkCopy toevoegen.
Anders Voegen we meteen, bij een lengte groter dan 1, een ChunkCopy toe.

### Move-Add-Move structuren
We hebben ook gemerkt dat er vaak verplaatst wordt een getal wordt opgeteld en dan weer terug verplaatst wordt naar de originele plaats.
Dit is equivalent aan het optellen met een offset.

Als we opeenvolgend een Move een Add en nog een Move met dezelfde count als de vorige tegenkomen vervangen ze deze door een Add met een offset.
De vorige Add kan ook al een offset bevatten. In dat geval worden de offsets opgeteld.

### Print collapse
Aangezien het ons sneller leek om prints meerder keren uit te voeren in een for loop dan elke keer door een match statement te moeten gaan, houden prints een count bij van hoeveel keer ze effectief willen printen.
Dit is een zeer kleine optimalisatie die niet veel effect heeft op het uitvoeren van programmas
