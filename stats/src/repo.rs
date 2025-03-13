use git2::{BranchType, DiffStats, Error, Oid, Repository};
use std::path::PathBuf;

pub struct ForkStats {
    pub commits_count: usize,
    pub insertions: usize,
    pub deletions: usize,
}

fn get_branch_oid(repository: &Repository, name: &str) -> Result<Oid, Error> {
    let branch = repository.find_branch(name, BranchType::Remote)?;
    let oid = branch.get().resolve()?.target().unwrap();

    Ok(oid)
}

fn get_merge_base(repository: &Repository, branch1: Oid, branch2: Oid) -> Result<Oid, Error> {
    let merge_base = repository.merge_base(branch1, branch2)?;

    Ok(merge_base)
}

fn count_commis(repository: &Repository, from: Oid, to: Oid) -> Result<usize, Error> {
    let mut revwalk = repository.revwalk()?;

    revwalk.push(from)?;
    revwalk.hide(to)?;

    let mut count = 0;
    for oid in revwalk {
        let commit = repository.find_commit(oid?)?;
        if commit.parent_count() > 1 {
            continue;
        }
        count += 1;
    }

    Ok(count)
}

fn get_diff_stats(repository: &Repository, from: Oid, to: Oid) -> Result<DiffStats, Error> {
    let new_tree = repository.find_commit(from)?.tree()?;
    let old_tree = repository.find_commit(to)?.tree()?;

    let diff = repository.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;

    Ok(diff.stats()?)
}

fn fetch_all_remotes(repository: &Repository) -> Result<(), Error> {
    for remote_name in repository.remotes()?.iter() {
        let mut remote = repository.find_remote(remote_name.unwrap())?;
        println!("fetching {}", remote.url().unwrap());
        remote.fetch::<&str>(&[], None, None)?;
    }

    Ok(())
}

pub fn get_fork_stats(
    repository_path: &PathBuf,
    upstream_branch: &str,
    fork_branch: &str,
) -> Result<ForkStats, String> {
    fn get_stats(
        repository_path: &PathBuf,
        upstream_branch: &str,
        fork_branch: &str,
    ) -> Result<ForkStats, Error> {
        let repository = Repository::open(repository_path)?;

        fetch_all_remotes(&repository)?;

        let upstream = get_branch_oid(&repository, upstream_branch)?;
        let maxiv = get_branch_oid(&repository, fork_branch)?;

        let merge_base = get_merge_base(&repository, upstream, maxiv)?;
        let diff_stats = get_diff_stats(&repository, maxiv, merge_base)?;

        let fork_stats = ForkStats {
            commits_count: count_commis(&repository, maxiv, merge_base)?,
            insertions: diff_stats.insertions(),
            deletions: diff_stats.deletions(),
        };

        Ok(fork_stats)
    }

    match get_stats(repository_path, upstream_branch, fork_branch) {
        Ok(fork_stats) => Ok(fork_stats),
        Err(err) => Err(err.message().to_string()),
    }
}
