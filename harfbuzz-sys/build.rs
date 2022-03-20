use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(feature = "build-native-harfbuzz")]
extern crate pkg_config;

#[cfg(feature = "build-native-harfbuzz")]
fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-env-changed=HARFBUZZ_SYS_NO_PKG_CONFIG");
    if target.contains("wasm32") || env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::probe_library("harfbuzz").is_ok() {
            return;
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    // let build_dir = out_path.join("build");

    let source = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("harfbuzz");

    let mut meson = Command::new("meson");
    meson.arg("setup").arg(&out_dir).arg(&source);

    if target.contains("apple") {
        meson.arg("-Dcoretext=enabled");
    }

    if host != target {
        // cross build
        println!(
            "Host: {:?}  Target: {:?}",
            env::var("HOST").unwrap(),
            env::var("TARGET").unwrap()
        );

        let mut cpu_family = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        if cpu_family == "powerpc" {
            cpu_family = "ppc".to_string();
        } else if cpu_family == "powerpc64" {
            cpu_family = "ppc64".to_string();
        }

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        let cpu = match (cpu_family.as_str(), target_os.as_str()) {
            ("aarch64", "ios") => "aarch64",
            ("aarch64", "tvos") => "arm64",
            (v, "android") if v.starts_with("arm") => "Cortex-M0+",
            _ => cpu_family.as_str(),
        };

        let system = if cfg!(target_vendor = "apple") {
            "darwin".to_string()
        } else if cfg!(all(windows, target_env = "gnu")) {
            "cygwin".to_string()
        } else {
            env::var("CARGO_CFG_TARGET_OS").unwrap()
        };

        let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();

        let host_machine = format!(
            "[host_machine]
system = '{}'
cpu_family = '{}'
cpu = '{}'
endian = '{}'",
            system = system,
            cpu_family = cpu_family,
            cpu = cpu,
            endian = env::var("CARGO_CFG_TARGET_ENDIAN").unwrap()
        );

        let binaries;

        let mut sysroot: Option<String> = None;

        if target_vendor == "apple" {
            let sdk_name = apple_sdk();
            let c = apple_sdk_find(&sdk_name, "clang").unwrap();
            let cpp = apple_sdk_find(&sdk_name, "clang++").unwrap();
            let ar = apple_sdk_find(&sdk_name, "ar").unwrap();
            let ld = apple_sdk_find(&sdk_name, "ld").unwrap();
            let strip = apple_sdk_find(&sdk_name, "strip").unwrap();
            binaries = format!(
                "[binaries]
ar = '{ar}'
c = '{c}'
cpp = '{cpp}'
c_ld = '{ld}'
cpp_ld = '{ld}'
strip = '{strip}'",
                c = &c,
                cpp = &cpp,
                ld = &ld,
                strip = strip
            );
            sysroot.replace(apple_sdk_root(&sdk_name).unwrap());
        } else if system == "android" {
            let ab = android_binaries();
            sysroot.replace(ab.0.to_string_lossy().to_string());
            binaries = ab.1;
        } else {
            unimplemented!()
        };

        let sysroot = if let Some(sysroot) = sysroot {
            format!("sys_root = '{}'\n", sysroot)
        } else {
            String::new()
        };

        let cross_file = out_path.join("cross-file.txt");

        std::fs::write(
            &cross_file,
            format!(
                "{}\n\n[properties]\nneeds_exe_wrapper = true\n{}{}",
                host_machine, sysroot, binaries
            ),
        )
        .unwrap();

        meson.arg("--cross-file").arg(&cross_file);
    }

    if let Ok("release") = env::var("PROFILE").as_deref() {
        meson.arg("--buildtype").arg("release");
    }

    meson
        .arg("--prefix")
        .arg(&out_dir)
        .arg("--libdir")
        .arg(&out_path.join("lib"))
        .arg("-Ddocs=disabled")
        .arg("-Dtests=enabled")
        .arg("-Dgraphite=disabled")
        .arg("-Dbenchmark=disabled")
        .arg("-Dicu_builtin=true")
        .arg("-Dc_args=-Wno-unused-but-set-variable")
        .arg("-Dcpp_args=-Wno-unused-but-set-variable")
        // .arg("-Dcpp_args='-Wno-unused-but-set-variable -Wno-unused-parameter -Wno-unused-variable'")
        .arg("--default-library")
        .arg("static")
        .arg("--optimization")
        .arg(&env::var("OPT_LEVEL").unwrap());
    let output = meson.output().unwrap();
    println!(
        "meson setup:\n{}",
        String::from_utf8(output.stdout).unwrap()
    );
    assert!(
        output.status.success(),
        "meson setup faild, exit status: {}, stderr:\n{}",
        output.status,
        String::from_utf8(output.stderr).unwrap()
    );

    let mut meson = Command::new("meson");
    meson.arg("compile").arg("-C").arg(&out_dir);
    let output = meson.output().unwrap();
    println!(
        "meson compile:\n{}",
        String::from_utf8(output.stdout).unwrap()
    );
    assert!(
        output.status.success(),
        "meson compile faild, exit status: {}, stderr:\n{}",
        output.status,
        String::from_utf8(output.stderr).unwrap()
    );

    let mut meson = Command::new("meson");
    meson
        .arg("install")
        .arg("--no-rebuild")
        .arg("-C")
        .arg(&out_dir);
    let output = meson.output().unwrap();
    println!(
        "meson install:\n{}",
        String::from_utf8(output.stdout).unwrap()
    );
    assert!(
        output.status.success(),
        "meson install faild, exit status: {}, stderr:\n{}",
        output.status,
        String::from_utf8(output.stderr).unwrap()
    );

    println!(
        "cargo:include={}",
        out_path.join("include").join("harfbuzz").display()
    );
    println!("cargo:rustc-link-lib=static=harfbuzz");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.join("lib").display()
    );
}

fn apple_sdk() -> String {
    let target = env::var("TARGET").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if cfg!(not(target_vendor = "apple")) || !matches!(os.as_str(), "ios" | "tvos") {
        panic!("unsupported arch `{}` for os `{}`", arch, os);
    }
    let sdk_name = match (arch.as_str(), os.as_str()) {
        ("aarch64", "tvos") => "appletvos",
        ("x86_64", "tvos") => "appletvsimulator",
        ("arm", "ios") => "iphoneos",
        ("aarch64", "ios") if target.contains("macabi") => "macosx",
        ("aarch64", "ios") if target.contains("sim") => "iphonesimulator",
        ("aarch64", "ios") => "iphoneos",
        ("x86", "ios") => "iphonesimulator",
        ("x86_64", "ios") if target.contains("macabi") => "macosx",
        ("x86_64", "ios") => "iphonesimulator",
        _ => {
            panic!("unsupported arch `{}` for os `{}`", arch, os);
        }
    };
    sdk_name.to_string()
}

fn apple_sdk_root(sdk_name: &str) -> Result<String, String> {
    // Following what clang does
    // (https://github.com/llvm/llvm-project/blob/
    // 296a80102a9b72c3eda80558fb78a3ed8849b341/clang/lib/Driver/ToolChains/Darwin.cpp#L1661-L1678)
    // to allow the SDK path to be set. (For clang, xcrun sets
    // SDKROOT; for rustc, the user or build system can set it, or we
    // can fall back to checking for xcrun on PATH.)
    if let Ok(sdkroot) = env::var("SDKROOT") {
        let p = Path::new(&sdkroot);
        match sdk_name {
            // Ignore `SDKROOT` if it's clearly set for the wrong platform.
            "appletvos"
                if sdkroot.contains("TVSimulator.platform")
                    || sdkroot.contains("MacOSX.platform") => {}
            "appletvsimulator"
                if sdkroot.contains("TVOS.platform") || sdkroot.contains("MacOSX.platform") => {}
            "iphoneos"
                if sdkroot.contains("iPhoneSimulator.platform")
                    || sdkroot.contains("MacOSX.platform") => {}
            "iphonesimulator"
                if sdkroot.contains("iPhoneOS.platform") || sdkroot.contains("MacOSX.platform") => {
            }
            "macosx10.15"
                if sdkroot.contains("iPhoneOS.platform")
                    || sdkroot.contains("iPhoneSimulator.platform") => {}
            // Ignore `SDKROOT` if it's not a valid path.
            _ if !p.is_absolute() || p == Path::new("/") || !p.exists() => {}
            _ => return Ok(sdkroot),
        }
    }
    let res = Command::new("xcrun")
        .arg("--show-sdk-path")
        .arg("-sdk")
        .arg(sdk_name)
        .output()
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                let error = String::from_utf8(output.stderr);
                let error = format!("process exit with error: {}", error.unwrap());
                Err(io::Error::new(io::ErrorKind::Other, &error[..]))
            }
        });

    match res {
        Ok(output) => Ok(output.trim().to_string()),
        Err(e) => Err(format!("failed to get {} SDK path: {}", sdk_name, e)),
    }
}

fn apple_sdk_find(sdk_name: &str, tool: &str) -> Result<String, String> {
    let res = Command::new("xcrun")
        .arg("-find")
        .arg("-sdk")
        .arg(sdk_name)
        .arg(tool)
        .output()
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                let error = String::from_utf8(output.stderr);
                let error = format!("process exit with error: {}", error.unwrap());
                Err(io::Error::new(io::ErrorKind::Other, &error[..]))
            }
        });

    match res {
        Ok(output) => Ok(output.trim().to_string()),
        Err(e) => Err(format!("failed to get {} SDK path: {}", sdk_name, e)),
    }
}

fn android_binaries() -> (PathBuf, String) {
    let ndk = ndk_build::ndk::Ndk::from_env().unwrap();
    let mut platforms = ndk.platforms().to_vec();
    dbg!(&platforms);
    platforms.sort();
    let platform = ndk.default_platform();
    let abi = ndk_build::target::Target::from_rust_triple(&env::var("TARGET").unwrap()).unwrap();
    let (c, cc) = ndk.clang(abi, platform).unwrap();
    let ar = ndk.toolchain_bin("ar", abi).unwrap();
    println!("using ar: {}", &ar.display());
    let ld = ndk.toolchain_dir().unwrap().join("bin").join("ld.lld");
    let strip = ndk.toolchain_bin("strip", abi).unwrap();
    (
        ndk.sysroot_platform_lib_dir(abi, 21).unwrap(),
        format!(
            "[binaries]
ar = '{ar}'
c = '{c}'
cpp = '{cpp}'
c_ld = '{ld}'
cpp_ld = '{ld}'
strip = '{strip}'",
            ar = ar.display(),
            c = c.display(),
            cpp = cc.display(),
            ld = ld.display(),
            strip = strip.display()
        ),
    )
}

#[cfg(not(feature = "build-native-harfbuzz"))]
fn main() {}
