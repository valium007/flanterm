fn main() {
    cc::Build::new()
        .flag("/c flanterm/flanterm.c flanterm/backends/fb.c")
        .flag("/std:c11")
        .flag("/GS-")
        .flag("/link /NODEFAULTLIB")
        .compile("flanterm")
}
