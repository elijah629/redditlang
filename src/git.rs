use git2::{Repository, ResetType, Direction};
use std::{fs, path::Path};

use crate::utils::Result;

pub fn fetch(repo: &Repository, remote: &str, branch: &str) -> Result<()> {
    Ok(repo.find_remote(remote)?.fetch(&[branch], None, None)?)
}

pub fn hard_reset(repo: &Repository, remote: &str, branch: &str) -> Result<()> {
    let remote_branch_ref = repo.refname_to_id(&format!("refs/remotes/{}/{}", remote, branch))?;
    let object = repo.find_object(remote_branch_ref, None)?;
    repo.reset(&object, ResetType::Hard, None)?;
    Ok(())
}

fn pull(repo: &Repository, remote: &str, branch: &str) -> Result<()> {
    fetch(&repo, remote, branch)?;
    hard_reset(&repo, remote, branch)?;
    Ok(())
}

pub fn clone_else_pull<P: AsRef<Path>>(
    url: &str,
    into: P,
    branch: &str,
) -> Result<()> {
    let cloned = into.as_ref().try_exists()?;
    if cloned {
        let repo = Repository::open(&into)?;
        pull(&repo, "origin", &branch)?;
    } else {
        Repository::clone(&url, &into)?;
    }
    Ok(())
}

/// Update a repository to the latest commit, if it is not cloned, do so. if it is cloned, compare
/// remote and local hashes to see if it is up to date, if it is not, pull. Returns a value that
/// indicates if the repository is up to date.
pub fn update<P: AsRef<Path>>(
    url: &str,
    into: P,
    branch: &str
) -> Result<bool> {
    let cloned = into.as_ref().try_exists()?;
    if cloned {
        let repo = Repository::open(&into)?;
        
        let local = repo.head()?;
        let local_hash = local.peel_to_commit()?.id();

        let mut remote = repo.find_remote("origin")?;
        let connection = remote.connect_auth(Direction::Fetch, None, None)?;

        let remote_hash = connection.list()?[0].oid();

        if local_hash != remote_hash {
            pull(&repo, "origin", &branch)?;
        }
        Ok(local_hash == remote_hash)
    } else {
        Repository::clone(&url, &into)?;
        Ok(false)
    }
}

pub fn checkout(repo: &Repository, refname: &str) -> Result<()> {
    repo.set_head(&refname)?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    Ok(())
}

/// Makes a new repository with the content of the repository at `url`
pub fn generate<P: AsRef<Path>>(
    url: &str,
    branch: Option<&str>,
    into: P,
) -> Result<()> {
    let repo = Repository::clone(&url, &into)?;

    if let Some(branch) = branch {
        checkout(&repo, branch)?;
    }

    fs::remove_dir_all(&into.as_ref().join(".git"))?;
    Repository::init(&into)?;
    Ok(())
}
