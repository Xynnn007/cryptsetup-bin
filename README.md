# Test Project for Musl build for libcryptsetup-rs

Run the code

```shell
git clone https://github.com/Xynnn007/cryptsetup-bin.git
cd cryptsetup-bin
docker run --rm -it -v $(pwd):/root/cryptsetup-bin \
   -w /root/cryptsetup-bin docker.io/library/rust@sha256:6c828d9865870a3bc8c02919d73803df22cac59b583d8f2cb30a296abe64748f \
   bash
```
Then, in the container shell, run
```shell
apt update -y && apt install -y libcryptsetup-dev pkg-config clang musl-tools
rustup target add x86_64-unknown-linux-musl

export PKG_CONFIG_SYSROOT_DIR=/
export PKG_CONFIG_PATH=/usr/lib/${ARCH}-linux-gnu/pkgconfig

cargo build --release --target x86_64-unknown-linux-musl
```

and some error like the following will occur

```
error: linking with `musl-gcc` failed: exit status: 1
  |
  = note:  "musl-gcc" "-m64" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/rustcT1s0na/symbols.o" "<3 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "/root/cryptsetup-bin/target/x86_64-unknown-linux-musl/release/deps/{liblibcryptsetup_rs-4720afb3041c01a4.rlib}.rlib" "-lcryptsetup" "/root/cryptsetup-bin/target/x86_64-unknown-linux-musl/release/deps/{libonce_cell-2dbf55d764320de7.rlib,libserde_json-4e5e11c0174b2bf0.rlib,libmemchr-71d4d8c92b1fe120.rlib,libitoa-e70f2b14af217e37.rlib,libryu-a4e6412cb1178e26.rlib,libserde-28f177adc1c8c263.rlib,liblibcryptsetup_rs_sys-8ead40ddb31749e0.rlib}.rlib" "-lcryptsetup" "/root/cryptsetup-bin/target/x86_64-unknown-linux-musl/release/deps/{libuuid-eec6f0c1f9beb0f2.rlib,libgetrandom-d398566de2d52b87.rlib,libcfg_if-166f716fedfe7782.rlib,libbitflags-dd7667da33f92de6.rlib,liblibc-072751853d1dd94e.rlib,libeither-049aff0dc032ac02.rlib}.rlib" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/{libstd-*,libpanic_unwind-*,libobject-*,libmemchr-*,libaddr2line-*,libgimli-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,libadler2-*,libunwind-*}.rlib" "-lunwind" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/{libcfg_if-*,liblibc-*}.rlib" "-lc" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/{librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "-L" "/tmp/rustcT1s0na/raw-dylibs" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-nostartfiles" "-L" "/usr/lib/x86_64-linux-gnu" "-L" "/usr/lib/x86_64-linux-gnu" "-L" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-L" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib" "-o" "/root/cryptsetup-bin/target/x86_64-unknown-linux-musl/release/deps/cryptsetup_bin-077c06d3fb30d9d9" "-Wl,--gc-sections" "-static-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-Wl,--strip-debug" "-nodefaultlibs" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: /usr/bin/ld: cannot find -lcryptsetup: No such file or directory
          /usr/bin/ld: have you installed the static version of the cryptsetup library ?
          /usr/bin/ld: cannot find -lcryptsetup: No such file or directory
          /usr/bin/ld: have you installed the static version of the cryptsetup library ?
          /usr/bin/ld: /usr/local/rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-40490f4ebf668869.rlib(std-40490f4ebf668869.std.60655b9f7d79336e-cgu.0.rcgu.o): in function `<std::sys::net::connection::socket::LookupHost as core::convert::TryFrom<(&str,u16)>>::try_from::{{closure}}':
          /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/net/connection/socket.rs:319:(.text._ZN117_$LT$std..sys..net..connection..socket..LookupHost$u20$as$u20$core..convert..TryFrom$LT$$LP$$RF$str$C$u16$RP$$GT$$GT$8try_from28_$u7b$$u7b$closure$u7d$$u7d$17h24f98f7a159c44f8E+0x57): warning: Using 'getaddrinfo' in statically linked applications requires at runtime the shared libraries from the glibc version used for linking
          /usr/bin/ld: /usr/local/rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-40490f4ebf668869.rlib(std-40490f4ebf668869.std.60655b9f7d79336e-cgu.0.rcgu.o): in function `std::sys::pal::unix::os::home_dir::fallback':
          /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/pal/unix/os.rs:671:(.text._ZN3std3env8home_dir17h65e4dda2fe82ed6bE+0xc5): warning: Using 'getpwuid_r' in statically linked applications requires at runtime the shared libraries from the glibc version used for linking
          collect2: error: ld returned 1 exit status
          /usr/bin/ld: /usr/local/rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-40490f4ebf668869.rlib(std-40490f4ebf668869.std.60655b9f7d79336e-cgu.0.rcgu.o): note: the message above does not take linker garbage collection into account
          /usr/bin/ld: /usr/local/rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-40490f4ebf668869.rlib(std-40490f4ebf668869.std.60655b9f7d79336e-cgu.0.rcgu.o): note: the message above does not take linker garbage collection into account
          

error: could not compile `cryptsetup-bin` (bin "cryptsetup-bin") due to 1 previous error
```