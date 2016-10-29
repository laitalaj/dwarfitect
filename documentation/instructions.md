# Instructions
Here's how to create (hopefully-)fantastic layouts with dwarfitect. Dig right in!

## 1. Creating an input file
The input file is a JSON file that specifies what kind of rooms do you desire,
how many of each room type you need and with what priority should the rooms be
close to each other. So open up your favourite text editor and get ready to 
specify your desired layout.

### Specifying rooms
Start with an opening curly bracket, a newline and the text `"rooms": [`. This
tells the program that you're about to start specifying a list of desired room
types. The start of your file should now look something like this:
```json
{
"rooms": [
```

Now we'll start typing in room info. For each desired room type write a line
like this:
```json
{"key": 0, "width": 9, "height": 8, "amount": 1}
```
Here the number after `"key":` specifies a unique key for the room type. Make sure
that no two room types have the same key! The number after `"width":` specifies 
the width and the number after `"height":` the height of the rooms of this type,
including walls (so floor space is (height-2)*(width-2) squares). The number
after `"amount":` tells the program how many rooms of this type you want.

At the end of each room type line except for the last one add a comma and a 
newline.

#### Room key 0
The key 0 is a special room; **there must be exactly one room of key 0!**. So
your room specification **MUST** have one line with `"key": 0` and `"amount": 0`!
This is because the layout is "anchored" on this room.

When you feel like you have enough rooms specified, end the listing of room types
with a newline, a square bracket and a comma. The listing should look something
like this:
```json
"rooms": [
    {"key": 0, "width": 9, "height": 9, "amount": 1},
    {"key": 1, "width": 7, "height": 7, "amount": 8},
    {"key": 2, "width": 3, "height": 4, "amount": 16},
    {"key": 3, "width": 6, "height": 6,  "amount": 2}
    ],
```

### Specifying targets
Now that you have desired rooms listed out, let's start listing targets. Targets
are priorities for rooms to be close to each other; they are used to calculate
the fitness of a solution. Start listing targets by goint to a new line and
typing in `"targets": [`. This tells the program again that you are starting a
list of targets.

Now you can start typing out target info with lines like this:
```json
{"from_key": 0, "to_key": 1, "weight": 3.0}
```
The target specifies that from which room type (`"from_key":`) to which room type
(`"to_key":`) you wish to minimize the distance and how important it is to minimize
that distance (`"weight":`). After `"from_key":` write in a key that corresponds
to some room type key that you specified before, and after `"to_key":` write in
another room type key. After `"weight":` type in a floating point number seperated
by a period; if you don't know what to use, go for `"weight": 1.0`. Again, add
a comma after each target that's not the last one.

#### Details on specifying targets
If you don't specify a target for some room key pair, the program won't care
about the distance between those rooms. You only need to specify a target in one
direction; if you already have `"from_key": 2, "to_key": 4`, you don't need to
add `"from_key": 4, "to_key": 2`. If you don't specify any targets, the program
will instead use the total area occupied by the rooms as it's measure of fitness.

When you are done listing targets, close the list with a square bracket.
Your list of targets should now look something like this:
```json
"targets": [
    {"from_key": 0, "to_key": 1, "weight": 3.0},
    {"from_key": 0, "to_key": 3, "weight": 4.0},
    {"from_key": 1, "to_key": 2, "weight": 0.5},
    {"from_key": 2, "to_key": 3, "weight": 0.2},
    {"from_key": 2, "to_key": 0, "weight": 0.05}
    ]
```

### Finalizing input file
Now that you've listed both room types and targets, it's time to wrap the input
file up. Add a new line and a closing curly bracket to tell the program that the
JSON object is done. Now your specification should look something like this:
```json
{
"rooms": [
    {"key": 0, "width": 9, "height": 9, "amount": 1},
    {"key": 1, "width": 7, "height": 7, "amount": 8},
    {"key": 2, "width": 3, "height": 4, "amount": 16},
    {"key": 3, "width": 6, "height": 6,  "amount": 2}
    ],
"targets": [
    {"from_key": 0, "to_key": 1, "weight": 3.0},
    {"from_key": 0, "to_key": 3, "weight": 4.0},
    {"from_key": 1, "to_key": 2, "weight": 0.5},
    {"from_key": 2, "to_key": 3, "weight": 0.2},
    {"from_key": 2, "to_key": 0, "weight": 0.05}
    ]
}
```
Note how theres exactly 1 room with key 0 and how there's a comma after every
variable and line in a list except the last and after the square bracket closing
the listing of rooms.

And that's it, now you should have yourself a working input file! Save it as
`your_filename_here.json` in the folder where the dwarfitect binary is located, 
and move on to generating a layout!

## 2. Making the magic happen
Now that you have an input file resting in the folder of the dwarfitect binary,
dwarfimain.exe (or just dwarfimain in unix), you are ready to generate a layout
from it. Start up dwarfimain, and get to it!

### Specifying input file
First, the program will ask you for input file name. Just type in the file name
of your input file and press enter. Remember the file extension!

### Specifying output file name
Type in any filename for a output file and press enter. 
The finished layout will be saved in the
file you give here. In Windows, you should use `.txt`-extension for your output
file.

**WARNING: If a file with the filename of the output file already exists in the
dwarfitect directory, it will be overwritten!**

### Specifying population size
The size of the population determines how many different optional solutions
the program will have each generation. Bigger population roughly means more optimal
solution in less generations, but slower computation. Just type in a number and
press enter. Suggested value: 500

### Specifying the amount of generations
This will tell the program for how long it should look for an optimal solution.
The more generations, the more optmal the solution will be in the end but it will
take longer to get the solution. Just type in a number and press enter, and the 
program will run. Suggested value: 1000

### Watch it go!
Dwarfitect will keep you up-to-date on the number of passed generations and the
current top fitness. When the specified amount of generations have been calculated,
the optimal solution will be saved in the output file you specified earlier.