# Learning The Rust Programming Language

Author: Caleb Standfield
\
Date: 12/14/24
\
IDE: RustRover

Why am I learning Rust? I have a couple of reasons:
 - I am developing an app with a team of 3 other people. Most simply put, it is a general purpose fitness and health tracker app. The backend will be written in rust and I am our primary backend man.
 - The University of Utah's robotics club uses Rust in the robots programming, thus I need to learn it, so I can contribute.
 - I plan on trying to make a Entity-Component-System (ECS) on my own for a personal project. 
 - I want to see if making a game is possible in Rust.

Learning from the book, "The Rust Programming Language".

I'm using a singular workspace, a folder with all my cargo creates inside of it.
There is a singular:
 - Cargo.lock file 
 - .gitignore
 - README.MD

Then each create and the folder has a :
 - Cargo.toml 

However, RustRover hates me and this folder setup, which is simply a folder containing many cargo crates, and will not allow me to push and pull from RustRover.
I recommend using GitHub Desktop to push and pull this repository. - After reading more about workspaces in chapter 14.3 I realize I set this up as a cargo workspace. 

A note from me after getting to chapter 13:

I want to document everything I am learning, I really do, I think its great practice and worthwhile. 
However, due to time constraints I have to make the hard decision to forgo some documentation in order to prioritize my teams needs.
So some subsections of the Learning Rust Book will go undocumented here.
I will of course go over every section thoroughly, this isn't about slacking on my learning, but I'm to the point in my CS career where a large portion of what I'm "learning" is closer to review, rather than new material.
So for the subchapters I deem necessary or worthy of writing down are the only files I will include in this repository from now on. 

A note after chapter 15:

    struct TreeNode {
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>,
        pub parent: RefCell<Option<Weak<TreeNode>>>, 

    }

I wanna cry just a little, Definitely more confusing than other languages that's for sure. But it makes a lot of sense. 
Following the logic and getting the typing right for data structures like this will be hard for the first couple times. 
Also, I'm basically using this README as a small journal now... Hope das chill. 

A note after chapters 18-20

These chapters while important, didn't really need to be written out, especially chapter 20; which covered lots of advanced features that by their own words wouldn't be used often.
All of it was good to know and read, keeping the definitions of what things are and how they work is what I considered to the important of this chapter. 



