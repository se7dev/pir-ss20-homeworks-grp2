# Chapter 5 - Using Structs to Structure Related Data in Rust
In this chapter we will take a look at structs and how to use them.

## Easy Task
- Define two struct's Hero and Monster including fields HP, Damage, Name
- Define two tuple struct's Potions and PoisonFlask with effect on (HP value,+/-)
- Make all four printable
- Create a "new" method to instantiate the struct's
- Use struct update syntax to clone a monster with the clone having less HP

## "Advanced Task"
- Implement methods for healing and attacking for the hero and monster(borrowing)
- Explicitly use borrowing for method receivers
- Use an associated function to summon a monster with less than standard HP. 