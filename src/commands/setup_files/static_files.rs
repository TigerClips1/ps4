pub fn default_config() -> &'static str {
    r#"{
        "architecture": "x86_64",
        "codename": "CyberGamerX"
        "version": "Gamer_1_0"
        "toolchain": "knot",
        "colour": true,
        "progressbar": true,
        "repos": [
            {
                "name": "ps4_core",
                "active": true
            },
            {
                "name": "ps4_extra",
                "active": true
            },
            {
                "name": "ps4_multilib",
                "active": true
            },
        ]
    }"#
}

pub fn default_mirrorlist() -> &'static str {
    r#"# ps4 default mirror list
https://github.com/TigerClips1/JaguarLinux-repo-PS4/$codename/$version/$repo/$arch/$toolchain"#
}