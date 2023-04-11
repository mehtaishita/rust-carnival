# A Carnival of Rust Code Challenges
*Borrowed from LinkedIn Learning by Tim McNamara*

Calculate the median: Key learning concepts
- Can't sort a list of floating point numbers so provide a higher order sort by function that uses partial compare. This will work for finite values. 
- For an even list, taking the average of two numbers requires middle_index + middle_index-1 (not +1);