# PatchKeeper

PatchKeeper (pk) is an alternative [Git](https://git-scm.com/) command-line interface, with
first-class support for managing patches.

If you like the workflow provided by tools like [Mercurial Queues](https://www.mercurial-scm.org/wiki/MqExtension)
(mq) and are looking for something similar but for *git*, then PatchKeeper might be for you.

PatchKeeper is just a front-end for various (sometimes semi-complicated sequence of) git commands.
It operates on normal git repositories, and the managed patches are normal git commits. It can therefore
be used either as a replacement of, or as a compliment to, the **git(1)** command.
