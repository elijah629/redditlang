use git2::{Repository, ResetType};
use std::{fs, path::Path};

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
    let cloned = Path::try_exists(&into.as_ref())?;
    if cloned {
        let repo = Repository::open(&into)?;
        pull(&repo, "origin", &branch)?;
    } else {
        Repository::clone(&url, &into)?;
    };
    Ok(())
}

pub fn checkout(repo: &Repository, object: &str) {
    let (object, reference) = repo.revparse_ext(object).expect("Object not found");

    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");
}

pub fn generate<P: AsRef<Path>>(
    url: &str,
    branch: Option<&str>,
    into: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::clone(&url, &into)?;

    if let Some(branch) = branch {
        checkout(&repo, branch);
    }

    fs::remove_dir_all(&into.as_ref().join(".git"))?;
    Repository::init(&into)?;
    Ok(())
}
