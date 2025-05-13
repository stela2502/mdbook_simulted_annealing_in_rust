# Create a Rust Project and Implement the Class

A Rust **class** is a combination of a struct- (data) and an impl-block (functions). But as in R a Rust **package** contains several other important files and a specific folder structure.

As an R programmer, you're likely familiar with using tools like devtools to streamline the creation of R packages. In Rust, the process of setting up a new package is just as straightforward, though the terminology and structure differ slightly. 

This will create the folder, populate it with some files and prepare it for git usage.
```
cargo new --bin simulated_annealing
```
For later use we create a **simulated_annealing/tests** folder and a **simulated_annealing/src/lib.rs** file.

I normally put as much code into the library and keep the script as slender as possible.

## For the class - which data structures do we need?

 1. The 'expression' data
 2. The rows <-> cluster connections
 3. The number of clusters asked for
 4. The actual temperature of the system
 
** Let's create the object**

Open the src/lib.rs file and add this:

```rust,no_run

pub struct SimulatedAnnealing {
    /// the normalized expression data
    pub data: Vec<Vec<f32>>,
    /// the names for each row
    rownames: Vec<String>,
    /// the cluster ids for each row of data
    clusters: Vec<usize>,
    /// expected clusters k
    k: usize,
    /// the actual temerature
    temp:f32,
}

impl SimulatedAnnealing {
    pub fn new( file_path:&str, k:usize, temp:f32, split:char ) -> Self{

        let (rownames, data) = Self::read_table_with_names( file_path, split ).unwrap();

        let clusters: Vec<usize> = rownames.iter().map(|_| rand::thread_rng().gen_range(1..=k)).collect();

        Self{
            data,
            rownames,
            clusters,
            clusters_energy: vec![0.0; k ],
            temp,
        }
    }
}

```

This will define a class with the variables data, rownames, clusters, k and temp as well as the new() function for this class, which in turn populates these variables with initial values. Save the file and try to compile your project using

```bash
cargo test -r
```

{{#compile_output:step1}}

### missing library error:

Right - we try to create the random cluster info and have not loaded the required package. That is simple to do in Rust:

```
cargo add rand
```

This will download and compile the rand crate and add the rand package as depencancy in Cargo.toml.
Do not forget to also load that library in the lib.rs file - at the top of that file add ``use rand::Rng;``.

### missing function error:

The read_table function is more complicated.
I normally out-source these simple, but tedious steps to ChatGPT or an other AI helper, but these are the steps we need to take:

 1. create a Path from the `&str` file_name
 2. open that Path
 3. create a BuffReader from that File to more efficiently read from it
 4. iterate over the lines and
 5. split the line by sep (could be user defined too - or?)
 6. use the first entry as rowname (String) and the rest as numeric (f32). 

To fix the issues you need load these libraries at the beginning of the lib.rs file:

```rust,no_run
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BuffRead};
use rand::Rng;
```
And then add the following rust function into the impl block:
```rust,no_run
    pub fn read_table_with_names(file_path: &str, split: char ) -> Result<( Vec<String>, Vec<Vec<f32>>), String> {
        let path = Path::new(file_path);
        let file = match File::open(path){
          Ok(f) => f,
          Err(e) => {
            return Err( format!("Failed to open file: {}", e) )
          }
        };
        let reader = BufReader::new(file);

        let mut data = Vec::new();
        let mut rownames= Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| format!("Error reading line {}: {}", line_num + 1, e))?;
            let mut parts = line.split( split );

            let row_name = parts.next().ok_or_else(|| format!("Missing row name at line {}", line_num + 1))?;
            if row_name == "" {
                // ignore column names
                continue;
            }
            rownames.push( row_name.to_string() );

            let values: Result<Vec<f32>, String> = parts
                .map(|num| num.parse::<f32>().map_err(|_| format!("Invalid number '{}' at line {}", num, line_num + 1)))
                .collect();

            let values = values.unwrap(); // will die on error
            data.push( values );
        }

        Ok( (rownames,data) )
    }
```

**Understanding `read_table_with_names`**

If you're an R developer, think of it as a combination of `read.table()` with custom handling for row names and numeric conversion.

## Function Overview

### Function Signature
```rust
pub fn read_table_with_names(file_path: &str, split: char ) -> Result<( Vec<String>, Vec<Vec<f32>>), String>
```

This function:
- Takes a **file path** (`file_path: &str`) as a string reference.
- Accepts a **delimiter character** (`split: char`) to separate values.
- Returns a **Result type** containing either:
  - A tuple of **row names** (`Vec<String>`) and **numeric data matrix** (`Vec<Vec<f32>>`).
  - Or an error message (`String`) in case of failure.

## Breaking It Down

### Step 1: Open the File
```rust
let path = Path::new(file_path);
let file = match File::open(path){
    Ok(f) => f,
    Err(e) => {
        return Err(format!("Failed to open file: {}", e));
    }
};
```
**What it does:**
- Converts `file_path` into a `Path` object.
- Tries to open the file using `File::open(path)`. If successful, it assigns the file handle to `f`.
- If the file opening fails, it returns an error message.

**R Equivalent:**
```r
if (!file.exists(file_path)) {
  stop(paste("Failed to open file:", file_path))
}
```

### Step 2: Prepare the Variables
```rust
let reader = BufReader::new(file);
let mut data = Vec::new();
let mut rownames = Vec::new();
```
**What it does:**
- Wraps the file handle in `BufReader` for efficient line-by-line reading.
- Initializes empty vectors:
  - `data`: Stores numeric values (like a matrix in R).
  - `rownames`: Stores row names.

### Step 3: Parse Each Line
```rust
for (line_num, line) in reader.lines().enumerate() {
    let line = line.map_err(|e| format!("Error reading line {}: {}", line_num + 1, e))?;
    let mut parts = line.split(split);
```
**What it does:**
- Iterates over each line, keeping track of line numbers.
- Splits the line into separate values based on `split` (e.g., `\t` for TSV, `,` for CSV).
- Handles file reading errors gracefully.

### Step 4: Extract Row Names
```rust
let row_name = parts.next().ok_or_else(|| format!("Missing row name at line {}", line_num + 1))?;
if row_name == "" {
    continue; // Ignore column names
}
rownames.push(row_name.to_string());
```
**What it does:**
- Retrieves the first value from the line as the **row name**.
- If no row name is found, it returns an error.
- Skips empty row names (useful for ignoring headers).
- Adds valid row names to `rownames`.

**R Equivalent:**
```r
data <- read.table(file_path, sep="\t", header=TRUE, row.names=1)
rownames <- rownames(data)
```

### Step 5: Convert Remaining Values to `f32`
```rust
let values: Result<Vec<f32>, String> = parts
    .map(|num| num.parse::<f32>().map_err(|_| format!("Invalid number '{}' at line {}", num, line_num + 1)))
    .collect();
let values = values.unwrap();
data.push(values);
```
**What it does:**
- Attempts to convert each remaining value in the row into `f32`.
- If a value is invalid, an error message is returned.
- Pushes the successfully parsed row into `data`.

**R Equivalent:**
```r
data_matrix <- as.matrix(data)
data_matrix <- apply(data_matrix, 2, as.numeric) # Convert to numeric
```

### Step 6: Return the Parsed Data
```rust
Ok((rownames, data))
```
- If everything succeeds, returns a tuple: **(row names, numeric matrix)**.
- The `Result` type ensures proper error handling.

### Why are there so many ways to create the error? 

In short Rust has two classes to handle errors: a **Result** that allows for an error being returned if something fails and a **Option** which only allows None to be returned if something fails. Error handling is discussed in more detail in the [Appendix](80-ErrorHandling.md).

So if we re-compile this - will it work? If not just follow the compiler's help ;-)


## Adding a Test for `read_table_with_names`

**Why Testing Matters**

Currently, our function compiles, but without a test, we cannot verify correctness. Unit testing ensures the function behaves as expected under various conditions.

## Implementing a Unit Test
Unit tests should be included at the end of the same file where the function is defined - there they can access private functions and variables of the class.

```rust,no_run
#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_read_data() {
        match read_table_with_names("tests/data/Spellman_Yeast_Cell_Cycle.tsv", '\t') {
            Ok((rownames, data)) => {
                assert_eq!(data.len(), 256, "Expected 256 rows");
                assert_eq!(data[0].len(), 16, "Expected 16 columns");
                assert_eq!(rownames.len(), data.len(), "Row names should match row count");
            }
            Err(e) => {
                panic!("Could not read the TSV file: {e}");
            }
        }
    }
}
```

### Explanation
- Uses Rust's built-in testing framework (`#[test]` attribute).
- Calls `read_table_with_names()` with a sample TSV file.
- Asserts expected row and column counts, ensuring data integrity.
- Panics with an error message if file reading fails.

By adding this test, we verify that our function properly reads tabular data and correctly extracts row names and numeric values. I have a complete chapter on testing in Rust: \@ref(testing). 

For this to work we need to create the folder in our package and download [this file](https://github.com/shambam/R_programming_1/raw/refs/heads/main/Spellman_Yeast_Cell_Cycle.tsv) there.

```
mkdir -p tests/data
wget https://github.com/shambam/R_programming_1/raw/refs/heads/main/Spellman_Yeast_Cell_Cycle.tsv -O tests/data/Spellman_Yeast_Cell_Cycle.tsv
```

Now you can compile and test that the file is read correctly:

```{bash}
cargo test -r
```

{{#compile_output:step2}}


Cool! First class first class function and first test - and everything is working - or?

Lets also add a test for the new() function. This function should be accessible from outside and therefore we should create a new test file. Create the file ``tests/test-SimulatedAnnealing.rs`` and fill it with this:

```rust,no_run
use simulated_annealing::SimulatedAnnealing; // load our library

#[test]
fn test_simulated_annealing() {
    let sa = SimulatedAnnealing::new( "tests/data/Spellman_Yeast_Cell_Cycle.tsv", 10, 200.0, '\t' );
    assert_eq!(sa.data.len(), 256, "we have 256 rows");
    assert_eq!(sa.data[0].len(), 16, "we have 16 cols");
}
```

Test this again.

So all that is left is to implement the main simulated annealing algorithm!
 
