
extern crate target_triple_parser;
use target_triple_parser::TargetTriple;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strings_in_constructor() {
        let t = TargetTriple::new("".to_string(), "".to_string(), "".to_string(), Some("".to_string()));
        assert_eq!(t.architecture, "");
        assert_eq!(t.vendor, "");
        assert_eq!(t.system, "");
        assert_eq!(t.abi, Some("".to_string()));
    }

    #[test]
    fn simple_parsing() {
        let t = target_triple_parser::parse("a-b-c").unwrap();
        assert_eq!(t.architecture, "a");
        assert_eq!(t.vendor, "b");
        assert_eq!(t.system, "c");
        assert_eq!(t.abi, None);

        let t = target_triple_parser::parse("a-b-c-d").unwrap();
        assert_eq!(t.architecture, "a");
        assert_eq!(t.vendor, "b");
        assert_eq!(t.system, "c");
        assert_eq!(t.abi, Some("d".to_string()));
    }

    fn check_parse_3(target_triple: &str, architecture: &str, vendor: &str, system: &str) {
        let t = target_triple_parser::parse(target_triple).unwrap();
        assert_eq!(t.architecture, architecture);
        assert_eq!(t.vendor, vendor);
        assert_eq!(t.system, system);
        assert_eq!(t.abi, None);
    }

    fn check_parse_4(target_triple: &str, architecture: &str, vendor: &str, system: &str, abi: &str) {
        let t = target_triple_parser::parse(target_triple).unwrap();
        assert_eq!(t.architecture, architecture);
        assert_eq!(t.vendor, vendor);
        assert_eq!(t.system, system);
        assert_eq!(t.abi, Some(abi.to_string()));
    }

    #[test]
    fn parse_real_target_triples() {
        check_parse_3("aarch64-apple-ios", "aarch64", "apple", "ios");
        check_parse_3("aarch64-linux-android", "aarch64", "linux", "android");
        check_parse_3("aarch64-unknown-cloudabi", "aarch64", "unknown", "cloudabi");
        check_parse_3("aarch64-unknown-freebsd", "aarch64", "unknown", "freebsd");
        check_parse_3("aarch64-unknown-fuchsia", "aarch64", "unknown", "fuchsia");
        check_parse_4("aarch64-unknown-linux-gnu", "aarch64", "unknown", "linux", "gnu");
        check_parse_4("aarch64-unknown-linux-musl", "aarch64", "unknown", "linux", "musl");
        check_parse_3("aarch64-unknown-openbsd", "aarch64", "unknown", "openbsd");
        check_parse_3("arm-linux-androideabi", "arm", "linux", "androideabi");
        check_parse_4("arm-unknown-linux-gnueabi", "arm", "unknown", "linux", "gnueabi");
        check_parse_4("arm-unknown-linux-gnueabihf", "arm", "unknown", "linux", "gnueabihf");
        check_parse_4("arm-unknown-linux-musleabi", "arm", "unknown", "linux", "musleabi");
        check_parse_4("arm-unknown-linux-musleabihf", "arm", "unknown", "linux", "musleabihf");
        check_parse_3("armebv7r-none-eabihf", "armebv7r", "none", "eabihf");
        check_parse_4("armv4t-unknown-linux-gnueabi", "armv4t", "unknown", "linux", "gnueabi");
        check_parse_4("armv5te-unknown-linux-gnueabi", "armv5te", "unknown", "linux", "gnueabi");
        check_parse_4("armv5te-unknown-linux-musleabi", "armv5te", "unknown", "linux", "musleabi");
        check_parse_4("armv6-unknown-netbsd-eabihf", "armv6", "unknown", "netbsd", "eabihf");
        check_parse_3("armv7-apple-ios", "armv7", "apple", "ios");
        check_parse_3("armv7-linux-androideabi", "armv7", "linux", "androideabi");
        check_parse_4("armv7-unknown-cloudabi-eabihf", "armv7", "unknown", "cloudabi", "eabihf");
        check_parse_4("armv7-unknown-linux-gnueabihf", "armv7", "unknown", "linux", "gnueabihf");
        check_parse_4("armv7-unknown-linux-musleabihf", "armv7", "unknown", "linux", "musleabihf");
        check_parse_4("armv7-unknown-netbsd-eabihf", "armv7", "unknown", "netbsd", "eabihf");
        check_parse_3("armv7s-apple-ios", "armv7s", "apple", "ios");
        check_parse_3("asmjs-unknown-emscripten", "asmjs", "unknown", "emscripten");
        check_parse_3("i386-apple-ios", "i386", "apple", "ios");
        check_parse_4("i586-pc-windows-msvc", "i586", "pc", "windows", "msvc");
        check_parse_4("i586-unknown-linux-gnu", "i586", "unknown", "linux", "gnu");
        check_parse_4("i586-unknown-linux-musl", "i586", "unknown", "linux", "musl");
        check_parse_3("i686-apple-darwin", "i686", "apple", "darwin");
        check_parse_3("i686-linux-android", "i686", "linux", "android");
        check_parse_4("i686-pc-windows-gnu", "i686", "pc", "windows", "gnu");
        check_parse_4("i686-pc-windows-msvc", "i686", "pc", "windows", "msvc");
        check_parse_3("i686-unknown-cloudabi", "i686", "unknown", "cloudabi");
        check_parse_3("i686-unknown-dragonfly", "i686", "unknown", "dragonfly");
        check_parse_3("i686-unknown-freebsd", "i686", "unknown", "freebsd");
        check_parse_3("i686-unknown-haiku", "i686", "unknown", "haiku");
        check_parse_4("i686-unknown-linux-gnu", "i686", "unknown", "linux", "gnu");
        check_parse_4("i686-unknown-linux-musl", "i686", "unknown", "linux", "musl");
        check_parse_3("i686-unknown-netbsd", "i686", "unknown", "netbsd");
        check_parse_3("i686-unknown-openbsd", "i686", "unknown", "openbsd");
        check_parse_4("mips-unknown-linux-gnu", "mips", "unknown", "linux", "gnu");
        check_parse_4("mips-unknown-linux-musl", "mips", "unknown", "linux", "musl");
        check_parse_4("mips-unknown-linux-uclibc", "mips", "unknown", "linux", "uclibc");
        check_parse_4("mips64-unknown-linux-gnuabi64", "mips64", "unknown", "linux", "gnuabi64");
        check_parse_4("mips64el-unknown-linux-gnuabi64", "mips64el", "unknown", "linux", "gnuabi64");
        check_parse_4("mipsel-unknown-linux-gnu", "mipsel", "unknown", "linux", "gnu");
        check_parse_4("mipsel-unknown-linux-musl", "mipsel", "unknown", "linux", "musl");
        check_parse_4("mipsel-unknown-linux-uclibc", "mipsel", "unknown", "linux", "uclibc");
        check_parse_3("msp430-none-elf", "msp430", "none", "elf");
        check_parse_4("powerpc-unknown-linux-gnu", "powerpc", "unknown", "linux", "gnu");
        check_parse_4("powerpc-unknown-linux-gnuspe", "powerpc", "unknown", "linux", "gnuspe");
        check_parse_3("powerpc-unknown-netbsd", "powerpc", "unknown", "netbsd");
        check_parse_4("powerpc64-unknown-linux-gnu", "powerpc64", "unknown", "linux", "gnu");
        check_parse_4("powerpc64le-unknown-linux-gnu", "powerpc64le", "unknown", "linux", "gnu");
        check_parse_4("s390x-unknown-linux-gnu", "s390x", "unknown", "linux", "gnu");
        check_parse_4("sparc-unknown-linux-gnu", "sparc", "unknown", "linux", "gnu");
        check_parse_4("sparc64-unknown-linux-gnu", "sparc64", "unknown", "linux", "gnu");
        check_parse_3("sparc64-unknown-netbsd", "sparc64", "unknown", "netbsd");
        check_parse_3("sparcv9-sun-solaris", "sparcv9", "sun", "solaris");
        check_parse_3("thumbv6m-none-eabi", "thumbv6m", "none", "eabi");
        check_parse_3("thumbv7em-none-eabi", "thumbv7em", "none", "eabi");
        check_parse_3("thumbv7em-none-eabihf", "thumbv7em", "none", "eabihf");
        check_parse_3("thumbv7m-none-eabi", "thumbv7m", "none", "eabi");
        check_parse_3("wasm32-experimental-emscripten", "wasm32", "experimental", "emscripten");
        check_parse_3("wasm32-unknown-emscripten", "wasm32", "unknown", "emscripten");
        check_parse_3("wasm32-unknown-unknown", "wasm32", "unknown", "unknown");
        check_parse_3("x86_64-apple-darwin", "x86_64", "apple", "darwin");
        check_parse_3("x86_64-apple-ios", "x86_64", "apple", "ios");
        check_parse_3("x86_64-linux-android", "x86_64", "linux", "android");
        check_parse_4("x86_64-pc-windows-gnu", "x86_64", "pc", "windows", "gnu");
        check_parse_4("x86_64-pc-windows-msvc", "x86_64", "pc", "windows", "msvc");
        check_parse_3("x86_64-rumprun-netbsd", "x86_64", "rumprun", "netbsd");
        check_parse_3("x86_64-sun-solaris", "x86_64", "sun", "solaris");
        check_parse_3("x86_64-unknown-bitrig", "x86_64", "unknown", "bitrig");
        check_parse_3("x86_64-unknown-cloudabi", "x86_64", "unknown", "cloudabi");
        check_parse_3("x86_64-unknown-dragonfly", "x86_64", "unknown", "dragonfly");
        check_parse_3("x86_64-unknown-freebsd", "x86_64", "unknown", "freebsd");
        check_parse_3("x86_64-unknown-fuchsia", "x86_64", "unknown", "fuchsia");
        check_parse_3("x86_64-unknown-haiku", "x86_64", "unknown", "haiku");
        check_parse_4("x86_64-unknown-l4re-uclibc", "x86_64", "unknown", "l4re", "uclibc");
        check_parse_4("x86_64-unknown-linux-gnu", "x86_64", "unknown", "linux", "gnu");
        check_parse_4("x86_64-unknown-linux-gnux32", "x86_64", "unknown", "linux", "gnux32");
        check_parse_4("x86_64-unknown-linux-musl", "x86_64", "unknown", "linux", "musl");
        check_parse_3("x86_64-unknown-netbsd", "x86_64", "unknown", "netbsd");
        check_parse_3("x86_64-unknown-openbsd", "x86_64", "unknown", "openbsd");
        check_parse_3("x86_64-unknown-redox", "x86_64", "unknown", "redox");
    }
}
