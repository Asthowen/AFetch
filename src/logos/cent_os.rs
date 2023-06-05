pub const CENT_OS: [&str; 4] = [
    "",
    "",
    "",
    r#"[38;5;215m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
[38;5;215m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
[38;5;215m⠀⠀⠀⠀⠀⠀       ⠀⠀⠀⠛⠛⠛⠛⢻⡟⠛⠛⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⠿⢋⣴⣿⣿⣿⣿⣿⡇[38;5;215m⢸⡇⠀[38;5;126m⣿⣿⣿⣿⣷⣄⠈⠻⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⣿⣿⣿⢿⣇⠐⢿⣿⣿⣿⣿⣿⣿⡇[38;5;215m⢸⡇⠀[38;5;126m⣿⣿⣿⣿⣿⣿⠗⠀⣨⡿⣿⣿⣿⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⣿⠟⢁⣄⠙⢷⣦⡙⠿⣿⣿⣿⣿⡇[38;5;215m⢸⡇⠀[38;5;126m⣿⣿⣿⣿⠟⠁⣠⡾⠋⣀⡈⠛⢿⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⢁⣴⣿⣿⣷⣄⠙⢿⣦⡈⠻⣿⣿⡇[38;5;215m⢸⡇⠀[38;5;126m⣿⣿⠟⢁⣠⠞⠁⣠⣾⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣷⣄⠙⢿⣦⡈⠻⠇[38;5;215m⢸⡇⠀[38;5;126m⠟⢁⣴⠟⠁⣠⣾⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
[38;5;126m⠀⠀⢀⣴⣿⠀[38;5;149m⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡙⠻⣦⡀[38;5;215m⠸⠇⠀[38;5;126m⣴⠟⠁⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⠀[38;5;24m⢸⣦⡀⠀⠀
[38;5;126m⢀⣴⣿⣿⣿⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀ ⠀⠀⠀⠀⠀[38;5;24m⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣸⣿⣿⣦⡀
[38;5;126m⠈⠻⣿⣿⣿⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀  [38;5;24m⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹⣿⡿⠛⠁
⠀⠀[38;5;126m⠈⠻⣿⠀[38;5;24m⢸⣿⣿⣿⣿⣿⣿⣿⠟⠉⢀⡴⠋⠀[38;5;149m⢰⡆[38;5;215m⠈⠻⣷⣄⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⠀[38;5;24m⠸⠋⠀⠀⠀
⠀⠀⠀⠀⠀⠀[38;5;24m⢸⣿⣿⣿⣿⣿⠟⠁⢀⡴⠋⢀⣴⠀[38;5;149m⢸⡇[38;5;215m⢰⣦⡈⠻⣷⣄⠙⢿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀[38;5;24m⠙⢿⣿⠟⠁⢀⡴⠋⢀⣴⣿⣿⠀[38;5;149m⢸⡇[38;5;215m⢸⣿⣿⣦⣈⠻⢷⣄⠙⢿⣿⣿⠟⢁⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀[38;5;24m⢰⣄⠀⠁⣠⠞⠋⢀⣴⣿⣿⣿⣿⠀[38;5;149m⢸⡇[38;5;215m⢸⣿⣿⣿⣿⣷⣄⠙⢷⣄⠉⣡⣶⣿⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀[38;5;24m⢸⣿⣷⣾⡁⠀⢴⣿⣿⣿⣿⣿⣿⠀[38;5;149m⢸⡇[38;5;215m⢸⣿⣿⣿⣿⣿⣿⡷⠀⣹⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀[38;5;24m⢸⣿⣿⣿⣿⣦⡀⠙⢿⣿⣿⣿⣿⠀[38;5;149m⢸⡇[38;5;215m⢸⣿⣿⣿⣿⡿⠋⣠⣾⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀[38;5;149m⢤⣤⣤⣤⣼⣧⣤⣤⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
[38;5;149m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⡟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#,
];
