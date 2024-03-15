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
I separate the responses using if-else which reads the request_line. If the read request_line is "GET / HTTP/1.1" then the code will respond back with the correct template and vice versa if the read request_line is wrong.

Refactoring is really needed because it has many benefits, including code that is easier to read and if the code is easy to read then the code will be easier to modify and if the code is easy to modify then the code will have high maintainability.

# Commit 4 Reflection Notes
when entering /URI, the application will respond quickly. But when the /sleep URI is entered and then the / URI is entered in a separate browser window, the application will wait until the /sleep loading process is complete and then start processing /. In other words, the user accessing / must wait for the loading of other users accessing /sleep.

# Commit 5 Reflection Notes
A thread pool is a group of threads that appear and are waiting and ready to handle a task. When the program receives a new task, it will assign one of the threads in the pool to that task, and that thread will process the task. The remaining threads in the pool are available to handle other incoming tasks while the first thread is processing. When the first thread finishes processing its task, it returns to the pool of idle threads, ready to handle new tasks. Thread pools allow servers to process connections simultaneously, thereby increasing server throughput.
</details>