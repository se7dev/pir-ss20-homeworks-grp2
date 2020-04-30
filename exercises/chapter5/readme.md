# A small Arena Fighter using Structs
In this tutorial you will program a small arena fighter in Rust mainly using Structs. In the end, your small game should be able to
- create a gamefield in different sizes
- spawn heroes and monsters
- let them move across the gamefield and update the field accordingly
- attack each other
- let the monster duplicate itself once it reaches a certain HP threshold

# Exercise 1
Generate some structs that will hold the information about
-

- your Hero
-  a Monster 
-  and the Gamefield

Your Monster and Hero structs should hold information about
-

- name
- HP
- damage it is able to deal
- position on the gamefield in (x,y) form

Your gamefield should hold information about
-

- the gamefield itself
  - **Hint:** your gamefield should be able to be generated dynamically at runtime, so best use Vectors
- the size of the gamefield

Print the information about your hero and your monster by using **println!** once you instanciated them.

Next, implement basic functions for your heroes and monsters to attack each other.

# Exercise 1.1
Once you are able to do the above, you now should **create** a gamefield and **spawn** your hero and monster using **associated functions**.

# Exercise 2
The gamefield should represent the positions of your Units on the field.

As vectors in Rust do not implement the **copy Trait** you have to find out how to pass the gamefield to a **display function** without breaking things!

Now, you need to make your units move across the gamefield and update it accordingly.

# Exercise 2.1
Your monster needs to have a special ability to make the fight more interesting.

This special ability will be a **cloning ability**. Once the monster reaches a certain HP threshold (e.g. 50 HP), it should automatically clone itself and place a copy of itself with only a different name and slightly waker next itself.

Which ability of structs would be suitable to use here?

# Notes
You will find a skeleton file in the exercises folder which could give you some hints on the structure of the code, but feel free to find your own solutions!