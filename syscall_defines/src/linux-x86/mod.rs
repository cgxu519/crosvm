// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Generated with: cat arch/x86/entry/syscalls/syscall_32.tbl |
//    awk ' { print "SYS_" $3 " = " $1"," } '
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum LinuxSyscall {
    SYS_restart_syscall = 0,
    SYS_exit = 1,
    SYS_fork = 2,
    SYS_read = 3,
    SYS_write = 4,
    SYS_open = 5,
    SYS_close = 6,
    SYS_waitpid = 7,
    SYS_creat = 8,
    SYS_link = 9,
    SYS_unlink = 10,
    SYS_execve = 11,
    SYS_chdir = 12,
    SYS_time = 13,
    SYS_mknod = 14,
    SYS_chmod = 15,
    SYS_lchown = 16,
    SYS_break = 17,
    SYS_oldstat = 18,
    SYS_lseek = 19,
    SYS_getpid = 20,
    SYS_mount = 21,
    SYS_umount = 22,
    SYS_setuid = 23,
    SYS_getuid = 24,
    SYS_stime = 25,
    SYS_ptrace = 26,
    SYS_alarm = 27,
    SYS_oldfstat = 28,
    SYS_pause = 29,
    SYS_utime = 30,
    SYS_stty = 31,
    SYS_gtty = 32,
    SYS_access = 33,
    SYS_nice = 34,
    SYS_ftime = 35,
    SYS_sync = 36,
    SYS_kill = 37,
    SYS_rename = 38,
    SYS_mkdir = 39,
    SYS_rmdir = 40,
    SYS_dup = 41,
    SYS_pipe = 42,
    SYS_times = 43,
    SYS_prof = 44,
    SYS_brk = 45,
    SYS_setgid = 46,
    SYS_getgid = 47,
    SYS_signal = 48,
    SYS_geteuid = 49,
    SYS_getegid = 50,
    SYS_acct = 51,
    SYS_umount2 = 52,
    SYS_lock = 53,
    SYS_ioctl = 54,
    SYS_fcntl = 55,
    SYS_mpx = 56,
    SYS_setpgid = 57,
    SYS_ulimit = 58,
    SYS_oldolduname = 59,
    SYS_umask = 60,
    SYS_chroot = 61,
    SYS_ustat = 62,
    SYS_dup2 = 63,
    SYS_getppid = 64,
    SYS_getpgrp = 65,
    SYS_setsid = 66,
    SYS_sigaction = 67,
    SYS_sgetmask = 68,
    SYS_ssetmask = 69,
    SYS_setreuid = 70,
    SYS_setregid = 71,
    SYS_sigsuspend = 72,
    SYS_sigpending = 73,
    SYS_sethostname = 74,
    SYS_setrlimit = 75,
    SYS_getrlimit = 76,
    SYS_getrusage = 77,
    SYS_gettimeofday = 78,
    SYS_settimeofday = 79,
    SYS_getgroups = 80,
    SYS_setgroups = 81,
    SYS_select = 82,
    SYS_symlink = 83,
    SYS_oldlstat = 84,
    SYS_readlink = 85,
    SYS_uselib = 86,
    SYS_swapon = 87,
    SYS_reboot = 88,
    SYS_readdir = 89,
    SYS_mmap = 90,
    SYS_munmap = 91,
    SYS_truncate = 92,
    SYS_ftruncate = 93,
    SYS_fchmod = 94,
    SYS_fchown = 95,
    SYS_getpriority = 96,
    SYS_setpriority = 97,
    SYS_profil = 98,
    SYS_statfs = 99,
    SYS_fstatfs = 100,
    SYS_ioperm = 101,
    SYS_socketcall = 102,
    SYS_syslog = 103,
    SYS_setitimer = 104,
    SYS_getitimer = 105,
    SYS_stat = 106,
    SYS_lstat = 107,
    SYS_fstat = 108,
    SYS_olduname = 109,
    SYS_iopl = 110,
    SYS_vhangup = 111,
    SYS_idle = 112,
    SYS_vm86old = 113,
    SYS_wait4 = 114,
    SYS_swapoff = 115,
    SYS_sysinfo = 116,
    SYS_ipc = 117,
    SYS_fsync = 118,
    SYS_sigreturn = 119,
    SYS_clone = 120,
    SYS_setdomainname = 121,
    SYS_uname = 122,
    SYS_modify_ldt = 123,
    SYS_adjtimex = 124,
    SYS_mprotect = 125,
    SYS_sigprocmask = 126,
    SYS_create_module = 127,
    SYS_init_module = 128,
    SYS_delete_module = 129,
    SYS_get_kernel_syms = 130,
    SYS_quotactl = 131,
    SYS_getpgid = 132,
    SYS_fchdir = 133,
    SYS_bdflush = 134,
    SYS_sysfs = 135,
    SYS_personality = 136,
    SYS_afs_syscall = 137,
    SYS_setfsuid = 138,
    SYS_setfsgid = 139,
    SYS__llseek = 140,
    SYS_getdents = 141,
    SYS__newselect = 142,
    SYS_flock = 143,
    SYS_msync = 144,
    SYS_readv = 145,
    SYS_writev = 146,
    SYS_getsid = 147,
    SYS_fdatasync = 148,
    SYS__sysctl = 149,
    SYS_mlock = 150,
    SYS_munlock = 151,
    SYS_mlockall = 152,
    SYS_munlockall = 153,
    SYS_sched_setparam = 154,
    SYS_sched_getparam = 155,
    SYS_sched_setscheduler = 156,
    SYS_sched_getscheduler = 157,
    SYS_sched_yield = 158,
    SYS_sched_get_priority_max = 159,
    SYS_sched_get_priority_min = 160,
    SYS_sched_rr_get_interval = 161,
    SYS_nanosleep = 162,
    SYS_mremap = 163,
    SYS_setresuid = 164,
    SYS_getresuid = 165,
    SYS_vm86 = 166,
    SYS_query_module = 167,
    SYS_poll = 168,
    SYS_nfsservctl = 169,
    SYS_setresgid = 170,
    SYS_getresgid = 171,
    SYS_prctl = 172,
    SYS_rt_sigreturn = 173,
    SYS_rt_sigaction = 174,
    SYS_rt_sigprocmask = 175,
    SYS_rt_sigpending = 176,
    SYS_rt_sigtimedwait = 177,
    SYS_rt_sigqueueinfo = 178,
    SYS_rt_sigsuspend = 179,
    SYS_pread64 = 180,
    SYS_pwrite64 = 181,
    SYS_chown = 182,
    SYS_getcwd = 183,
    SYS_capget = 184,
    SYS_capset = 185,
    SYS_sigaltstack = 186,
    SYS_sendfile = 187,
    SYS_getpmsg = 188,
    SYS_putpmsg = 189,
    SYS_vfork = 190,
    SYS_ugetrlimit = 191,
    SYS_mmap2 = 192,
    SYS_truncate64 = 193,
    SYS_ftruncate64 = 194,
    SYS_stat64 = 195,
    SYS_lstat64 = 196,
    SYS_fstat64 = 197,
    SYS_lchown32 = 198,
    SYS_getuid32 = 199,
    SYS_getgid32 = 200,
    SYS_geteuid32 = 201,
    SYS_getegid32 = 202,
    SYS_setreuid32 = 203,
    SYS_setregid32 = 204,
    SYS_getgroups32 = 205,
    SYS_setgroups32 = 206,
    SYS_fchown32 = 207,
    SYS_setresuid32 = 208,
    SYS_getresuid32 = 209,
    SYS_setresgid32 = 210,
    SYS_getresgid32 = 211,
    SYS_chown32 = 212,
    SYS_setuid32 = 213,
    SYS_setgid32 = 214,
    SYS_setfsuid32 = 215,
    SYS_setfsgid32 = 216,
    SYS_pivot_root = 217,
    SYS_mincore = 218,
    SYS_madvise = 219,
    SYS_getdents64 = 220,
    SYS_fcntl64 = 221,
    SYS_gettid = 224,
    SYS_readahead = 225,
    SYS_setxattr = 226,
    SYS_lsetxattr = 227,
    SYS_fsetxattr = 228,
    SYS_getxattr = 229,
    SYS_lgetxattr = 230,
    SYS_fgetxattr = 231,
    SYS_listxattr = 232,
    SYS_llistxattr = 233,
    SYS_flistxattr = 234,
    SYS_removexattr = 235,
    SYS_lremovexattr = 236,
    SYS_fremovexattr = 237,
    SYS_tkill = 238,
    SYS_sendfile64 = 239,
    SYS_futex = 240,
    SYS_sched_setaffinity = 241,
    SYS_sched_getaffinity = 242,
    SYS_set_thread_area = 243,
    SYS_get_thread_area = 244,
    SYS_io_setup = 245,
    SYS_io_destroy = 246,
    SYS_io_getevents = 247,
    SYS_io_submit = 248,
    SYS_io_cancel = 249,
    SYS_fadvise64 = 250,
    SYS_exit_group = 252,
    SYS_lookup_dcookie = 253,
    SYS_epoll_create = 254,
    SYS_epoll_ctl = 255,
    SYS_epoll_wait = 256,
    SYS_remap_file_pages = 257,
    SYS_set_tid_address = 258,
    SYS_timer_create = 259,
    SYS_timer_settime = 260,
    SYS_timer_gettime = 261,
    SYS_timer_getoverrun = 262,
    SYS_timer_delete = 263,
    SYS_clock_settime = 264,
    SYS_clock_gettime = 265,
    SYS_clock_getres = 266,
    SYS_clock_nanosleep = 267,
    SYS_statfs64 = 268,
    SYS_fstatfs64 = 269,
    SYS_tgkill = 270,
    SYS_utimes = 271,
    SYS_fadvise64_64 = 272,
    SYS_vserver = 273,
    SYS_mbind = 274,
    SYS_get_mempolicy = 275,
    SYS_set_mempolicy = 276,
    SYS_mq_open = 277,
    SYS_mq_unlink = 278,
    SYS_mq_timedsend = 279,
    SYS_mq_timedreceive = 280,
    SYS_mq_notify = 281,
    SYS_mq_getsetattr = 282,
    SYS_kexec_load = 283,
    SYS_waitid = 284,
    SYS_sys_setaltroot = 285,
    SYS_add_key = 286,
    SYS_request_key = 287,
    SYS_keyctl = 288,
    SYS_ioprio_set = 289,
    SYS_ioprio_get = 290,
    SYS_inotify_init = 291,
    SYS_inotify_add_watch = 292,
    SYS_inotify_rm_watch = 293,
    SYS_migrate_pages = 294,
    SYS_openat = 295,
    SYS_mkdirat = 296,
    SYS_mknodat = 297,
    SYS_fchownat = 298,
    SYS_futimesat = 299,
    SYS_fstatat64 = 300,
    SYS_unlinkat = 301,
    SYS_renameat = 302,
    SYS_linkat = 303,
    SYS_symlinkat = 304,
    SYS_readlinkat = 305,
    SYS_fchmodat = 306,
    SYS_faccessat = 307,
    SYS_pselect6 = 308,
    SYS_ppoll = 309,
    SYS_unshare = 310,
    SYS_set_robust_list = 311,
    SYS_get_robust_list = 312,
    SYS_splice = 313,
    SYS_sync_file_range = 314,
    SYS_tee = 315,
    SYS_vmsplice = 316,
    SYS_move_pages = 317,
    SYS_getcpu = 318,
    SYS_epoll_pwait = 319,
    SYS_utimensat = 320,
    SYS_signalfd = 321,
    SYS_timerfd_create = 322,
    SYS_eventfd = 323,
    SYS_fallocate = 324,
    SYS_timerfd_settime = 325,
    SYS_timerfd_gettime = 326,
    SYS_signalfd4 = 327,
    SYS_eventfd2 = 328,
    SYS_epoll_create1 = 329,
    SYS_dup3 = 330,
    SYS_pipe2 = 331,
    SYS_inotify_init1 = 332,
    SYS_preadv = 333,
    SYS_pwritev = 334,
    SYS_rt_tgsigqueueinfo = 335,
    SYS_perf_event_open = 336,
    SYS_recvmmsg = 337,
    SYS_fanotify_init = 338,
    SYS_fanotify_mark = 339,
    SYS_prlimit64 = 340,
    SYS_name_to_handle_at = 341,
    SYS_open_by_handle_at = 342,
    SYS_clock_adjtime = 343,
    SYS_syncfs = 344,
    SYS_sendmmsg = 345,
    SYS_setns = 346,
    SYS_process_vm_readv = 347,
    SYS_process_vm_writev = 348,
    SYS_kcmp = 349,
    SYS_finit_module = 350,
    SYS_sched_setattr = 351,
    SYS_sched_getattr = 352,
    SYS_renameat2 = 353,
    SYS_seccomp = 354,
    SYS_getrandom = 355,
    SYS_memfd_create = 356,
    SYS_bpf = 357,
    SYS_execveat = 358,
    SYS_socket = 359,
    SYS_socketpair = 360,
    SYS_bind = 361,
    SYS_connect = 362,
    SYS_listen = 363,
    SYS_accept4 = 364,
    SYS_getsockopt = 365,
    SYS_setsockopt = 366,
    SYS_getsockname = 367,
    SYS_getpeername = 368,
    SYS_sendto = 369,
    SYS_sendmsg = 370,
    SYS_recvfrom = 371,
    SYS_recvmsg = 372,
    SYS_shutdown = 373,
    SYS_userfaultfd = 374,
    SYS_membarrier = 375,
    SYS_mlock2 = 376,
}

pub fn from_name(name: &str) -> Result<LinuxSyscall, super::Error> {
    match name {
        "restart_syscall" => Ok(LinuxSyscall::SYS_restart_syscall),
        "exit" => Ok(LinuxSyscall::SYS_exit),
        "fork" => Ok(LinuxSyscall::SYS_fork),
        "read" => Ok(LinuxSyscall::SYS_read),
        "write" => Ok(LinuxSyscall::SYS_write),
        "open" => Ok(LinuxSyscall::SYS_open),
        "close" => Ok(LinuxSyscall::SYS_close),
        "waitpid" => Ok(LinuxSyscall::SYS_waitpid),
        "creat" => Ok(LinuxSyscall::SYS_creat),
        "link" => Ok(LinuxSyscall::SYS_link),
        "unlink" => Ok(LinuxSyscall::SYS_unlink),
        "execve" => Ok(LinuxSyscall::SYS_execve),
        "chdir" => Ok(LinuxSyscall::SYS_chdir),
        "time" => Ok(LinuxSyscall::SYS_time),
        "mknod" => Ok(LinuxSyscall::SYS_mknod),
        "chmod" => Ok(LinuxSyscall::SYS_chmod),
        "lchown" => Ok(LinuxSyscall::SYS_lchown),
        "break" => Ok(LinuxSyscall::SYS_break),
        "oldstat" => Ok(LinuxSyscall::SYS_oldstat),
        "lseek" => Ok(LinuxSyscall::SYS_lseek),
        "getpid" => Ok(LinuxSyscall::SYS_getpid),
        "mount" => Ok(LinuxSyscall::SYS_mount),
        "umount" => Ok(LinuxSyscall::SYS_umount),
        "setuid" => Ok(LinuxSyscall::SYS_setuid),
        "getuid" => Ok(LinuxSyscall::SYS_getuid),
        "stime" => Ok(LinuxSyscall::SYS_stime),
        "ptrace" => Ok(LinuxSyscall::SYS_ptrace),
        "alarm" => Ok(LinuxSyscall::SYS_alarm),
        "oldfstat" => Ok(LinuxSyscall::SYS_oldfstat),
        "pause" => Ok(LinuxSyscall::SYS_pause),
        "utime" => Ok(LinuxSyscall::SYS_utime),
        "stty" => Ok(LinuxSyscall::SYS_stty),
        "gtty" => Ok(LinuxSyscall::SYS_gtty),
        "access" => Ok(LinuxSyscall::SYS_access),
        "nice" => Ok(LinuxSyscall::SYS_nice),
        "ftime" => Ok(LinuxSyscall::SYS_ftime),
        "sync" => Ok(LinuxSyscall::SYS_sync),
        "kill" => Ok(LinuxSyscall::SYS_kill),
        "rename" => Ok(LinuxSyscall::SYS_rename),
        "mkdir" => Ok(LinuxSyscall::SYS_mkdir),
        "rmdir" => Ok(LinuxSyscall::SYS_rmdir),
        "dup" => Ok(LinuxSyscall::SYS_dup),
        "pipe" => Ok(LinuxSyscall::SYS_pipe),
        "times" => Ok(LinuxSyscall::SYS_times),
        "prof" => Ok(LinuxSyscall::SYS_prof),
        "brk" => Ok(LinuxSyscall::SYS_brk),
        "setgid" => Ok(LinuxSyscall::SYS_setgid),
        "getgid" => Ok(LinuxSyscall::SYS_getgid),
        "signal" => Ok(LinuxSyscall::SYS_signal),
        "geteuid" => Ok(LinuxSyscall::SYS_geteuid),
        "getegid" => Ok(LinuxSyscall::SYS_getegid),
        "acct" => Ok(LinuxSyscall::SYS_acct),
        "umount2" => Ok(LinuxSyscall::SYS_umount2),
        "lock" => Ok(LinuxSyscall::SYS_lock),
        "ioctl" => Ok(LinuxSyscall::SYS_ioctl),
        "fcntl" => Ok(LinuxSyscall::SYS_fcntl),
        "mpx" => Ok(LinuxSyscall::SYS_mpx),
        "setpgid" => Ok(LinuxSyscall::SYS_setpgid),
        "ulimit" => Ok(LinuxSyscall::SYS_ulimit),
        "oldolduname" => Ok(LinuxSyscall::SYS_oldolduname),
        "umask" => Ok(LinuxSyscall::SYS_umask),
        "chroot" => Ok(LinuxSyscall::SYS_chroot),
        "ustat" => Ok(LinuxSyscall::SYS_ustat),
        "dup2" => Ok(LinuxSyscall::SYS_dup2),
        "getppid" => Ok(LinuxSyscall::SYS_getppid),
        "getpgrp" => Ok(LinuxSyscall::SYS_getpgrp),
        "setsid" => Ok(LinuxSyscall::SYS_setsid),
        "sigaction" => Ok(LinuxSyscall::SYS_sigaction),
        "sgetmask" => Ok(LinuxSyscall::SYS_sgetmask),
        "ssetmask" => Ok(LinuxSyscall::SYS_ssetmask),
        "setreuid" => Ok(LinuxSyscall::SYS_setreuid),
        "setregid" => Ok(LinuxSyscall::SYS_setregid),
        "sigsuspend" => Ok(LinuxSyscall::SYS_sigsuspend),
        "sigpending" => Ok(LinuxSyscall::SYS_sigpending),
        "sethostname" => Ok(LinuxSyscall::SYS_sethostname),
        "setrlimit" => Ok(LinuxSyscall::SYS_setrlimit),
        "getrlimit" => Ok(LinuxSyscall::SYS_getrlimit),
        "getrusage" => Ok(LinuxSyscall::SYS_getrusage),
        "gettimeofday" => Ok(LinuxSyscall::SYS_gettimeofday),
        "settimeofday" => Ok(LinuxSyscall::SYS_settimeofday),
        "getgroups" => Ok(LinuxSyscall::SYS_getgroups),
        "setgroups" => Ok(LinuxSyscall::SYS_setgroups),
        "select" => Ok(LinuxSyscall::SYS_select),
        "symlink" => Ok(LinuxSyscall::SYS_symlink),
        "oldlstat" => Ok(LinuxSyscall::SYS_oldlstat),
        "readlink" => Ok(LinuxSyscall::SYS_readlink),
        "uselib" => Ok(LinuxSyscall::SYS_uselib),
        "swapon" => Ok(LinuxSyscall::SYS_swapon),
        "reboot" => Ok(LinuxSyscall::SYS_reboot),
        "readdir" => Ok(LinuxSyscall::SYS_readdir),
        "mmap" => Ok(LinuxSyscall::SYS_mmap),
        "munmap" => Ok(LinuxSyscall::SYS_munmap),
        "truncate" => Ok(LinuxSyscall::SYS_truncate),
        "ftruncate" => Ok(LinuxSyscall::SYS_ftruncate),
        "fchmod" => Ok(LinuxSyscall::SYS_fchmod),
        "fchown" => Ok(LinuxSyscall::SYS_fchown),
        "getpriority" => Ok(LinuxSyscall::SYS_getpriority),
        "setpriority" => Ok(LinuxSyscall::SYS_setpriority),
        "profil" => Ok(LinuxSyscall::SYS_profil),
        "statfs" => Ok(LinuxSyscall::SYS_statfs),
        "fstatfs" => Ok(LinuxSyscall::SYS_fstatfs),
        "ioperm" => Ok(LinuxSyscall::SYS_ioperm),
        "socketcall" => Ok(LinuxSyscall::SYS_socketcall),
        "syslog" => Ok(LinuxSyscall::SYS_syslog),
        "setitimer" => Ok(LinuxSyscall::SYS_setitimer),
        "getitimer" => Ok(LinuxSyscall::SYS_getitimer),
        "stat" => Ok(LinuxSyscall::SYS_stat),
        "lstat" => Ok(LinuxSyscall::SYS_lstat),
        "fstat" => Ok(LinuxSyscall::SYS_fstat),
        "olduname" => Ok(LinuxSyscall::SYS_olduname),
        "iopl" => Ok(LinuxSyscall::SYS_iopl),
        "vhangup" => Ok(LinuxSyscall::SYS_vhangup),
        "idle" => Ok(LinuxSyscall::SYS_idle),
        "vm86old" => Ok(LinuxSyscall::SYS_vm86old),
        "wait4" => Ok(LinuxSyscall::SYS_wait4),
        "swapoff" => Ok(LinuxSyscall::SYS_swapoff),
        "sysinfo" => Ok(LinuxSyscall::SYS_sysinfo),
        "ipc" => Ok(LinuxSyscall::SYS_ipc),
        "fsync" => Ok(LinuxSyscall::SYS_fsync),
        "sigreturn" => Ok(LinuxSyscall::SYS_sigreturn),
        "clone" => Ok(LinuxSyscall::SYS_clone),
        "setdomainname" => Ok(LinuxSyscall::SYS_setdomainname),
        "uname" => Ok(LinuxSyscall::SYS_uname),
        "modify_ldt" => Ok(LinuxSyscall::SYS_modify_ldt),
        "adjtimex" => Ok(LinuxSyscall::SYS_adjtimex),
        "mprotect" => Ok(LinuxSyscall::SYS_mprotect),
        "sigprocmask" => Ok(LinuxSyscall::SYS_sigprocmask),
        "create_module" => Ok(LinuxSyscall::SYS_create_module),
        "init_module" => Ok(LinuxSyscall::SYS_init_module),
        "delete_module" => Ok(LinuxSyscall::SYS_delete_module),
        "get_kernel_syms" => Ok(LinuxSyscall::SYS_get_kernel_syms),
        "quotactl" => Ok(LinuxSyscall::SYS_quotactl),
        "getpgid" => Ok(LinuxSyscall::SYS_getpgid),
        "fchdir" => Ok(LinuxSyscall::SYS_fchdir),
        "bdflush" => Ok(LinuxSyscall::SYS_bdflush),
        "sysfs" => Ok(LinuxSyscall::SYS_sysfs),
        "personality" => Ok(LinuxSyscall::SYS_personality),
        "afs_syscall" => Ok(LinuxSyscall::SYS_afs_syscall),
        "setfsuid" => Ok(LinuxSyscall::SYS_setfsuid),
        "setfsgid" => Ok(LinuxSyscall::SYS_setfsgid),
        "_llseek" => Ok(LinuxSyscall::SYS__llseek),
        "getdents" => Ok(LinuxSyscall::SYS_getdents),
        "_newselect" => Ok(LinuxSyscall::SYS__newselect),
        "flock" => Ok(LinuxSyscall::SYS_flock),
        "msync" => Ok(LinuxSyscall::SYS_msync),
        "readv" => Ok(LinuxSyscall::SYS_readv),
        "writev" => Ok(LinuxSyscall::SYS_writev),
        "getsid" => Ok(LinuxSyscall::SYS_getsid),
        "fdatasync" => Ok(LinuxSyscall::SYS_fdatasync),
        "_sysctl" => Ok(LinuxSyscall::SYS__sysctl),
        "mlock" => Ok(LinuxSyscall::SYS_mlock),
        "munlock" => Ok(LinuxSyscall::SYS_munlock),
        "mlockall" => Ok(LinuxSyscall::SYS_mlockall),
        "munlockall" => Ok(LinuxSyscall::SYS_munlockall),
        "sched_setparam" => Ok(LinuxSyscall::SYS_sched_setparam),
        "sched_getparam" => Ok(LinuxSyscall::SYS_sched_getparam),
        "sched_setscheduler" => Ok(LinuxSyscall::SYS_sched_setscheduler),
        "sched_getscheduler" => Ok(LinuxSyscall::SYS_sched_getscheduler),
        "sched_yield" => Ok(LinuxSyscall::SYS_sched_yield),
        "sched_get_priority_max" => Ok(LinuxSyscall::SYS_sched_get_priority_max),
        "sched_get_priority_min" => Ok(LinuxSyscall::SYS_sched_get_priority_min),
        "sched_rr_get_interval" => Ok(LinuxSyscall::SYS_sched_rr_get_interval),
        "nanosleep" => Ok(LinuxSyscall::SYS_nanosleep),
        "mremap" => Ok(LinuxSyscall::SYS_mremap),
        "setresuid" => Ok(LinuxSyscall::SYS_setresuid),
        "getresuid" => Ok(LinuxSyscall::SYS_getresuid),
        "vm86" => Ok(LinuxSyscall::SYS_vm86),
        "query_module" => Ok(LinuxSyscall::SYS_query_module),
        "poll" => Ok(LinuxSyscall::SYS_poll),
        "nfsservctl" => Ok(LinuxSyscall::SYS_nfsservctl),
        "setresgid" => Ok(LinuxSyscall::SYS_setresgid),
        "getresgid" => Ok(LinuxSyscall::SYS_getresgid),
        "prctl" => Ok(LinuxSyscall::SYS_prctl),
        "rt_sigreturn" => Ok(LinuxSyscall::SYS_rt_sigreturn),
        "rt_sigaction" => Ok(LinuxSyscall::SYS_rt_sigaction),
        "rt_sigprocmask" => Ok(LinuxSyscall::SYS_rt_sigprocmask),
        "rt_sigpending" => Ok(LinuxSyscall::SYS_rt_sigpending),
        "rt_sigtimedwait" => Ok(LinuxSyscall::SYS_rt_sigtimedwait),
        "rt_sigqueueinfo" => Ok(LinuxSyscall::SYS_rt_sigqueueinfo),
        "rt_sigsuspend" => Ok(LinuxSyscall::SYS_rt_sigsuspend),
        "pread64" => Ok(LinuxSyscall::SYS_pread64),
        "pwrite64" => Ok(LinuxSyscall::SYS_pwrite64),
        "chown" => Ok(LinuxSyscall::SYS_chown),
        "getcwd" => Ok(LinuxSyscall::SYS_getcwd),
        "capget" => Ok(LinuxSyscall::SYS_capget),
        "capset" => Ok(LinuxSyscall::SYS_capset),
        "sigaltstack" => Ok(LinuxSyscall::SYS_sigaltstack),
        "sendfile" => Ok(LinuxSyscall::SYS_sendfile),
        "getpmsg" => Ok(LinuxSyscall::SYS_getpmsg),
        "putpmsg" => Ok(LinuxSyscall::SYS_putpmsg),
        "vfork" => Ok(LinuxSyscall::SYS_vfork),
        "ugetrlimit" => Ok(LinuxSyscall::SYS_ugetrlimit),
        "mmap2" => Ok(LinuxSyscall::SYS_mmap2),
        "truncate64" => Ok(LinuxSyscall::SYS_truncate64),
        "ftruncate64" => Ok(LinuxSyscall::SYS_ftruncate64),
        "stat64" => Ok(LinuxSyscall::SYS_stat64),
        "lstat64" => Ok(LinuxSyscall::SYS_lstat64),
        "fstat64" => Ok(LinuxSyscall::SYS_fstat64),
        "lchown32" => Ok(LinuxSyscall::SYS_lchown32),
        "getuid32" => Ok(LinuxSyscall::SYS_getuid32),
        "getgid32" => Ok(LinuxSyscall::SYS_getgid32),
        "geteuid32" => Ok(LinuxSyscall::SYS_geteuid32),
        "getegid32" => Ok(LinuxSyscall::SYS_getegid32),
        "setreuid32" => Ok(LinuxSyscall::SYS_setreuid32),
        "setregid32" => Ok(LinuxSyscall::SYS_setregid32),
        "getgroups32" => Ok(LinuxSyscall::SYS_getgroups32),
        "setgroups32" => Ok(LinuxSyscall::SYS_setgroups32),
        "fchown32" => Ok(LinuxSyscall::SYS_fchown32),
        "setresuid32" => Ok(LinuxSyscall::SYS_setresuid32),
        "getresuid32" => Ok(LinuxSyscall::SYS_getresuid32),
        "setresgid32" => Ok(LinuxSyscall::SYS_setresgid32),
        "getresgid32" => Ok(LinuxSyscall::SYS_getresgid32),
        "chown32" => Ok(LinuxSyscall::SYS_chown32),
        "setuid32" => Ok(LinuxSyscall::SYS_setuid32),
        "setgid32" => Ok(LinuxSyscall::SYS_setgid32),
        "setfsuid32" => Ok(LinuxSyscall::SYS_setfsuid32),
        "setfsgid32" => Ok(LinuxSyscall::SYS_setfsgid32),
        "pivot_root" => Ok(LinuxSyscall::SYS_pivot_root),
        "mincore" => Ok(LinuxSyscall::SYS_mincore),
        "madvise" => Ok(LinuxSyscall::SYS_madvise),
        "getdents64" => Ok(LinuxSyscall::SYS_getdents64),
        "fcntl64" => Ok(LinuxSyscall::SYS_fcntl64),
        "gettid" => Ok(LinuxSyscall::SYS_gettid),
        "readahead" => Ok(LinuxSyscall::SYS_readahead),
        "setxattr" => Ok(LinuxSyscall::SYS_setxattr),
        "lsetxattr" => Ok(LinuxSyscall::SYS_lsetxattr),
        "fsetxattr" => Ok(LinuxSyscall::SYS_fsetxattr),
        "getxattr" => Ok(LinuxSyscall::SYS_getxattr),
        "lgetxattr" => Ok(LinuxSyscall::SYS_lgetxattr),
        "fgetxattr" => Ok(LinuxSyscall::SYS_fgetxattr),
        "listxattr" => Ok(LinuxSyscall::SYS_listxattr),
        "llistxattr" => Ok(LinuxSyscall::SYS_llistxattr),
        "flistxattr" => Ok(LinuxSyscall::SYS_flistxattr),
        "removexattr" => Ok(LinuxSyscall::SYS_removexattr),
        "lremovexattr" => Ok(LinuxSyscall::SYS_lremovexattr),
        "fremovexattr" => Ok(LinuxSyscall::SYS_fremovexattr),
        "tkill" => Ok(LinuxSyscall::SYS_tkill),
        "sendfile64" => Ok(LinuxSyscall::SYS_sendfile64),
        "futex" => Ok(LinuxSyscall::SYS_futex),
        "sched_setaffinity" => Ok(LinuxSyscall::SYS_sched_setaffinity),
        "sched_getaffinity" => Ok(LinuxSyscall::SYS_sched_getaffinity),
        "set_thread_area" => Ok(LinuxSyscall::SYS_set_thread_area),
        "get_thread_area" => Ok(LinuxSyscall::SYS_get_thread_area),
        "io_setup" => Ok(LinuxSyscall::SYS_io_setup),
        "io_destroy" => Ok(LinuxSyscall::SYS_io_destroy),
        "io_getevents" => Ok(LinuxSyscall::SYS_io_getevents),
        "io_submit" => Ok(LinuxSyscall::SYS_io_submit),
        "io_cancel" => Ok(LinuxSyscall::SYS_io_cancel),
        "fadvise64" => Ok(LinuxSyscall::SYS_fadvise64),
        "exit_group" => Ok(LinuxSyscall::SYS_exit_group),
        "lookup_dcookie" => Ok(LinuxSyscall::SYS_lookup_dcookie),
        "epoll_create" => Ok(LinuxSyscall::SYS_epoll_create),
        "epoll_ctl" => Ok(LinuxSyscall::SYS_epoll_ctl),
        "epoll_wait" => Ok(LinuxSyscall::SYS_epoll_wait),
        "remap_file_pages" => Ok(LinuxSyscall::SYS_remap_file_pages),
        "set_tid_address" => Ok(LinuxSyscall::SYS_set_tid_address),
        "timer_create" => Ok(LinuxSyscall::SYS_timer_create),
        "timer_settime" => Ok(LinuxSyscall::SYS_timer_settime),
        "timer_gettime" => Ok(LinuxSyscall::SYS_timer_gettime),
        "timer_getoverrun" => Ok(LinuxSyscall::SYS_timer_getoverrun),
        "timer_delete" => Ok(LinuxSyscall::SYS_timer_delete),
        "clock_settime" => Ok(LinuxSyscall::SYS_clock_settime),
        "clock_gettime" => Ok(LinuxSyscall::SYS_clock_gettime),
        "clock_getres" => Ok(LinuxSyscall::SYS_clock_getres),
        "clock_nanosleep" => Ok(LinuxSyscall::SYS_clock_nanosleep),
        "statfs64" => Ok(LinuxSyscall::SYS_statfs64),
        "fstatfs64" => Ok(LinuxSyscall::SYS_fstatfs64),
        "tgkill" => Ok(LinuxSyscall::SYS_tgkill),
        "utimes" => Ok(LinuxSyscall::SYS_utimes),
        "fadvise64_64" => Ok(LinuxSyscall::SYS_fadvise64_64),
        "vserver" => Ok(LinuxSyscall::SYS_vserver),
        "mbind" => Ok(LinuxSyscall::SYS_mbind),
        "get_mempolicy" => Ok(LinuxSyscall::SYS_get_mempolicy),
        "set_mempolicy" => Ok(LinuxSyscall::SYS_set_mempolicy),
        "mq_open" => Ok(LinuxSyscall::SYS_mq_open),
        "mq_unlink" => Ok(LinuxSyscall::SYS_mq_unlink),
        "mq_timedsend" => Ok(LinuxSyscall::SYS_mq_timedsend),
        "mq_timedreceive" => Ok(LinuxSyscall::SYS_mq_timedreceive),
        "mq_notify" => Ok(LinuxSyscall::SYS_mq_notify),
        "mq_getsetattr" => Ok(LinuxSyscall::SYS_mq_getsetattr),
        "kexec_load" => Ok(LinuxSyscall::SYS_kexec_load),
        "waitid" => Ok(LinuxSyscall::SYS_waitid),
        "sys_setaltroot" => Ok(LinuxSyscall::SYS_sys_setaltroot),
        "add_key" => Ok(LinuxSyscall::SYS_add_key),
        "request_key" => Ok(LinuxSyscall::SYS_request_key),
        "keyctl" => Ok(LinuxSyscall::SYS_keyctl),
        "ioprio_set" => Ok(LinuxSyscall::SYS_ioprio_set),
        "ioprio_get" => Ok(LinuxSyscall::SYS_ioprio_get),
        "inotify_init" => Ok(LinuxSyscall::SYS_inotify_init),
        "inotify_add_watch" => Ok(LinuxSyscall::SYS_inotify_add_watch),
        "inotify_rm_watch" => Ok(LinuxSyscall::SYS_inotify_rm_watch),
        "migrate_pages" => Ok(LinuxSyscall::SYS_migrate_pages),
        "openat" => Ok(LinuxSyscall::SYS_openat),
        "mkdirat" => Ok(LinuxSyscall::SYS_mkdirat),
        "mknodat" => Ok(LinuxSyscall::SYS_mknodat),
        "fchownat" => Ok(LinuxSyscall::SYS_fchownat),
        "futimesat" => Ok(LinuxSyscall::SYS_futimesat),
        "fstatat64" => Ok(LinuxSyscall::SYS_fstatat64),
        "unlinkat" => Ok(LinuxSyscall::SYS_unlinkat),
        "renameat" => Ok(LinuxSyscall::SYS_renameat),
        "linkat" => Ok(LinuxSyscall::SYS_linkat),
        "symlinkat" => Ok(LinuxSyscall::SYS_symlinkat),
        "readlinkat" => Ok(LinuxSyscall::SYS_readlinkat),
        "fchmodat" => Ok(LinuxSyscall::SYS_fchmodat),
        "faccessat" => Ok(LinuxSyscall::SYS_faccessat),
        "pselect6" => Ok(LinuxSyscall::SYS_pselect6),
        "ppoll" => Ok(LinuxSyscall::SYS_ppoll),
        "unshare" => Ok(LinuxSyscall::SYS_unshare),
        "set_robust_list" => Ok(LinuxSyscall::SYS_set_robust_list),
        "get_robust_list" => Ok(LinuxSyscall::SYS_get_robust_list),
        "splice" => Ok(LinuxSyscall::SYS_splice),
        "sync_file_range" => Ok(LinuxSyscall::SYS_sync_file_range),
        "tee" => Ok(LinuxSyscall::SYS_tee),
        "vmsplice" => Ok(LinuxSyscall::SYS_vmsplice),
        "move_pages" => Ok(LinuxSyscall::SYS_move_pages),
        "getcpu" => Ok(LinuxSyscall::SYS_getcpu),
        "epoll_pwait" => Ok(LinuxSyscall::SYS_epoll_pwait),
        "utimensat" => Ok(LinuxSyscall::SYS_utimensat),
        "signalfd" => Ok(LinuxSyscall::SYS_signalfd),
        "timerfd_create" => Ok(LinuxSyscall::SYS_timerfd_create),
        "eventfd" => Ok(LinuxSyscall::SYS_eventfd),
        "fallocate" => Ok(LinuxSyscall::SYS_fallocate),
        "timerfd_settime" => Ok(LinuxSyscall::SYS_timerfd_settime),
        "timerfd_gettime" => Ok(LinuxSyscall::SYS_timerfd_gettime),
        "signalfd4" => Ok(LinuxSyscall::SYS_signalfd4),
        "eventfd2" => Ok(LinuxSyscall::SYS_eventfd2),
        "epoll_create1" => Ok(LinuxSyscall::SYS_epoll_create1),
        "dup3" => Ok(LinuxSyscall::SYS_dup3),
        "pipe2" => Ok(LinuxSyscall::SYS_pipe2),
        "inotify_init1" => Ok(LinuxSyscall::SYS_inotify_init1),
        "preadv" => Ok(LinuxSyscall::SYS_preadv),
        "pwritev" => Ok(LinuxSyscall::SYS_pwritev),
        "rt_tgsigqueueinfo" => Ok(LinuxSyscall::SYS_rt_tgsigqueueinfo),
        "perf_event_open" => Ok(LinuxSyscall::SYS_perf_event_open),
        "recvmmsg" => Ok(LinuxSyscall::SYS_recvmmsg),
        "fanotify_init" => Ok(LinuxSyscall::SYS_fanotify_init),
        "fanotify_mark" => Ok(LinuxSyscall::SYS_fanotify_mark),
        "prlimit64" => Ok(LinuxSyscall::SYS_prlimit64),
        "name_to_handle_at" => Ok(LinuxSyscall::SYS_name_to_handle_at),
        "open_by_handle_at" => Ok(LinuxSyscall::SYS_open_by_handle_at),
        "clock_adjtime" => Ok(LinuxSyscall::SYS_clock_adjtime),
        "syncfs" => Ok(LinuxSyscall::SYS_syncfs),
        "sendmmsg" => Ok(LinuxSyscall::SYS_sendmmsg),
        "setns" => Ok(LinuxSyscall::SYS_setns),
        "process_vm_readv" => Ok(LinuxSyscall::SYS_process_vm_readv),
        "process_vm_writev" => Ok(LinuxSyscall::SYS_process_vm_writev),
        "kcmp" => Ok(LinuxSyscall::SYS_kcmp),
        "finit_module" => Ok(LinuxSyscall::SYS_finit_module),
        "sched_setattr" => Ok(LinuxSyscall::SYS_sched_setattr),
        "sched_getattr" => Ok(LinuxSyscall::SYS_sched_getattr),
        "renameat2" => Ok(LinuxSyscall::SYS_renameat2),
        "seccomp" => Ok(LinuxSyscall::SYS_seccomp),
        "getrandom" => Ok(LinuxSyscall::SYS_getrandom),
        "memfd_create" => Ok(LinuxSyscall::SYS_memfd_create),
        "bpf" => Ok(LinuxSyscall::SYS_bpf),
        "execveat" => Ok(LinuxSyscall::SYS_execveat),
        "socket" => Ok(LinuxSyscall::SYS_socket),
        "socketpair" => Ok(LinuxSyscall::SYS_socketpair),
        "bind" => Ok(LinuxSyscall::SYS_bind),
        "connect" => Ok(LinuxSyscall::SYS_connect),
        "listen" => Ok(LinuxSyscall::SYS_listen),
        "accept4" => Ok(LinuxSyscall::SYS_accept4),
        "getsockopt" => Ok(LinuxSyscall::SYS_getsockopt),
        "setsockopt" => Ok(LinuxSyscall::SYS_setsockopt),
        "getsockname" => Ok(LinuxSyscall::SYS_getsockname),
        "getpeername" => Ok(LinuxSyscall::SYS_getpeername),
        "sendto" => Ok(LinuxSyscall::SYS_sendto),
        "sendmsg" => Ok(LinuxSyscall::SYS_sendmsg),
        "recvfrom" => Ok(LinuxSyscall::SYS_recvfrom),
        "recvmsg" => Ok(LinuxSyscall::SYS_recvmsg),
        "shutdown" => Ok(LinuxSyscall::SYS_shutdown),
        "userfaultfd" => Ok(LinuxSyscall::SYS_userfaultfd),
        "membarrier" => Ok(LinuxSyscall::SYS_membarrier),
        "mlock2" => Ok(LinuxSyscall::SYS_mlock2),
        _ => Err(super::Error::UnknownSyscall),
    }
}