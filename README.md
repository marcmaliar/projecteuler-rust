# Project Euler select Rust problems

I solved some Project Euler problems in Rust. I think it's problem 1, 2, and prime family.

The prime family one was challenging. It works by getting the powerset of a digit size, zeroing out the digits chosen, and adding 1*p, 2*p, 3\*p, etc. With the optimizations I've got (using a cache so you don't have to recompute the same prime family with different elements and other caches) it can run through a lot of numbers on my M1 mac.
