# user_points-rs

This is a meme project from [Snackers HQ](https://discord.gg/epe5NbhJhc) (The discord server for the popular VTuber Filian).
This does __**NOT**__ reflect my actual feelings about anyone.

# History
- it began as a project in __**C**__ named `hate.c`.
  - this only had the basic `user(char* name[])` and `setPoints(user, char* name[], int points)` methods.

- then it soon was rewritten in __**C++**__ for better ergonomics for the API, it was named `hate.cpp`.
  - the __**C++**__ version also brought love points and a new management system.

- then i remade it in __**TypeScript**__ because i needed to rethink the whole API, and __**TypeScript**__ is easier for me to prototype in, it was named `hate.ts`.
  - after i tried further improving the __**C++**__ version i ran into problems with the language so i made the __**TypeScript**__ version.. which added better control over points and the silly point type.

- and now we have the __**Rust**__ version, named `user_points.rs`.
  - this is a direct port of the __**TypeScript**__ version
  - but i have added the friendship points type.
  - and the crush points type.

