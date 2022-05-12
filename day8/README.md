
# Advent of code 2021 - Day 8

 We have a set of seven-segment displays
 The wires that turn on each segment have been messed up
 so now each wire is connected to a random segment.

 We need to configure all the wires to the correct segments again,
 so we need to figure out patterns that help us understand to which segment
 each wire should be connected to.

 This is how each display is represented:

 ```text
    0:      1:      2:      3:      4:
   aaaa    ....    aaaa    aaaa    ....
  b    c  .    c  .    c  .    c  b    c
  b    c  .    c  .    c  .    c  b    c
   ....    ....    dddd    dddd    dddd
  e    f  .    f  e    .  .    f  .    f
  e    f  .    f  e    .  .    f  .    f
   gggg    ....    gggg    gggg    ....

    5:      6:      7:      8:      9:
   aaaa    aaaa    aaaa    aaaa    aaaa
  b    .  b    .  .    c  b    c  b    c
  b    .  b    .  .    c  b    c  b    c
   dddd    dddd    ....    dddd    dddd
  .    f  e    f  .    f  e    f  .    f
  .    f  e    f  .    f  e    f  .    f
   gggg    gggg    ....    gggg    gggg
 ```

 Some hints that can help:

 The number 1 is the only number that requires 2 wires,
 so whenever we have only 2 wires on, we know these 2 wires should
 should be connected to the segments c & f

 The number 8 is the only number that requires 3 wires.

 So to summarize, we have 4 numbers with a unique combination of wires that can help us deduce the rest:

 1 (2 wires), 7 (3 wires), 4 (4 wires), 8 (7 wires)

## Day 1

 Given an input with the following format:

```text
fbcag eg dbge gcbfe acdgfe gec fdcegba efbdc fbedgc efabdc | ge fagbc dfebc eg
|--------------------------------------------------------|   |---------------|
              All the 10 unique combinations                   Output number
```

 Count how much 1, 7, 4, and 8 digits are in the output number
