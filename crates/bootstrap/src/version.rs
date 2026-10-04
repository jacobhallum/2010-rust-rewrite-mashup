//! One line naming the build that produced this binary.
//!
//! `build.rs` bakes the facts in at compile time and every field is optional:
//! a build from a tarball with no `git` leaves them empty. This formats
//! whatever survived into something a log reader can act on.

/// The build line for this binary, for logging at startup.
pub fn build_line() -> String {
    line_from(
        env!("IW4L_BUILD_GIT_DESCRIBE"),
        env!("IW4L_BUILD_GIT_SHA"),
        env!("IW4L_BUILD_GIT_DIRTY"),
        env!("IW4L_BUILD_PROFILE_KIND"),
        env!("IW4L_BUILD_RUSTC"),
    )
}

fn line_from(describe: &str, sha: &str, dirty: &str, profile: &str, rustc: &str) -> String {
    let version = if !describe.is_empty() {
        describe
    } else if !sha.is_empty() {
        // A short sha is enough to find the commit, and fits a log line.
        &sha[..sha.len().min(7)]
    } else {
        // `git` could not answer at build time. Say so rather than implying clean.
        "unknown"
    };
    let mut parts = vec!["build", version];
    if !profile.is_empty() {
        parts.push(profile);
    }
    if dirty == "true" {
        parts.push("dirty");
    }
    if !rustc.is_empty() {
        parts.push(rustc);
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::line_from;

    #[test]
    fn names_the_tag_the_profile_and_the_dirty_tree() {
        // A tagged, clean release build: the describe string is the version.
        assert_eq!(
            line_from("jh-0.1", "abc1234def", "false", "release", "rustc 1.99.0"),
            "build jh-0.1 release rustc 1.99.0"
        );
        // Uncommitted changes must be visible; a log from a dirty tree is not
        // reproducible and the reader has to know.
        assert_eq!(
            line_from("jh-0.1-dirty", "abc1234def", "true", "play", "rustc 1.99.0"),
            "build jh-0.1-dirty play dirty rustc 1.99.0"
        );
        // No tag yet: fall back to a short sha rather than saying nothing.
        assert_eq!(
            line_from("", "abc1234def5678", "false", "release", "rustc 1.99.0"),
            "build abc1234 release rustc 1.99.0"
        );
        // git was unavailable at build time, which is not the same as clean.
        assert_eq!(
            line_from("", "", "", "release", "rustc 1.99.0"),
            "build unknown release rustc 1.99.0"
        );
        // Nothing at all survived.
        assert_eq!(line_from("", "", "", "", ""), "build unknown");
    }
}
