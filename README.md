# Key-value store application 

Course outline: https://github.com/pingcap/talent-plan

## Project spec

The cargo project, `kvs`, builds a command-line key-value store client called
`kvs`, which in turn calls into a library called `kvs`.

The `kvs` executable supports the following command line arguments:

- `kvs set <KEY> <VALUE>`

  Set the value of a string key to a string

- `kvs get <KEY>`

  Get the string value of a given string key

- `kvs rm <KEY>`

  Remove a given key

- `kvs -V`

  Print the version

The `kvs` library contains a type, `KvStore`, that supports the following
methods:

- `KvStore::set(&mut self, key: String, value: String)`

  Set the value of a string key to a string

- `KvStore::get(&self, key: String) -> Option<String>`

  Get the string value of the a string key. If the key does not exist,
  return `None`.

- `KvStore::remove(&mut self, key: String)`

  Remove a given key.

The `KvStore` type stores values in-memory, and thus the command-line client can
do little more than print the version. The `get`/ `set` / `rm` commands will 
return an "unimplemented" error when run from the command line. Future projects 
will store values on disk and have a working command line interface.
