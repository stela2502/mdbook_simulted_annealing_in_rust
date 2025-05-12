

# Using Rust to Implement the Simulated Annealing Algorithm

This tutorial is for experienced bioinformaticians who have been programming in R and/or Python, or other scripting languages. If you have experience with C, C++, or Julia, that will also be beneficial.

While R is widely used in bioinformatics for statistical analysis and data manipulation, Rust is a **systems programming language** that provides **strong type safety, memory safety without garbage collection**, and **high performance**. This tutorial will introduce key differences between R and Rust and guide you through implementing a simulated annealing algorithm in Rust.

---

## Rust vs. R: Key Differences

Before diving into the implementation, let's highlight a few **fundamental differences between R and Rust** that will be important for this tutorial:

| Feature        | R (Scripting Language)        | Rust (Systems Language) |
|---------------|-----------------------------|-------------------------|
| **Typing**    | Dynamic (e.g., `x <- 10` can later be `x <- "text"`) | Static (types must be defined, e.g., `let x: i32 = 10;`) |
| **Memory Management** | Garbage collection | Ownership and borrowing |
| **Performance** | Optimized for high-level analysis | Compiled, low-level efficiency |
| **Object-Oriented Features** | R has S3, S4, and R6 classes but isn't purely OO | Rust supports structs and traits for OO-like behavior |
| **Concurrency** | Limited parallel processing | Strong concurrency model with `async` and threads |

Understanding these differences will help when translating concepts from R to Rust.

---

## **A Quick Refresher: Object-Oriented Programming (OO)**
Although **R is not strictly an object-oriented language**, it does support OO through **S3, S4, and R6 classes**. Rust, on the other hand, does not have traditional classes like Python or Java but instead **uses structs and traits** to achieve similar functionality.

Here’s a quick comparison of how **objects and methods** are implemented in R and Rust:

### **R Example: Object-Oriented Approach Using S3**
```r
# Define an S3 object
cluster <- list(centroid = c(2.3, 3.4), members = c(1, 5, 9))
class(cluster) <- "Cluster"

# Define a method for the Cluster class
print.Cluster <- function(obj) {
  cat("Cluster centroid at:", obj$centroid, "
")
}

# Use the method
print(cluster)
```

### **Rust Equivalent: Using Structs and Traits**
```rust
struct Cluster {
    centroid: (f64, f64),
    members: Vec<u32>,
}

impl Cluster {
    fn print(&self) {
        println!("Cluster centroid at: {:?}", self.centroid);
    }
}

fn main() {
    let cluster = Cluster {
        centroid: (2.3, 3.4),
        members: vec![1, 5, 9],
    };

    cluster.print();
}
```

### **Key Takeaways**
- **R uses lists and S3/S4 classes for OO design**, whereas **Rust uses structs and traits**.
- **Methods in R** are just functions that check an object's class, while **Rust uses `impl` blocks** to define methods.
- **Rust enforces type safety at compile time**, while **R allows more flexibility** at the cost of runtime type checking.

---

## **The Problem: Clustering Expression Data Using Simulated Annealing**
The algorithm we will implement is based on an **R programming exercise** given to students. It clusters gene expression data using **simulated annealing**, a probabilistic optimization method. This algorithm is **simple to understand and implement**, but it also touches on several **important programming concepts**, including:
- **File reading**
- **Control flow (loops, conditionals)**
- **Data structures (vectors, matrices, hashes)**
- **Mathematical operations**
- **Performance considerations**

### **The Data**
For this problem, we will use **yeast cell-cycle data**, which is a small **time-course dataset** consisting of **250 genes across 16 timepoints**. The data is available here:  
🔗 [Spellman_Yeast_Cell_Cycle.tsv](https://github.com/shambam/R_programming_1/blob/main/Spellman_Yeast_Cell_Cycle.tsv)  

#### **Biological Context**
Yeast in liquid culture were synchronized at the **same cell-cycle phase** and then released. They underwent **two synchronized divisions**, and samples were taken at **16 timepoints**. Since genes involved in the same biological processes tend to have similar expression patterns, this dataset contains **distinct clusters of co-regulated genes**. Our task is to **cluster genes based on their expression profiles** using simulated annealing.


## Simulated annealing

We can think of well clustered data having low energy, in that each cluster is tight and has little within cluster variance. If we calculate the variance *within* each cluster and sum over all clusters we get the total variance (energy) of the system. To measure the distance between two genes $i$ and $j$ over $t$ timepoints we do:

$$d_{ij}=\sqrt{\sum{(g^{i}_t-g^{j}_t)^2}}$$

so to measure the energy (total variance) of a clustering we sum the pairwise distances for each cluster $K$, and then sum over all $K$s and them divide by $K$.

$$ E(K)=\frac{1}{K}\sum^K_{k=1} \left[ \sum_{i\epsilon Ck}\sum_{j\epsilon Ck} d_{ij}\right] $$

For a well clustered data, $E(K)$ should be as **small** as possible. Lets say we have 1000 genes, and we want to partition them into 10 clusters. The number of combinations is too high for us to try each one to brute force a true $E$. This is why we use a *heuristic* algorithm to get us as close to the solution as possible in a smaller amount of time.

If we tried to visualise the energy landscape we can imagine it might look something like this:

<p align="center">
  <img src="images/EnergyLandscape.png" alt="Energy Landscape" width="60%">
</p>

The idea behind simulated annealing is that "bad" moves are also allowed for a proportion of the iterations allowing exploration of the energy landscape, thereby avoiding local minima.

## The Algorithm

I do not want to focus too much onto the algoirithm here as the main focus is on the Rust implementation, but the steps that need to be run are as follows:

  1. Load the data and scale it so each gene's value lie between 0 and 1
  2. Create a random cluster information and calculate the energy of this clustering
  3. Randomly shift any gene from it's cluster to another
  4. Calculate the new energy and check if the new cluster info should be kept; do that
  5. Repeat 3 and 4 for n iterations
  6. report the cluster information - if possible create plots
  
And all of that in Rust ;-)

