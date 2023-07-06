use git2::{Repository, ResetType};
use std::path::Path;

pub fn fetch(repo: &Repository, remote: &str, branch: &str) -> Result<(), git2::Error> {
    repo.find_remote(remote)?.fetch(&[branch], None, None)
}

pub fn hard_reset(repo: &Repository, remote: &str, branch: &str) -> Result<(), git2::Error> {
    let remote_branch_ref = repo.refname_to_id(&format!("refs/remotes/{}/{}", remote, branch))?;
    let object = repo.find_object(remote_branch_ref, None)?;
    repo.reset(&object, ResetType::Hard, None)?;
    Ok(())
}

fn pull(repo: &Repository, remote: &str, branch: &str) -> Result<(), git2::Error> {
    fetch(&repo, remote, branch)?;
    hard_reset(&repo, remote, branch)?;
    Ok(())
}

pub fn clone_else_pull<P: AsRef<Path>>(
    url: &str,
    into: P,
    branch: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cloned = Path::try_exists(into.as_ref())?;
    if cloned {
        let repo = Repository::open(into)?;
        pull(&repo, "origin", branch)?;
    } else {
        Repository::clone(url, into)?;
    };
    Ok(())
}
