# Intoduction

This is a native and web gui for interacting with the PGP protocall. When I eventually finnish this project, anyone will be able to use this in place of the gpg commandline tool at the very least.

# Testing Guidelines

You should have a working knowledge of rust and PGP protocall, the ability to communicate technically and technical skills in whatever operating system you are using (if you are testing on windows, you will have to figure out how to build it your self, I just could not figure out how to build it on windows). As for instructions, try to break it in any way possible, short of feeding it invalid data. Use it, and give me feedback on design (I sI have very little UI/ UX expirence). Give me recommendations in features to add. If you want to contribute, you should know rust at an intermediate level, as well as egui, and (optionally) wasm. If you find bugs you can report them via the github issue tracker or dm me on discord at `inyourface3445`.

# Current and planned features 

- [x] Certifacte import
  - [x] Puiblic keys
  - [x] Private Keys
- [x] New certifcate
- [ ] Sign
- [ ] Verify
- [ ] Encrypt
- [ ] Decrypt

# Installation 

There is no packeges on any repos, and if there are, they are not made or endorsed by me (exept for this one). As of right now, you should clone down the repo, build it and drop the executable wherever you want it. A proper instalation guid will be made when the project is past its alpha phase. 

# Windows and MacOS native builds

Right now I have no plans for a MacOS, as i do not own a Mac nor do I plan on paying there 100 USD per year. Windows builds are a possibilty, once i can build for it. 

# Misc

I recently (7-11-26) started signing all commits, and decided to retoactivly singn all commits in the project. Thge way i did this made it look like all of the commits haoppened on the same day, which they did not. All of the content and commit messages are intact however.