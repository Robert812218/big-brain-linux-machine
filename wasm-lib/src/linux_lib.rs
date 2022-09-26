use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn () -> {
    pub fn system_based_commands() {
        // 1. System Based Commands
        // uname 	 Displays  Linux system information
        let : ("", vec![]),
        // uname -r	Displays  kernel release information
        let : ("", vec![]),
        // uptime	Displays how long the system has been running including load average
        let : ("", vec![]),
        // hostname	Shows the system hostname
        let : ("", vec![]),
        // hostname -i	Displays the IP address of the system
        let : ("", vec![]),
        // last reboot	Shows system reboot history
        let : ("", vec![]),
        // date	Displays current system date and time
        let : ("", vec![]),
        // timedatectl	Query and change the System clock
        let : ("", vec![]),
        // cal	Displays the current calendar month and day
        let : ("", vec![]),
        // w	Displays currently  logged in users in the system
        let : ("", vec![]),
        // whoami	Displays who you are logged in as
        let : ("", vec![]),
        // finger username	Displays information about the user
        let : ("", vec![]),
    }

    pub fn hardware_based_components() {
        // 2. Hardware Based Commands
        // dmesg	Displays bootup messages
        let : ("", vec![]),
        // cat /proc/cpuinfo	Displays more information about CPU e.g model, model name, cores, vendor id
        let : ("", vec![]),
        // cat /proc/meminfo	Displays more information about hardware memory e.g. Total and Free memory
        let : ("", vec![]),
        // lshw	Displays information about system's hardware configuration
        let : ("", vec![]),
        // lsblk	Displays block devices related information
        let : ("", vec![]),
        // free -m	Displays free and used memory in the system (-m flag indicates memory in MB)
        let : ("", vec![]),
        // lspci -tv	Displays PCI devices in a tree-like diagram
        let : ("", vec![]),
        // lsusb -tv	Displays USB devices in a tree-like diagram
        let : ("", vec![]),
        // dmidecode	Displays hardware information from the BIOS
        let : ("", vec![]),
        // hdparm -i /dev/xda	Displays information about disk data
        let : ("", vec![]),
        // hdparm -tT /dev/xda <:code>	Conducts a read speed test on device xda
        let : ("", vec![]),
        // badblocks -s /dev/xda	Tests  for unreadable blocks on disk
        let : ("", vec![]),
    }
    
    pub fn users_management_commands() {
        // 3. Users Management Commands
        // id	Displays the details of the active user e.g. uid, gid, and groups
        let : ("", vec![]),
        // last	Shows the last logins in the system
        let : ("", vec![]),
        // who	Shows who is logged in to the system
        let : ("", vec![]),
        // groupadd "admin" 	Adds the group 'admin'
        let : ("", vec![]),
        // adduser "Sam"	Adds user Sam
        let : ("", vec![]),
        // userdel "Sam"	Deletes user Sam
        let : ("", vec![]),
        // usermod	Used for changing / modifying user information
        let : ("", vec![]),
    }

    pub fn file_commands() {
        // 4. File Commands
        // ls -al	Lists files - both regular &  hidden files and their permissions as well.
        let : ("", vec![]),
        // pwd	Displays the current directory file path
        let : ("", vec![]),
        // mkdir 'directory_name'	Creates a new directory
        let : ("", vec![]),
        // rm file_name 	Removes a file
        let : ("", vec![]),
        // rm -f filename	Forcefully removes a file
        let : ("", vec![]),
        // rm -r directory_name	Removes a directory recursively
        let : ("", vec![]),
        // rm -rf directory_name	Removes a directory forcefully and recursively
        let : ("", vec![]),
        // cp file1 file2	Copies the contents of file1 to file2
        let : ("", vec![]),
        // cp -r dir1 dir2	Recursively Copies dir1 to dir2. dir2 is created if it does not exist
        let : ("", vec![]),
        // mv file1 file2	Renames file1 to file2
        let : ("", vec![]),
        // ln -s /path/to/file_name   link_name	Creates a symbolic link to file_name
        let : ("", vec![]),
        // touch file_name	Creates a new file
        let : ("", vec![]),
        // cat > file_name	Places standard input into a file
        let : ("", vec![]),
        // more file_name	Outputs the contents of a file
        let : ("", vec![]),
        // head file_name	Displays the first 10 lines of a file
        let : ("", vec![]),
        // tail file_name	Displays the last 10 lines of a file
        let : ("", vec![]),
        // gpg -c file_name	Encrypts a file
        let : ("", vec![]),
        // gpg file_name.gpg	Decrypts a file
        let : ("", vec![]),
        // wc	Prints the number of bytes, words and lines in a file
        let : ("", vec![]),
        // xargs	Executes commands from standard input
        let : ("", vec![]),
    }
}

pub fn process_related_commands() {
        // 5. Process Related Commands
        let : ("", vec![]),
        // ps	Display currently active processes
        let : ("", vec![]),
        // ps aux | grep 'telnet'	Searches for the id of the process 'telnet'
        let : ("", vec![]),
        // pmap	Displays memory map of processes
        let : ("", vec![]),
        // top	 Displays all running processes
        let : ("", vec![]),
        // kill pid	Terminates process with a given pid
        let : ("", vec![]),
        // killall proc	Kills / Terminates all processes named proc
        let : ("", vec![]),
        // pkill process-name	Sends a signal to a process with its name
        let : ("", vec![]),
        // bg	Resumes suspended jobs in the background
        let : ("", vec![]),
        // fg	Brings suspended jobs to the foreground
        let : ("", vec![]),
        // fg n	job n to the foreground
        let : ("", vec![]),
        // lsof	Lists files that are open by processes
        let : ("", vec![]),
        // renice 19 PID	makes a process run with very low priority
        let : ("", vec![]),
        // pgrep firefox	find Firefox process ID
        let : ("", vec![]),
        // pstree	visualizing processes in tree model
        let : ("", vec![]),
}

pub fn file_permission_commands() {
        // 6. File Permission Commands
        // chmod octal filename         	Change file permissions of the file to octal
        // 
        // Example
        // chmod 777 /data/test.c       	Set rwx permissions to owner, group and everyone (everyone else who has access to the server)
        // chmod 755 /data/test.c       	Set rwx to the owner and r_x to group and everyone
        // chmod 766 /data/test.c       	Sets rwx for owner, rw for group and everyone
        // chown owner user-file         	Change ownership of the file
        // chown owner-user:owner-group file_name       	Change owner and group owner of the file
        // chown owner-user:owner-group directory  	Change owner and group owner of the directory
}

pub fn network_commands() {
        // 7. Network Commands
        // ip addr show                   	Displays IP addresses and all the network interfaces
        // ip address add 192.168.0.1/24 dev eth0    	Assigns IP address 192.168.0.1 to interface eth0
        // ifconfig                             	Displays IP addresses of all network interfaces
        // ping  host                       	ping command sends an ICMP echo request to establish a connection to server / PC
        // whois domain                  	Retrieves more information about a domain name
        // dig domain                       	Retrieves DNS information about the domain
        // dig -x host                    	Performs reverse lookup on a domain
        // host google.com          	Performs an IP lookup for the domain name
        // hostname -i                     	Displays local IP address
        // wget file_name             	Downloads a file from an online source
        // netstat -pnltu     	Displays all active listening ports
}

pub fn compression_archives_commands() {
        // 8. Compression/Archives Commands
        // tar -cf home.tar home<:code>	Creates archive file called 'home.tar' from file 'home'
        // tar -xf files.tar              	Extract archive file 'files.tar'
        // tar -zcvf home.tar.gz source-folder    	Creates gzipped tar archive file from the source folder
        // gzip file 	Compression a file with .gz extension
}

pub fn install_packages_commands() {
        // 9. Install Packages Commands
        // rpm -i pkg_name.rpm            	Install an rpm package
        // rpm -e pkg_name                     	Removes an rpm package
        // dnf install pkg_name	Install package using dnf utility
}

pub fn install_source_compilation() {
        // 10. Install Source (Compilation)
        // ./configure	Checks your system for the required software needed to build the program. It will build the Makefile containing the instructions required to effectively build the project
        // make	It reads the Makefile to compile the program with the required operations. The process may take some time, depending on your system and the size of the program
        // make install	The command installs the binaries in the default/modified paths after the compilation
}

pub fn search_commands() {
        // 11. Search Commands
        // grep 'pattern' files                           	Search for a given pattern in files
        // grep -r pattern dir                             	Search recursively for a pattern in a given directory
        // locate file                                            	Find all instances of the file
        // find /home/ -name "index"              	Find file names that begin with 'index' in /home folder
        // find /home -size +10000k	Find files greater than 10000k in the home folder
}

pub fn login_commands() {
        // 12. Login Commands
        // ssh user@host                                        	Securely connect to host as user
        // ssh -p port_number user@host      	Securely connect to host using a specified port
        // ssh host                                                	Securely connect to the system via SSH default port 22
        // telnet host 	Connect to host via telnet default port 23
}

pub fn file_transfer_commands() {
        // 13. File Transfer Commands
        // scp file1.txt server2/tmp           	Securely copy file1.txt to server2 in /tmp directory
        // rsync -a /home/apps  /backup/ 	Synchronize contents in /home/apps directory with /backup  directory
}

pub fn disk_use_commands() {
        // 14. Disk Usage Commands
        // df  -h                           	Displays free space on mounted systems
        // df  -i                          	Displays free inodes on filesystems
        // fdisk  -l                    	Shows disk partitions, sizes, and types
        // du  -sh                        	Displays disk usage in the current directory in a human-readable format
        // findmnt                      	Displays target mount point for all filesystems
        // mount device-path mount-point	Mount a device
}

pub fn directory_traverse_commands() {
        // 15. Directory Traverse Commands
        // cd ..             	Move up one level in the directory tree structure
        // cd                 	Change directory to $HOME directory
        // cd /test 	Change directory to /test directory
}
