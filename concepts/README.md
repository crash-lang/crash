# Crash concepts
**We're currently in a pseudo-project.**

In here is a Crash project, which uses the Crasher package manager.

Nothing in here is tested and just exists to show you the concepts of Crash
and the overall language design.

# Core concepts
**You should know this, before looking at Crash code!**

- Everything is an object, except primitive data-types.
- Normally there are only references of instances. No pointers or copies.
- Enums are sort of carrier-objects.
- There is no null. You have to use a Optional enum.
- There are no Exceptions. So you won't find throw or try-catch statements.
  You would have to use a Result enum.
- Reflection is kind of an actual safe feature. You cannot mess it up.
- Just everything is safe.