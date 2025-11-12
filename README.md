# Learning The Rust Programming Language

Author: Caleb Standfield
\
Date: 12/14/24
\
IDE: RustRover

Why am I learning Rust? I have a couple of reasons:
 - I am developing an app with a team of three other people. Most simply put, it is a general purpose fitness and health tracker app. The backend will be written in rust, and I am our primary backend man.
 - The University of Utah's robotics club uses Rust for robotic programming; thus I need to learn it, so I can contribute.
 - I plan on trying to make an Entity-Component-System (ECS) on my own for a personal project. 
 - I want to see if making a game is possible in Rust.

Learning from the book, "The Rust Programming Language."

I'm using a singular workspace, a folder with all my cargo crates inside it.
There is a singular:
 - Cargo.lock file 
 - .gitignore
 - README.MD

Then each crate and the folder has a:
 - Cargo.toml 

However, RustRover hates me and this folder setup, which is simply a folder containing many cargo crates, and will not allow me to push and pull from RustRover.
I recommend using GitHub Desktop to push and pull this repository. After reading more about workspaces in chapter 14.3, I realize I set this up as a cargo workspace. 

A note from me after getting to chapter 13:

I want to document everything I am learning, I really do, I think it's a great practice and a worthwhile task. 
However, due to time constraints, I have to make the hard decision to forgo some documentation to prioritize my teams' needs.
So some subsections of the Learning Rust Book will go undocumented here.
I will, of course, go over every section thoroughly. This isn't about slacking on my learning, but I'm to the point in my CS career where a large portion of what I'm "learning" is closer to review, rather than new material.
So for the subchapters I deem necessary or worthy of writing down are the only files I will include in this repository from now on. 

A note after chapter 15:

    struct TreeNode {
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>,
        pub parent: RefCell<Option<Weak<TreeNode>>>, 

    }

I want to cry just a little. Rust is definitely more confusing than other languages, that's for sure. But it makes a lot of sense. 
Following the logic and getting the typing right for data structures like this will be hard for the first couple times. 
Also, I'm basically using this README as a small journal now... Hope das chill. 

A note after chapters 18 through 20

These chapters, while important, didn't really need to be written out, especially chapter 20, which covered lots of advanced features that by their own words wouldn't be used often.
All of it was good to know and read, keeping the definitions of what things are and how they work is what I considered to the importance of this chapter.

Hello

Chapter 21 is over (appendix too), and I'm ready to move on from this book and get started creating something new. 
Kinda a sad moment, I really enjoyed this book, perhaps I should have done the educational one with interactive quizzes, but I forgot about that... oops. 
Continuing on, I've got a couple projects I am doing in rust, and so I'll look back to this repository/book for help when I'm stuck. 

I'm coming back to this because I actually forgot to share my thoughts on rust after "learning it." I'm kinda torn between c++ and rust. 

If you were the best programmer in the world, would you choose rust or c++? 

I think in that case c++ might be better. Is there really anything rust can do that c++ can't?
Because of LLVM (rust gets compiled to essentially the same code as c++), I struggle to see why rust would be better. However, since we are only human, we will make mistakes.
I think you can make an argument that you could create more with rust in the same time frame as c++ because you would catch your mistakes at compile time instead of run time.
You could also potentially write code that is more readable and maintainable in rust. However, right now there are significantly fewer libraries in rust than in c++.
So companies might have to make the choice of developing their applications c++ with the chance of having possible runtime errors but save money by using preexisting libraries.
Or develop their own libraries in rust, but then still also have to develop the application which may or may not be more costly.

Perhaps for now there is still a place for c++ and rust in the world.

I really would love to chat with someone who has lots of experience in rust and c++ in a professional setting, so that I could see what they think about this topic.
Maybe someday I'll have that opportunity. Fingers crossed :) 
When I finally get that opportunity, I'll update this journal. <3

panic!("Goodbye");



