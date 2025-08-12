fn main() {
    cc::Build::new()
        .file("flanterm/flanterm.c")
        .file("flanterm/backends/fb.c")
        .std("c11")
        .flag("/GS-")
        .flag("/link /NODEFAULTLIB")
        .compile("flanterm")
}
