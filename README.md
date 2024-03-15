<details>
<summary>Tutorial Module 6</summary>

# Commit 1 Reflection notes
In the `handle_connection` function, there is a new `BufReader` instance that wraps a mutable reference to the stream. `BufReader` adds buffering by managing calls to the `std::io::Read` trait method.

There is also a variable called `http_request` to collect the request lines that the browser sends to the server. We indicate that we want to collect these lines in a vector by adding an annotation of type `Vec<_>`.

`BufReader` implements the `std::io::BufRead` trait, which provides a line method. The line method returns an iterator `Result<String, std::io::Error>` by splitting the data stream every time it sees a new line byte. To get each String, we map and unwrap each Result. The result may be an error if the data is not valid UTF-8 or if there was a problem reading from the stream.

# Commit 2 Screen Capture
![Commit 2 screen capture](/assets/images/commit2ss.png)


</details>