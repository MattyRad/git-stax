> I would recommend [git-machete](https://github.com/VirtusLab/git-machete) for stacking operations, it accomplishes most of the goals of this repo. It also has an excellent [IDE Plugin](https://github.com/VirtusLab/git-machete-intellij-plugin#git-machete-intellij-plugin).

# git-stax

Commands for helping with stacked branches.

## Problem

Branching multiple times solves the problem of [staying unblocked](https://graphite.dev/blog/stacked-changes). However, tracking and maintaining stacked branches is a huge pain.

> The [new](https://adamj.eu/tech/2022/10/15/how-to-rebase-stacked-git-branches/) `--update-refs` is helpful, but doesn't fix all problems.

## Goals

- Automate the naming of branches.
- Traverse between parent/child branches easily.
- Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- Easily sync up and/or rebase after code is merged in remotes.

See [comparison](/comparison.md).

## Branch Naming Convention

We can skip most of the headaches that come with DAG traversal by using a branch naming convention:

Have your base or "feature" branch, and then append a hypen for every child branch:

```sh
trunk # or master or main
new/feature
new/feature-anx
new/feature-anx-mie
new/feature-dfg
```

## Commands

### new

Branches off of the current branch, appending a random string.

```zsh
[repo] git-stax new                                                        feature/beta
[repo] git-stax new                                                    feature/beta-08W
[repo]                                                             feature/beta-08W-9j0
```

TODO: maybe some options about what the appended string looks like

### prev

Switches to parent branch.

```zsh
[repo] git-stax prev                                                   temp/one-08W-9jO
[repo]                                                                     temp/one-08W
```

### next

Switches to child branch.

Since a branch can have multiple children, if there is more than one child, it will do an interactive prompt to select which child you want.

```zsh
[repo] git-stax next                                                       temp/one-08W
[repo]                                                                 temp/one-08W-9jO
```

### snip

For when MRs get merged.

- Updates the parent branch with the remote
- Checks if the current branch's HEAD is present the parent branch. If so, offer to delete the current branch.

### complete

TODO

Same as `new`, then

- Pushes to remote
- Creates a GitLab MR
	- Targets parent branch
	- Sets the MR Title to the last commit, stripping out the type/scope from conventional commits

- [1] Yes, we could switch to email patches, but let's be realistic.