#ifndef _SYS_STAT_H
#define _SYS_STAT_H

typedef unsigned long long int __dev_t;
typedef unsigned long long int __ino_t;
typedef unsigned long long int __mode_t;
typedef __mode_t mode_t;
typedef unsigned long long int __nlink_t;
typedef unsigned long long int __uid_t;
typedef unsigned long long int __gid_t;
typedef unsigned long long int __off_t;
typedef __off_t off_t;
typedef unsigned long long int __blksize_t;
typedef unsigned long long int __blkcnt_t;
typedef unsigned long long int __time_t;

// taken from https://github.com/lattera/glibc/blob/master/sysdeps/unix/sysv/linux/bits/stat.h

struct stat
  {
    __dev_t st_dev;			/* Device.  */
    unsigned short int __pad1;
#ifndef __USE_FILE_OFFSET64
    __ino_t st_ino;			/* File serial number.	*/
#else
    __ino_t __st_ino;			/* 32bit file serial number.	*/
#endif
    __mode_t st_mode;			/* File mode.  */
    __nlink_t st_nlink;			/* Link count.  */
    __uid_t st_uid;			/* User ID of the file's owner.	*/
    __gid_t st_gid;			/* Group ID of the file's group.*/
    __dev_t st_rdev;			/* Device number, if device.  */
    unsigned short int __pad2;
#ifndef __USE_FILE_OFFSET64
    __off_t st_size;			/* Size of file, in bytes.  */
#else
    __off64_t st_size;			/* Size of file, in bytes.  */
#endif
    __blksize_t st_blksize;		/* Optimal block size for I/O.  */

#ifndef __USE_FILE_OFFSET64
    __blkcnt_t st_blocks;		/* Number 512-byte blocks allocated. */
#else
    __blkcnt64_t st_blocks;		/* Number 512-byte blocks allocated. */
#endif
#ifdef __USE_XOPEN2K8
    /* Nanosecond resolution timestamps are stored in a format
       equivalent to 'struct timespec'.  This is the type used
       whenever possible but the Unix namespace rules do not allow the
       identifier 'timespec' to appear in the <sys/stat.h> header.
       Therefore we have to handle the use of this header in strictly
       standard-compliant sources special.  */
    struct timespec st_atim;		/* Time of last access.  */
    struct timespec st_mtim;		/* Time of last modification.  */
    struct timespec st_ctim;		/* Time of last status change.  */
# define st_atime st_atim.tv_sec	/* Backward compatibility.  */
# define st_mtime st_mtim.tv_sec
# define st_ctime st_ctim.tv_sec
#else
    __time_t st_atime;			/* Time of last access.  */
    unsigned long long int st_atimensec;	/* Nscecs of last access.  */
    __time_t st_mtime;			/* Time of last modification.  */
    unsigned long long int st_mtimensec;	/* Nsecs of last modification.  */
    __time_t st_ctime;			/* Time of last status change.  */
    unsigned long long int st_ctimensec;	/* Nsecs of last status change.  */
#endif
#ifndef __USE_FILE_OFFSET64
    unsigned long long int __glibc_reserved4;
    unsigned long long int __glibc_reserved5;
#else
    __ino64_t st_ino;			/* File serial number.	*/
#endif
  };

int stat(const char * pathname, struct stat * statbuf);
int lstat(const char *path, struct stat *buf);
int fstat(int fildes, struct stat *buf);
int open(const char *pathname, int flags, ...);
int open64(const char *pathname, int oflag,...); 
int mkdir(const char *path, mode_t mode);
mode_t umask(mode_t mask);


#define 	S_IFMT   00170000
 
#define 	S_IFSOCK   0140000
 
#define 	S_IFLNK   0120000
 
#define 	S_IFREG   0100000
 
#define 	S_IFBLK   0060000
 
#define 	S_IFDIR   0040000
 
#define 	S_IFCHR   0020000
 
#define 	S_IFIFO   0010000
 
#define 	S_ISUID   0004000
 
#define 	S_ISGID   0002000
 
#define 	S_ISVTX   0001000
 
#define 	S_ISLNK(m)   (((m) & S_IFMT) == S_IFLNK)
 
#define 	S_ISREG(m)   (((m) & S_IFMT) == S_IFREG)
 
#define 	S_ISDIR(m)   (((m) & S_IFMT) == S_IFDIR)
 
#define 	S_ISCHR(m)   (((m) & S_IFMT) == S_IFCHR)
 
#define 	S_ISBLK(m)   (((m) & S_IFMT) == S_IFBLK)
 
#define 	S_ISFIFO(m)   (((m) & S_IFMT) == S_IFIFO)
 
#define 	S_ISSOCK(m)   (((m) & S_IFMT) == S_IFSOCK)
 
#define 	S_IRWXU   00700
 
#define 	S_IRUSR   00400
 
#define 	S_IWUSR   00200
 
#define 	S_IXUSR   00100
 
#define 	S_IRWXG   00070
 
#define 	S_IRGRP   00040
 
#define 	S_IWGRP   00020
 
#define 	S_IXGRP   00010
 
#define 	S_IRWXO   00007
 
#define 	S_IROTH   00004
 
#define 	S_IWOTH   00002
 
#define 	S_IXOTH   00001

#endif