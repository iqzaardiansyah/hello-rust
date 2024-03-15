<details>
<summary>Tutorial Module 6</summary>

# Commit 1 Reflection notes
In the `handle_connection` function, there is a new `BufReader` instance that wraps a mutable reference to the stream. `BufReader` adds buffering by managing calls to the `std::io::Read` trait method.

There is also a variable called `http_request` to collect the request lines that the browser sends to the server. We indicate that we want to collect these lines in a vector by adding an annotation of type `Vec<_>`.

`BufReader` implements the `std::io::BufRead` trait, which provides a line method. The line method returns an iterator `Result<String, std::io::Error>` by splitting the data stream every time it sees a new line byte. To get each String, we map and unwrap each Result. The result may be an error if the data is not valid UTF-8 or if there was a problem reading from the stream.

# Commit 2 Screen Capture
![Commit 2 screen capture](/assets/images/commit2ss.png)

# Commit 3 Reflection Notes
![Commit 3 screen capture](/assets/images/commit3ss.png)
I followed the method used in the rust book, namely by separating the responses using `if-else` which reads the `request_line`. If the read `request_line` is `GET / HTTP/1.1` then the code will respond back with the correct template and vice versa if the read `request_line` is wrong.

Refactoring is very necessary because it has many benefits, including code that is easier to read and if the code is easy to read then the code will be easier to modify and if the code is easy to modify then the code will have high maintainability. And as we know large companies want faster and more efficient code and refactoring can help with that. Developers may waste too much time trying to find bugs and errors and updating every line of code if the old code is clumsy and voluminous. For quick and effective work with the code, code refactoring is a need. Spend money on code reworking now to save a ton of time and money later.

# Commit 4 Reflection Notes
When entering `/` URI, the application will respond quickly. However, when the `/sleep` URI is entered and then the `/` URI is entered in a separate browser window, the application will wait until the `/sleep` loading process is complete and then begin processing the `/` which in this case is a 5 second delay. In other words, users who access `/` must wait for the loading process from other users who access `/sleep`. This case is of course a big drawback because no user wants to wait 5 seconds to access something. Therefore, this problem will be solved at the next stage, namely by using `ThreadPool`. `ThreadPool` provides many threads that are ready to handle many accesses from users simultaneously which is expected to help deal with the problems faced.

# Commit 5 Reflection Notes
A thread pool is a group of threads that are spawned and waiting and ready to handle a task. When the program receives a new task, it will assign one of the threads in the pool to that task, and that thread will process the task. The remaining threads in the pool are available to handle other incoming tasks while the first thread is processing. When the first thread finishes processing its task, it returns to the pool of idle threads, ready to handle new tasks. Thread pools allow servers to process connections simultaneously, thereby increasing server throughput. Threads generated from the ThreadPool must be limited because if they are not limited, the application will be vulnerable to Denial of Service (DoS) attacks.
</details>