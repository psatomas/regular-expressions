/*
We've received a batch of server logs that
detail user requests to our API server. A
sample log entry looks like this:
 
[2025-06-15 14:32:10] INFO: User 127.0.0.1 requested /api/orders/12345
 
Each entry includes the following pieces of
information:
- A timestamp (ex: 2025-06-15 14:32:10)
- A category (ex: INFO)
- An endpoint (ex: orders)
- A product id (ex: 12345)
 
Our program has a LogEntry struct defined with
4 fields matching the 4 pieces of information
above.
 
Using RegEx, create a vector of LogEntry
structs where each LogEntry will store the
respective fields of data. Print out
the vector in pretty-print format.
 
There are also a few invalid rows of data.
They may not have all the pieces of information
or they may consist of random symbols like !!!!.
Exclude these rows from the operations.
 
The output should look like this:
 
[
    LogEntry {
        timestamp: "2025-06-15 14:32:10",
        category: "INFO",
        endpoint: "orders",
        id: "12345",
    },
    LogEntry {
        timestamp: "2025-06-15 14:33:02",
        category: "ERROR",
        endpoint: "users",
        id: "9876",
    },
    LogEntry {
        timestamp: "2025-06-15 14:34:55",
        category: "DEBUG",
        endpoint: "products",
        id: "999",
    },
    LogEntry {
        timestamp: "2025-06-15 14:35:01",
        category: "INFO",
        endpoint: "orders",
        id: "54321",
    },
    LogEntry {
        timestamp: "2025-06-15 14:35:45",
        category: "ERROR",
        endpoint: "products",
        id: "4567",
    },
    LogEntry {
        timestamp: "2025-06-15 14:36:12",
        category: "WARN",
        endpoint: "users",
        id: "101",
    },
    LogEntry {
        timestamp: "2025-06-15 14:37:09",
        category: "INFO",
        endpoint: "categories",
        id: "88",
    },
    LogEntry {
        timestamp: "2025-06-15 14:38:42",
        category: "INFO",
        endpoint: "orders",
        id: "11111",
    },
    LogEntry {
        timestamp: "2025-06-15 14:40:20",
        category: "ERROR",
        endpoint: "orders",
        id: "00000",
    },
]
*/
 
#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    category: String,
    endpoint: String,
    id: String,
}
 
fn main() {
    let server_logs = vec![
        "[2025-06-15 14:32:10] INFO: User 127.0.0.1 requested /api/orders/12345",
        "[2025-06-15 14:33:02] ERROR: Failed to fetch /api/users/9876",
        "[2025-06-15 14:34:55] DEBUG: User 10.0.0.1 requested /api/products/999",
        "!!!!",
        "[2025-06-15 14:35:01] INFO: User 192.168.1.5 requested /api/orders/54321",
        "[2025-06-15 14:35:45] ERROR: Failed to fetch /api/products/4567",
        "!!!!",
        "[2025-06-15 14:36:12] WARN: User 10.0.0.99 requested /api/users/101",
        "[2025-06-15 14:37:09] INFO: User 127.0.0.1 requested /api/categories/88",
        "[2025-06-15 14:37:59] WARN: Malformed URL detected",
        "[2025-06-15 14:38:42] INFO: User 172.16.0.10 requested /api/orders/11111",
        "[2025-06-15 14:39:00] DEBUG: Skipping health check ping",
        "[2025-06-15 14:40:20] ERROR: Failed to fetch /api/orders/00000",
        "!!!!",
    ];

    let re = Regex::new(r"\[(\d{4}-\d{2}-\d{2}\d{2}:\d{2}:\d{2})\] (\w+):.*?/api/(\w+)/(\d+)").unwrap();

    let entries: Vec<_> = server_logs
        .into_iter()
        .filter_map(|log| re.find(log))
        .collect();

    println!("{entries:#?}");
}