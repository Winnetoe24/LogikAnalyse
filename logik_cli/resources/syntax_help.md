**Die Syntax**

Es gibt drei große Commands:
1. SET
2. PRINT
3. TABELLE

**SET**

Parsed und setzt eine Formel. 

Syntax:`SET <ART> <NAME> <FORMEL>` 

Arten:
1. AUSSAGEN

**PRINT**

Gibt etwas aus. Was ausgegeben wird, hängt vom Subcommand ab.

**Formel**

Gibt eine Formel aus. Kann zum Formatieren genutzt werden. 

Syntax:`PRINT <FORMATIERUNG> <FORMELNAME>` 

Formatierungen:
1. Formel-UTF
2. Formel-ASCII

**Tabelle**

Gibt eine Wahrheitstabelle aus, wenn sie vorher generiert wurde.

Syntax: `PRINT TABELLE`

**Belegung**

Gibt, aus ob eine Belegung true oder false ist. Die angegebenen Variabeln werden auf true gesetzt.

Syntax: `PRINT BELEGUNG <FUNKTIONENNAMEN>... | <VARIABELNAMEN>`

**Äquivalenz**

Gibt aus, ob die angegebenen Funktionen äquivalent sind. 

Syntax: `PRINT AEQUIVALENZ <FUNKTIONENNAMEN>...`

**Tabelle**
Generiert eine Wahrheitstabelle für die angegebenen Funktionen. 

Syntax: `TABELLE <FUNKTIONENNAMEN>...`