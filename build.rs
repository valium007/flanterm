fn main() {
    cc::Build::new()
        .compiler(Path::new("C:\\Users\\Admin\\Downloads\\clang-18.1.8-windows-amd64-msvc17-msvcrt\\clang-18.1.8-windows-amd64-msvc17-msvcrt\\bin\\clang.exe"))
        .file("flanterm/flanterm.c")
        .file("flanterm/backends/fb.c")
        .flag("-std=c11")
        .flag("-ffreestanding")
        .flag("-fno-stack-protector")
        .flag("-fno-stack-check")
        .flag("-mno-red-zone")
        .compile("flanterm")
}
