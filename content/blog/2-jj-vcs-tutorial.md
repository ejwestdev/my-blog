---
title = "Quick start guide for Jujutsu Version Control"
date = "September 1st 2026"
---

Once you spend a little time learning jj you will never want to go back to git again. Every brain cycle we spend on version control is wasted and would be better spent on actual programming problems. jj is a great way to offload a lot of that energy, and spend it programming instead. Make sure to check out [the official tutorial and docs for more info](https://docs.jj-vcs.dev/latest/tutorial/), this will just be an overview.

First, run `jj git init` it will create a colocated `.jj` folder in your repo. jj will handle git co-operability. This has the great advantage that while you work using jj, your teammates don't have to even know. It [is possible](https://docs.jj-vcs.dev/latest/FAQ/#should-i-colocate-my-repository) to run jj and git commands at once, but it can be pretty confusing and not worth the hassle. Once you go into jj, it's best to _commit_ ;) all the way.

In jj we work with revisions. A revision is similar to a git commit, except you do not have to manually add to it when you're done working and move on to the next one. Instead, changes are automatically tracked, and you simply create a new revision when you want to make a new set of changes.

To describe the contents of the work in a revision you use `jj describe -m`. Note you can do this at any time hassle free. You can edit a different revision then come back to this one and change the message, or you can write the message at the start of your work, and maybe edit it before pushing. It's great.

To create a new revision you use `jj new`.

To see the status of your current revision and it's parent, you can use `jj st`. To see a longer log (similar to `git log -s`) you use `jj log`. These logs will have an `@` symbol next to the revision you're on, and they will show a line denoting which is the parent and which is the child for each revision and where they branch. To go back to an old revision you simply do `jj edit <REVSETS>`. Note that your changes are automatically saved on the previous revision, so there's no need to worry about `git stash` etc. You can also abandon a revision with `jj abandon`

To push to your remote repo we use Bookmarks. A bookmark is similar to a branch in git. If I want to push my current revision to the main branch, I simply type `jj bookmark set main` followed by `jj git push`. If I want to target a specific revision I would do `jj bookmark set main -r <REVSETS>`. Note for you only have to type the unique characters for a revset, which is often just one or two. The log and status commands will highlight the amount of characters you actually have to type.

If I want to then push my bookmark to the git branch, I simply type `jj git push`. This will push my bookmarks to their corresponding branches. If you have multiple remotes, like an origin and an upstream, you can use `jj bookmark track`. For example: `jj bookmark track main --remote=upstream` or `jj bookmark track dev --remote=origin`.

To pull changes you just do `jj git fetch`. Seeing changes is pretty easy as well. You can use `jj diff` to see changes, or `jj diff -r <REVSETS>` to see changes against a specific ~~branch~~ revision.

I won't go into detail on merge conflicts, but they are pretty straightforward as well. [Have a look at the docs for a great explanation](https://docs.jj-vcs.dev/latest/tutorial/#conflicts). This is one of the most powerful aspects of jj. Seriously, it will change your life.

There is more to it than that, but just these couple of commands handle almost everything you need, they take about an hour to get used to, and they are very easy to work with. I recommend you give it a shot for yourself, and you will quickly see how pain free it is. Of course I suggest reading the official docs as they go into much more detail. Below are all the aliases I made for jj in my terminal, to showcase basically the only commands I ever use. Thanks for reading!

```
alias jn='jj new'
alias je='jj edit'
alias ja='jj abandon'
alias jdm='jj describe -m'
alias jdiff='jj diff'
alias diffNames='jj diff --name-only'
alias jb='jj bookmark set'
```
