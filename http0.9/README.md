## Implementation of HTTP Versions

This repository is made to study networking programming and web security while learning about the http history.

## HTTP/0.9

Was the first version of http, there were no status codes, headers, only html pages.

## References

- HTTP/0.9 - https://http.dev/0.9
- Evolution of HTTP - https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP
- The first http web page - http://info.cern.ch/hypertext/WWW/TheProject.html

## How to test

### Existing file

```bash
$ telnet localhost 3333
Trying ::1...
Connected to localhost.
Escape character is '^]'.
GET /index.html
<html>
  A very simple HTML page
</html>
Connection closed by foreign host.
```

### Not existing file

```bash
$ telnet localhost 3333
Trying ::1...
Connected to localhost.
Escape character is '^]'.
GET /foo.html
<html>
  Not found
</html>
Connection closed by foreign host.
```

## The first version of HTTP/0.9 is vulnerable to [path traversal attacks](https://owasp.org/www-community/attacks/Path_Traversal)

- [Commit](https://github.com/thiago-figueredo/http-history/commit/106e0ef31704ee02647ed729b314b747e022d076)

You can explore the path traversal vulnerability be trying to read important system files like **/etc/passwd**

```bash
$ telnet localhost 3333
Trying ::1...
Connected to localhost.
Escape character is '^]'.
GET /../../../../../../../../../../etc/passwd
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
bin:x:2:2:bin:/bin:/usr/sbin/nologin
sys:x:3:3:sys:/dev:/usr/sbin/nologin
sync:x:4:65534:sync:/bin:/bin/sync
games:x:5:60:games:/usr/games:/usr/sbin/nologin
man:x:6:12:man:/var/cache/man:/usr/sbin/nologin
lp:x:7:7:lp:/var/spool/lpd:/usr/sbin/nologin
mail:x:8:8:mail:/var/mail:/usr/sbin/nologin
news:x:9:9:news:/var/spool/news:/usr/sbin/nologin
uucp:x:10:10:uucp:/var/spool/uucp:/usr/sbin/nologin
proxy:x:13:13:proxy:/bin:/usr/sbin/nologin
www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin
backup:x:34:34:backup:/var/backups:/usr/sbin/nologin
list:x:38:38:Mailing List Manager:/var/list:/usr/sbin/nologin
irc:x:39:39:ircd:/var/run/ircd:/usr/sbin/nologin
gnats:x:41:41:Gnats Bug-Reporting System (admin):/var/lib/gnats:/usr/sbin/nologin
nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin
_apt:x:100:65534::/nonexistent:/usr/sbin/nologin
Connection closed by foreign host.
```

## Path traversal attack solution

```rust
fn handle_data(stream: &mut TcpStream) {
  let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
  let mut buffer = String::new();

  reader.read_line(&mut buffer).unwrap();

  match buffer.split_whitespace().collect::<Vec<&str>>().as_slice() {
    [method, path] if *method == "GET" => {
      let cwd = env::current_dir().unwrap();
      let mut abs_path = format!("{}/public{}", cwd.display(), path);

      while abs_path.contains("../") {
          abs_path = abs_path.replace("../", "")
      }

      Http::try_send_file(stream, &abs_path)
    }

    _ => {
        Http::error(stream)
    }
  }
}

```

```rust
while abs_path.contains("../") {
  abs_path = abs_path.replace("../", "")
}
```

This code is necessary because if we remove ../ only one time the attacker can make this http request to bypass the filter:

```
GET /..././..././..././..././..././..././..././..././..././..././etc/passwd
```

That would be turned into:

```
GET /../../../../../../../../../../etc/passwd
```

with the recursive filter all ../ are remove and the final result is:

```
GET /etc/passwd
```
