/// scaffold-only checklist for a tiny contribution drill.

pub fn contribution_drill_steps() -> Vec<&'static str> {
    vec![
        "read the issue and linked docs",
        "reproduce the behavior locally",
        "write or identify the smallest failing test/doc gap",
        "ask one focused maintainer question if needed",
        "submit a tiny reviewable patch",
    ]
}
