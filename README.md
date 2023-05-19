# git-stax

Commands for helping with stacked branches.

## Branch Naming Convention

There doesn't appear to be many straightforward ways of stacking branches.

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