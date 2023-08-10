
## [git-stack](https://github.com/gitext-rs/git-stack)

> See also [orignal doc](https://github.com/gitext-rs/git-stack/blob/main/docs/comparison.md)

- [ ] Automate the naming of branches.
- [x] Traverse between parent/child branches easily.
	- `next` and `prev`.
- [x] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
	- Looks like a mostly local workflow.
- [ ] Easily sync up and/or rebase after code is merged in remotes.
	- `sync` is manual

## [git-branchless](https://github.com/arxanas/git-branchless)

- [ ] Automate the naming of branches.
- [x] Traverse between parent/child branches easily.
	- `next` and `prev`. It's not always correct though :( it becomes confused in non-ideal situations.
- [x] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [ ] Easily sync up and/or rebase after code is merged in remotes.
	- `sync` is manual

## [git-machete](https://github.com/VirtusLab/git-machete)

- [ ] Automate the naming of branches.
- [x] Traverse between parent/child branches easily.
	- `go down/first/last/next/root/prev/up`. Appears very flexible, very nice.
	- `show down/first/last/next/root/prev/up`.
- [x] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [x] Easily sync up and/or rebase after code is merged in remotes.

## [git-ps-rs](https://github.com/uptech/git-ps-rs)

- [x] Automate the naming of branches.
	- Doesn't use branches, which is good! However there's still manual labor involved in
- [x] Traverse between parent/child branches easily.
- [ ] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [x] Easily sync up and/or rebase after code is merged in remotes.

Strong contender and the philosophy is clear. Dirty local branches are a dealbreaker.

## Jujutsu

- [x] Automate the naming of branches.
	- Doesn't use branches, which is good!
- [x] Traverse between parent/child branches easily.
- [ ] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [?] Easily sync up and/or rebase after code is merged in remotes.

Strong contender, but technically it does not operate on a git repo, and its git compatibility layer is questionable. With a better UX then it would probably work.

## [Stacked Git](https://stacked-git.github.io/)

- [ ] Automate the naming of branches.
- [x] Traverse between parent/child branches easily.
- [ ] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [?] Easily sync up and/or rebase after code is merged in remotes.

## [git-chain](https://github.com/dashed/git-chain)

- [ ] Automate the naming of branches.
- [?] Traverse between parent/child branches easily.
- [?] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [?] Easily sync up and/or rebase after code is merged in remotes.

# Offsite

> I dislike GitHub as much as the next guy, but you can't knock that it assists with discoverability. Repositories that live off of Github have an extra hurdle to adoption.

## [depo-tools](https://commondatastorage.googleapis.com/chrome-infra-docs/flat/depot_tools/docs/html/depot_tools_tutorial.html)

- [ ] Automate the naming of branches.
- [?] Traverse between parent/child branches easily.
- [?] Do not taint local branch state with changes that are potentially unmerged in remotes. Local state should match remote state as best as possible.
- [?] Easily sync up and/or rebase after code is merged in remotes.

## [git-branchstack](https://git.sr.ht/~krobelus/git-branchstack)

Doesn't exactly look like a stacked commit workflow? However, `branchstack-pick` looks useful completely separate from the idea of stacked commits.

## [sapling](https://github.com/facebook/sapling)

TODO

# Abandonded

- [git-series](https://github.com/git-series/git-series)
- [gh-stack](https://github.com/timothyandrew/gh-stack)
- [Graphite](https://github.com/screenplaydev/graphite-cli)

# Coupled to 3rd parties

- Github - [git spr](https://github.com/ejoffe/spr) (looks pretty nice though)
- Github - [ghstack](https://github.com/ezyang/ghstack)
- Phabricator - [arcanist](https://secure.phabricator.com/book/phabricator/article/arcanist/)
