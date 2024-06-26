#![cfg_attr(not(test), no_std)]

extern crate alloc;

use memory_addr::{align_up_4k, is_aligned_4k};
use fileops::iovec;
use fileops::get_user_str;

#[macro_use]
extern crate log;

const MAX_SYSCALL_ARGS: usize = 6;
pub type SyscallArgs = [usize; MAX_SYSCALL_ARGS];

pub fn do_syscall(args: SyscallArgs, sysno: usize) -> usize {
    match sysno {
        LINUX_SYSCALL_IOCTL => {
            linux_syscall_ioctl(args)
        },
        LINUX_SYSCALL_GETCWD => {
            linux_syscall_getcwd(args)
        },
        LINUX_SYSCALL_CHDIR => {
            linux_syscall_chdir(args)
        },
        LINUX_SYSCALL_FACCESSAT => {
            linux_syscall_faccessat(args)
        },
        LINUX_SYSCALL_CHMODAT => {
            linux_syscall_fchmodat(args)
        },
        LINUX_SYSCALL_CHOWNAT => {
            linux_syscall_fchownat(args)
        },
        LINUX_SYSCALL_MKDIRAT => {
            linux_syscall_mkdirat(args)
        },
        LINUX_SYSCALL_UNLINKAT => {
            linux_syscall_unlinkat(args)
        },
        LINUX_SYSCALL_OPENAT => {
            linux_syscall_openat(args)
        },
        LINUX_SYSCALL_CLOSE => {
            linux_syscall_close(args)
        },
        LINUX_SYSCALL_READ => {
            linux_syscall_read(args)
        },
        LINUX_SYSCALL_GETDENTS64 => {
            linux_syscall_getdents64(args)
        },
        LINUX_SYSCALL_WRITE => {
            linux_syscall_write(args)
        },
        LINUX_SYSCALL_WRITEV => {
            linux_syscall_writev(args)
        },
        LINUX_SYSCALL_READLINKAT => {
            usize::MAX
        },
        LINUX_SYSCALL_FSTATAT => {
            linux_syscall_fstatat(args)
        },
        LINUX_SYSCALL_UNAME => {
            linux_syscall_uname(args)
        },
        LINUX_SYSCALL_BRK => {
            linux_syscall_brk(args)
        },
        LINUX_SYSCALL_MUNMAP => {
            linux_syscall_munmap(args)
        },
        LINUX_SYSCALL_MMAP => {
            linux_syscall_mmap(args)
        },
        LINUX_SYSCALL_MSYNC => {
            linux_syscall_msync(args)
        },
        LINUX_SYSCALL_MPROTECT => {
            linux_syscall_mprotect(args)
        },
        LINUX_SYSCALL_SET_TID_ADDRESS => {
            linux_syscall_set_tid_address(args)
        },
        LINUX_SYSCALL_SET_ROBUST_LIST=> {
            linux_syscall_set_robust_list(args)
        },
        LINUX_SYSCALL_PRLIMIT64 => {
            linux_syscall_prlimit64(args)
        },
        LINUX_SYSCALL_GETRANDOM => {
            linux_syscall_getrandom(args)
        },
        LINUX_SYSCALL_CLOCK_GETTIME => {
            linux_syscall_clock_gettime(args)
        },
        LINUX_SYSCALL_RT_SIGPROCMASK => {
            linux_syscall_rt_sigprocmask(args)
        },
        LINUX_SYSCALL_RT_SIGACTION => {
            linux_syscall_rt_sigaction(args)
        },
        LINUX_SYSCALL_GETTID => {
            linux_syscall_gettid(args)
        },
        LINUX_SYSCALL_GETPID => {
            linux_syscall_getpid(args)
        },
        LINUX_SYSCALL_GETGID => {
            linux_syscall_getgid(args)
        },
        LINUX_SYSCALL_TGKILL => {
            linux_syscall_tgkill(args)
        },
        LINUX_SYSCALL_EXIT => {
            linux_syscall_exit(args)
        },
        LINUX_SYSCALL_EXIT_GROUP => {
            linux_syscall_exit_group(args)
        },
        _ => {
            panic!("Unsupport sysno: {}, {:#X}", sysno, sysno);
        }
    }
}

//
// Linux syscall
//
const LINUX_SYSCALL_GETCWD:     usize = 0x11;
const LINUX_SYSCALL_IOCTL:      usize = 0x1d;
const LINUX_SYSCALL_MKDIRAT:    usize = 0x22;
const LINUX_SYSCALL_UNLINKAT:   usize = 0x23;
const LINUX_SYSCALL_FACCESSAT:  usize = 0x30;
const LINUX_SYSCALL_CHDIR:      usize = 0x31;
const LINUX_SYSCALL_CHMODAT:    usize = 0x35;
const LINUX_SYSCALL_CHOWNAT:    usize = 0x36;
const LINUX_SYSCALL_OPENAT:     usize = 0x38;
const LINUX_SYSCALL_CLOSE:      usize = 0x39;
const LINUX_SYSCALL_GETDENTS64: usize = 0x3d;
const LINUX_SYSCALL_READ:       usize = 0x3f;
const LINUX_SYSCALL_WRITE:      usize = 0x40;
const LINUX_SYSCALL_WRITEV:     usize = 0x42;
const LINUX_SYSCALL_READLINKAT: usize = 0x4e;
const LINUX_SYSCALL_FSTATAT:    usize = 0x4f;
const LINUX_SYSCALL_EXIT:       usize = 0x5d;
const LINUX_SYSCALL_EXIT_GROUP: usize = 0x5e;
const LINUX_SYSCALL_TGKILL:     usize = 0x83;
const LINUX_SYSCALL_UNAME:      usize = 0xa0;
const LINUX_SYSCALL_GETPID:     usize = 0xac;
const LINUX_SYSCALL_GETGID:     usize = 0xb0;
const LINUX_SYSCALL_GETTID:     usize = 0xb2;
const LINUX_SYSCALL_BRK:        usize = 0xd6;
const LINUX_SYSCALL_MUNMAP:     usize = 0xd7;
const LINUX_SYSCALL_MMAP:       usize = 0xde;
const LINUX_SYSCALL_MPROTECT:   usize = 0xe2;
const LINUX_SYSCALL_MSYNC:      usize = 0xe3;
const LINUX_SYSCALL_PRLIMIT64:  usize = 0x105;
const LINUX_SYSCALL_GETRANDOM:  usize = 0x116;

const LINUX_SYSCALL_SET_TID_ADDRESS:    usize = 0x60;
const LINUX_SYSCALL_SET_ROBUST_LIST:    usize = 0x63;
const LINUX_SYSCALL_CLOCK_GETTIME:      usize = 0x71;
const LINUX_SYSCALL_RT_SIGACTION:       usize = 0x86;
const LINUX_SYSCALL_RT_SIGPROCMASK:     usize = 0x87;

fn linux_syscall_faccessat(args: SyscallArgs) -> usize {
    let [dfd, filename, mode, ..] = args;
    info!("linux_syscall_faccessat dfd {:#X} filename {:#X} mode {}",
           dfd, filename, mode);
    let filename = get_user_str(filename);
    warn!("filename: {}", filename);
    0
}

fn linux_syscall_fchownat(args: SyscallArgs) -> usize {
    let [dfd, pathname, owner, group, flags, ..] = args;
    let pathname = get_user_str(pathname);
    warn!("impl fchownat dfd {:#X} path {} owner:group {}:{} flags {:#X}",
        dfd, pathname, owner, group, flags);
    0
}

fn linux_syscall_fchmodat(args: SyscallArgs) -> usize {
    let [dfd, pathname, mode, flags, ..] = args;
    let pathname = get_user_str(pathname);
    warn!("impl fchmodat dfd {:#X} path {} mode {:#o} flags {:#X}",
        dfd, pathname, mode, flags);
    0
}

fn linux_syscall_mkdirat(args: SyscallArgs) -> usize {
    let [dfd, pathname, mode, ..] = args;
    let pathname = get_user_str(pathname);
    fileops::mkdirat(dfd, &pathname, mode)
}

fn linux_syscall_unlinkat(args: SyscallArgs) -> usize {
    let [dfd, path, flags, ..] = args;
    let path = get_user_str(path);
    warn!("impl unlinkat dfd {}, path {} flags {:#X}", dfd, path, flags);
    0
}

fn linux_syscall_openat(args: SyscallArgs) -> usize {
    let [dfd, filename, flags, mode, ..] = args;

    let filename = get_user_str(filename);
    info!("filename: {}\n", filename);
    fileops::register_file(fileops::openat(dfd, &filename, flags, mode))
}

fn linux_syscall_close(_args: SyscallArgs) -> usize {
    info!("Todo: linux_syscall_close");
    0
}

fn linux_syscall_read(args: SyscallArgs) -> usize {
    let [fd, buf, count, ..] = args;

    let ubuf = unsafe {
        core::slice::from_raw_parts_mut(buf as *mut u8, count)
    };

    fileops::read(fd, ubuf)
}

fn linux_syscall_getdents64(args: SyscallArgs) -> usize {
    let [fd, _dirp, count, ..] = args;
    warn!("impl linux_syscall_getdents64 fd {}, count {}", fd, count);
    0
}

fn linux_syscall_write(args: SyscallArgs) -> usize {
    let [fd, buf, size, ..] = args;
    info!("write: {:#x}, {:#x}, {:#x}", fd, buf, size);

    let ubuf = unsafe { core::slice::from_raw_parts(buf as *const u8, size) };
    fileops::write(fd, ubuf)
}

fn linux_syscall_writev(args: SyscallArgs) -> usize {
    let [fd, array, size, ..] = args;
    info!("writev: {:#x}, {:#x}, {:#x}", fd, array, size);

    let iov_array = unsafe { core::slice::from_raw_parts(array as *const iovec, size) };
    fileops::writev(fd, iov_array)
}

fn linux_syscall_fstatat(args: SyscallArgs) -> usize {
    let [dfd, path, statbuf, flags, ..] = args;
    fileops::fstatat(dfd, path, statbuf, flags)
}

fn linux_syscall_mmap(args: SyscallArgs) -> usize {
    let [va, len, prot, flags, fd, offset] = args;
    assert!(is_aligned_4k(va));
    debug!("###### mmap!!! {:#x} {:#x} {:#x} {:#x} {:#x} {:#x}", va, len, prot, flags, fd, offset);

    mmap::mmap(va, len, prot, flags, fd, offset).unwrap()
}

fn linux_syscall_msync(args: SyscallArgs) -> usize {
    let [va, len, flags, ..] = args;
    mmap::msync(va, len, flags)
}

fn linux_syscall_ioctl(args: SyscallArgs) -> usize {
    let [fd, request, udata, ..] = args;
    fileops::ioctl(fd, request, udata)
}

fn linux_syscall_getcwd(args: SyscallArgs) -> usize {
    let [buf, size, ..] = args;

    let ubuf = unsafe {
        core::slice::from_raw_parts_mut(buf as *mut u8, size)
    };
    fileops::getcwd(ubuf)
}

fn linux_syscall_chdir(args: SyscallArgs) -> usize {
    let [pathname, ..] = args;
    let pathname = get_user_str(pathname);
    fileops::chdir(&pathname)
}

fn linux_syscall_mprotect(_args: SyscallArgs) -> usize {
    warn!("impl linux_syscall_mprotect");
    0
}

fn linux_syscall_set_tid_address(_args: SyscallArgs) -> usize {
    warn!("impl linux_syscall_set_tid_address");
    0
}

fn linux_syscall_set_robust_list(_args: SyscallArgs) -> usize {
    warn!("impl linux_syscall_set_robust_list");
    0
}

fn linux_syscall_prlimit64(args: SyscallArgs) -> usize {
    let [pid, resource, new_rlim, old_rlim, ..] = args;
    sys::prlimit64(pid, resource, new_rlim, old_rlim)
}

fn linux_syscall_getrandom(args: SyscallArgs) -> usize {
    let [buf, len, flags, ..] = args;
    warn!("impl linux_syscall_getrandom buf {:#X}, len {} flags {:#X}", buf, len, flags);
    0
}

fn linux_syscall_clock_gettime(_args: SyscallArgs) -> usize {
    warn!("impl linux_syscall_clock_gettime");
    0
}

fn linux_syscall_rt_sigprocmask(args: SyscallArgs) -> usize {
    let [how, set, oldset, sigsetsize, ..] = args;
    warn!("impl linux_syscall_rt_sigprocmask how {} set {:#X} oldset {:#X} size {}",
          how, set, oldset, sigsetsize);
    0
}

fn linux_syscall_rt_sigaction(_args: SyscallArgs) -> usize {
    warn!("impl linux_syscall_rt_sigaction");
    0
}

fn linux_syscall_gettid(_args: SyscallArgs) -> usize {
    sys::gettid()
}

fn linux_syscall_getpid(_args: SyscallArgs) -> usize {
    sys::getpid()
}

fn linux_syscall_getgid(_args: SyscallArgs) -> usize {
    sys::getgid()
}

fn linux_syscall_tgkill(_args: SyscallArgs) -> usize {
    warn!("impl linux_syscall_tgkill");
    0
}

const UTS_LEN: usize = 64;

#[repr(C)]
struct utsname {
    sysname: [u8; UTS_LEN + 1],
    nodename: [u8; UTS_LEN + 1],
    release: [u8; UTS_LEN + 1],
    version: [u8; UTS_LEN + 1],
    machine: [u8; UTS_LEN + 1],
    domainname: [u8; UTS_LEN + 1],
}

fn linux_syscall_uname(args: SyscallArgs) -> usize {
    let ptr = args[0];
    info!("uname: {:#x}", ptr);

    let uname = unsafe { (ptr as *mut utsname).as_mut().unwrap() };

    init_bytes_from_str(&mut uname.sysname[..], "Linux");
    init_bytes_from_str(&mut uname.nodename[..], "host");
    init_bytes_from_str(&mut uname.domainname[..], "(none)");
    init_bytes_from_str(&mut uname.release[..], "5.9.0-rc4+");
    init_bytes_from_str(&mut uname.version[..], "#1337 SMP Fri Mar 4 09:36:42 CST 2022");
    init_bytes_from_str(&mut uname.machine[..], "riscv64");

    return 0;
}

fn init_bytes_from_str(dst: &mut [u8], src: &str) {
    let src = src.as_bytes();
    let (left, right) = dst.split_at_mut(src.len());
    axhal::arch::enable_sum();
    left.copy_from_slice(src);
    right.fill(0);
    axhal::arch::disable_sum();
}

fn linux_syscall_brk(args: SyscallArgs) -> usize {
    let va = align_up_4k(args[0]);
    mmap::set_brk(va)
}

fn linux_syscall_munmap(args: SyscallArgs) -> usize {
    let [va, len, ..] = args;
    debug!("munmap!!! {:#x} {:#x}", va, len);
    unimplemented!();
    //return 0;
}

fn linux_syscall_exit(args: SyscallArgs) -> usize {
    let ret = args[0] as i32;
    warn!("impl exit ...{}", ret);
    task::exit(ret);
}

fn linux_syscall_exit_group(_tf: SyscallArgs) -> usize {
    warn!("impl exit_group!");
    return 0;
}

pub fn init() {
}
