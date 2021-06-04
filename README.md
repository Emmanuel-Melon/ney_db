# NeyDB
## Project Overview
NeyDB is a distributed Key-Value Store that was inspired by Cassandra's architecture.  I decided to build this project in order to solidifiy my understanding of Distributed Systems by building an actual project.

## Goals
Not very sure why I wanna build this but
- Optimized for Writes
- DataTypes
- Data Access Mechanism
## Features
### Data Persistence
This application persists data in the File System.
### Cluster Support
This application stores data in multiple nodes.
### Fault Tolerance
This application is fault tolerant and the failure of one Node doesn't affect the others.
## Structure
### API
You can interact with NeyDB via the following interfaces
### Write API
Allows you to insert items into the database.
### Read API
Allows you to read items from the database.
### Storage Mechanism
File System is used to store data.
### Nodes and Connections
Communication is enabled via a REST interface.  
gRPC to be added in the near future.

## Architecture
This application's Architecture is publicly available at.
### Data Storage
Disk Storage
### Underlying Data Structures
Hash Tables
## Technology Stack
- Rust
- REST
## Running the application
### Building the application
- MacOS
- Linux
### Prerequisites
Have Rust installed.

## References & Guides
### Reading Material
I wouldn't have been able to build this application without these amazing resources.
- [Some Notes on Distributed Key Stores](https://randomfoo.net/2009/04/20/some-notes-on-distributed-key-stores)
- [Implementing a Key-Value Store](http://codecapsule.com/2012/11/07/implementing-a-key-value-store-part-1-what-are-key-value-stores-and-why-implement-one/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Key Value Stores: A Practical Overview](http://blog.marc-seeger.de/2009/09/21/key-value-stores-a-practical-overview/)
### Examples and Projects
- [Siddon Tang - TiKV - building a distributed key-value store with Rust](https://www.youtube.com/watch?v=mSRwr7I-ZOg)
- [Rust at speed — building a fast concurrent database](https://www.youtube.com/watch?v=s19G6n0UjsM)
- [The NoSQL Ecosystem](http://www.aosabook.org/en/nosql.html)
- [Building a Distributed Fault-Tolerant Key-Value Store](http://blog.fourthbit.com/2015/04/12/building-a-distributed-fault-tolerant-key-value-store/)
- [Design a Key-Value Store (Part I)](http://blog.gainlo.co/index.php/2016/06/14/design-a-key-value-store-part-i/)
- [An Approach to Designing a Distributed, Fault-Tolerant, Horizontally Scalable Event Scheduler](https://medium.com/walmartlabs/an-approach-to-designing-distributed-fault-tolerant-horizontally-scalable-event-scheduler-278c9c380637)
## Contributions
For contributions, please checkout the `contributions.md` file for guides and code of conduct.
## Authors
- [Emmanuel Daniel](https://github.com/Emmanuel-Melon)
