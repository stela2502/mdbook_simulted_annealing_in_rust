
# Implement the Simulated Annealing Algorithm

The idea behind this workshop is not to teach you how to implement the simulated annealing algorithm, but to show you how you can start to use Rust.
Therefore we will use Shamit's R implementation and convert that to Rust.

## scaling of the data or "why `scale.01` in R is Fundamentally Flawed"

### The R Implementation

The scale.01 function in R is defined as follows:

```
scale.01 <- function(v){
  sc.01 <- (v - min(v)) / (max(v) - min(v))
  sc.01
}
```

At first glance, this function seems to be a simple and effective way to scale a vector of numerical values between 0 and 1. And **to state that clearly - it is perfectly valid R code**. However, there are several fundamental issues with this implementation.

### The Problems with scale.01

1. **Inefficiency:** The function computes min(v) and max(v) twice. This is unnecessary and can significantly slow down performance when applied to large vectors.

2. **Vectorized but Inefficient in Apply Calls:** The function is designed to be used in an apply() statement, which is idiomatic in R. However, repeatedly calling this function on subsets of data results in redundant computations.

3. **Mutation vs. Copying:** In R, this function creates a new vector instead of modifying the existing one in place. This is inefficient for large datasets and contrasts with best practices in languages like Rust.

## The Rust Approach

In Rust, the equivalent function would modify the existing data structure in place for efficiency. Here’s an example:

```
pub fn scale_01(&mut self) {
    for row in &mut self.data {
        let mut min = f32::INFINITY;
        let mut range = f32::NEG_INFINITY;
        
        for &value in row.iter() {
            min = min.min(value);
            range = range.max(value);
        }
        range -= min;
        for value in row.iter_mut() {
            *value = (*value - min) / range;
        }
    }
}
```

### Why the Rust Approach is Better

  1. **In-Place Modification:** The Rust function modifies the existing data structure instead of creating unnecessary copies.

  2. **Optimized Min/Max Computation:** The min and max values are computed only once per row, avoiding redundant calculations.

  3. **Memory Efficiency:** By avoiding extra allocations, the Rust implementation is more memory-efficient, making it suitable for large datasets.
 
  4. **Safety & Performance:** Rust enforces strict memory safety and avoids unintended data duplication, unlike R, where copies can be created implicitly.

### Summary
- R makes vectorized operations look simple but hides inefficiencies.
- Rust requires explicit memory management but ensures better performance and correctness.


## Calculating Cluster Energy

The R function `calc.Ek` calculates the energy of a specific cluster in a dataset. Here's the R code:

```
calc.Ek <- function(m, clus, coi){
  clus.d <- m[which(clus == coi), ]
  Ek <- sum(dist(clus.d))
  Ek
}
```

### Explanation of the R Code
1. **Selecting Rows for the Cluster**:  
   The first step in the R function is to create a new matrix, `clus.d`, that contains only the rows of the matrix `m` where the cluster label in the `clus` vector is equal to the cluster of interest `coi`. This is done using the `which()` function to find the indices of the rows that belong to the target cluster.
   
2. **Calculating the Energy**:  
   The function then calculates the energy (`Ek`) of the cluster by summing the pairwise Euclidean distances between all the rows in `clus.d`. The `dist()` function in R calculates the Euclidean distance between each pair of rows, and `sum()` is used to add these distances together.

3. **Output**:  
   The energy of the cluster (`Ek`) is returned as the result.

### How It Works in R
- R makes use of high-level functions like `which()` and `dist()`, which simplify the implementation but may introduce some overhead due to the creation of intermediate data structures.



## The Rust Implementation

The equivalent function in Rust is implemented with more explicit control over memory and data. Here's the Rust code:

```
/// Function to compute the Euclidean distance between rows of data
fn euclidean_distance(&self, i: usize, j: usize) -> f32 {
    let v1 = &self.data[i];
    let v2 = &self.data[j];
    let mut sum: f32 = 0.0;
    for i in 0..v1.len() {
        sum += (v1[i] - v2[i]).powi(2);
    }
    sum.sqrt()
}

/// Calculates the cluster energy
fn calc_ek(&self, clus: usize) -> f32 {
    let ids = self.cluster_rows(clus);
    let mut sum = 0.0;
    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            sum += self.euclidean_distance(ids[i], ids[j]);
        }
    }
    sum
}

/// Which rows are in cluster `clus`?
fn cluster_rows(&self, clus: usize) -> Vec<usize> {
    let mut ret = Vec::<usize>::with_capacity(self.clusters.len());
    for i in 0..self.clusters.len() {
        if self.clusters[i] == clus {
            ret.push(i);
        }
    }
    ret
}
```

### Explanation of the Rust Code
1. **Euclidean Distance Calculation**:  
   In Rust, the `euclidean_distance` function calculates the Euclidean distance between two rows. It takes the indices of the rows (`i` and `j`) and computes the sum of squared differences between corresponding elements in the rows. Finally, it returns the square root of the sum to get the distance.

2. **Cluster Energy Calculation**:  
   The `calc_ek` function calculates the energy of a cluster by:
   - Finding the indices of the rows that belong to the specified cluster using the `cluster_rows` function.
   - For each pair of rows in the cluster, it computes the Euclidean distance and adds it to the total sum, which represents the cluster energy.

3. **Cluster Rows**:  
   The `cluster_rows` function simply iterates over the `clusters` vector and collects the indices of the rows that belong to the specified cluster (`clus`). It returns a `Vec<usize>` containing these indices.

### How It Works in Rust
- Rust gives you more control over memory and data access. The program works with mutable references and does not create intermediate data structures unless necessary.
- The `cluster_rows` function is used to filter the rows based on the cluster, while the `euclidean_distance` function calculates the pairwise distance.

That said, there are libraries that make Rust code look more like R, but I want to use as few libraries as possible here.

---

## The main worker function


The R code provided implements the simulated annealing algorithm in the following steps:

1. **Initial Cluster Assignment**: A random assignment of K clusters to the rows of data is performed using `sample(1:K, nrow(data), replace = TRUE)`.
2. **Energy Calculation**: The function `calc.Ek()` calculates the energy of the cluster (the sum of distances between elements within the cluster).
3. **Annealing Process**: In each iteration, a row (data point) is randomly chosen and moved to a new cluster. The energy of the system is recalculated based on the new cluster assignment.
4. **Acceptance Criteria**: If the new configuration has a lower energy, it is accepted. If the energy is higher, the new configuration is accepted based on a probabilistic criterion derived from the temperature (simulating the annealing process).
5. **Cooling**: The temperature is updated in each iteration by multiplying it with a cooling factor.


```
#This is the main algorithm that performs the annealing. It takes the data,how
#many K we are looking for, The number of iterations to perform, starting
#temperature and the cooling factor.

sa.ok <- function(data,K,Iter,Temp,cool){

  clusters <- sample(1:K,nrow(data),replace = T) #initialise random clusters
  
  #clusters.o <- clusters
  
  Es.old <- calc.E.all(data,clusters)
  #E.old <- E.tot(Es.old)
 
  for(i in 1:Iter){   # start iterating 
    
    clusters.new <- clusters #copy the clusters
    #Es.new <- Es.old
    
    row.id <- sample(1:nrow(data),1) #pick a gene at random
    
    from.c <- clusters.new[row.id] # get the cluster it's moving from
    #to.c <- sample((1:K)[!(1:K) %in% from.c],1)
    to.c <- sample((1:K)[-from.c],1) # randomly choose a new cluster
    
    clusters.new[row.id] <-  to.c # replace the old cluster with the new
    
    Es.new <- Es.old #make a copy of the energies vector
    # calc the energies of the two changed clusters
    Es.new[from.c] <- calc.Ek(data,clusters.new,from.c) 
    Es.new[to.c] <- calc.Ek(data,clusters.new,to.c)
    
    E.new <- E.tot(Es.new) # calculate the new average E
    E.old <- E.tot(Es.old) # calculate the old average E
    
    if(E.new < E.old){  # if new < old accept the move copy the new clusters into the previous
      clusters <-  clusters.new #copy the new clusters into the previous
      Es.old <- Es.new # make Enew to Eold
    }else{
      
      if(calc.exp(E.new,E.old,Temp) > runif(1)){ #evaluate the exprssion against the random number from runif(1)
        clusters <- clusters.new #copy the new clusters into the previous
        Es.old <- Es.new  # make Enew to Eold
      }
    }
    
    {cat("\r",E.old)} #print out the energy to the screen
    
    Temp <- Temp*cool # cool the system
  }
  clusters # return the clusters
}
```

### Rust Implementation

The Rust version of the algorithm closely follows the same logic as the R implementation but is adapted for performance and memory safety in the Rust programming language:

1. **Random Cluster Assignment**: In Rust, clusters are assigned using `rand::random_range()` instead of the R `sample()` function.
2. **Energy Calculation**: The energy calculation (`calc_ek()`) is similar to R’s `calc.Ek()`, but it uses a more efficient loop-based calculation of Euclidean distances.
3. **Annealing Process**: The process for updating the cluster assignment and calculating energies is similar to the R version but is implemented with Rust’s ownership model and safety guarantees.
4. **Acceptance Criteria**: The acceptance criterion is implemented with the same logic, but uses Rust’s `f32::exp()` to handle the energy calculations and probabilistic decision-making.
5. **Cooling**: The temperature is updated similarly to R, using a multiplicative cooling factor.


```
    pub fn run( &mut self, max_iter:usize, cool:f32 ) -> usize {
    
      let mut it = 0;
      // calculate the inital energies - this will be modified later
      let mut old_energies= Vec::<f32>::with_capacity( self.k );
      for i in 0..self.k {
          old_energies.push( self.calc_ek( i ) );
      }
      
      let mut old_total: f32 = old_energies.iter().sum::<f32>() / self.k as f32;
      
      let mut rand = rand::rng();
      
      for _ in 0..max_iter{
          it += 1;
          // initate all varaibales
          let mut new_energies = old_energies.clone();
          let moving_row = rand.random_range(0..self.data.len());
          let move_from = self.clusters[moving_row];
          let mut move_to = rand.random_range(0..self.k);
          while move_from == move_to{
              move_to = rand.random_range(0..self.k);
          }
          // move the row from to
          self.clusters[moving_row] = move_to;
          // calculate the new energies
          new_energies[move_from] = self.calc_ek( move_from );
          new_energies[move_to] = self.calc_ek( move_to );
          
          let new_total:u32 = new_energies.iter().sum::<f32>() / self.k as f32;
          
          if new_total < old_total || 
            (-((new_total - old_total) / self.temp)).exp() > rand.random_range(0.0..1.0){
              // that is a good one - keep this
              old_energies[move_from] = new_energies[move_from];
              old_energies[move_to] = new_energies[move_to];
              old_total = new_total;
          }else {
              //this move was not good - drop it!
              self.clusters[moving_row] = move_from;
          }
          // cool the system
          self.temp *= cool;
      }
      it
    }
    
```

There is not a lot of differences in the implementation - the rust code is even one line shorter as the R one.


## And Funtions to Plot and Write the Data

I assume that by now you understand what the write_clusters function is doing.

Plotting is a lot different from R; I have just obtained that function structure from ChatGPT and fixed some errors.
Just take it as is.

```
use plotters::prelude::*;
use std::error::Error;
use std::io::Write;

    pub fn write_clusters(&self, ofile: &str, sep: char) -> Result<(), Box<dyn Error>> {
        // Open the file in write mode
        let mut file = File::create(ofile)?;

        // Write the header (optional)
        writeln!(file, "Rowname{}Cluster", sep)?;

        // Iterate over the rownames and clusters, writing them to the file
        for (rowname, cluster) in self.rownames.iter().zip(self.clusters.iter()) {
            writeln!(file, "{}{}{}", rowname, sep, cluster+1)?;
        }

        Ok(())
    }
    
    pub fn plot(&self, prefix:&str )-> Result<(), Box<dyn std::error::Error>> {
        let output_dir = Path::new(prefix).parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(output_dir)?;

        for cluster_id in 0..self.k {
            let filename = format!("{}_cluster_{}.png", prefix, cluster_id +1 );
            let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption(format!("Cluster {}", cluster_id+1), ("sans-serif", 20))
                .margin(20)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0..self.data[0].len(), 0.0f32..1.0)?; // Adjust Y range if needed

            chart.configure_mesh().draw()?;

            // Collect all rows belonging to this cluster
            let cluster_data: Vec<&Vec<f32>> = self.data.iter()
                .zip(&self.clusters)
                .filter(|&(_, &c)| c == cluster_id)
                .map(|(row, _)| row)
                .collect();

            // Draw each row as a line plot
            for row in cluster_data {
                chart.draw_series(LineSeries::new(
                    row.iter().enumerate().map(|(x, &y)| (x, y)),
                    &BLUE,
                ))?;
            }

            root.present()?;
            println!("Saved: {}", filename);
        }

        Ok(())
    }

```

The plot function needs one more library:

``` 
cargo add plotters
```

If all has gone well we could improve on our tests!
Add a second test case into the lib.rs file:

```
    #[test]
    fn tes_scale01(){
        let mut obj = SimulatedAnnealing::new( "tests/data/Spellman_Yeast_Cell_Cycle.tsv", 8, 1000.0, '\t' );
        obj.scale_01();
        let exp5:Vec<f32> = vec![0.6989,0.0000,0.0968,0.3333,0.4301,1.0000,0.7419,0.7419,0.6022,0.7634,0.1720,0.4301,0.5161,0.7634,0.6989,0.6559];
        let exp7:Vec<f32> = vec![0.0803,0.0000,0.2867,0.5849,0.9679,1.0000,0.7775,0.7156,0.5505,0.5459,0.4518,0.6193,0.8440,0.8532,0.9335,0.7752];
        let mut dist: f32 = 0.0;
        for i in 0..exp5.len() {
            assert!(
                (obj.data[5][i] - exp5[i]).abs() < 1e-4,
                "Mismatch in gene 5 at index {}: got {}, expected {}",
                i, obj.data[5][i], exp5[i]
            );
            assert!(
                (obj.data[7][i] - exp7[i]).abs() < 1e-4,
                "Mismatch in gene 7 at index {}: got {}, expected {}",
                i, obj.data[7][i], exp7[i]
            );
            dist += (obj.data[7][i]-obj.data[5][i]).powi(2);
        }
        dist = dist.sqrt();
        obj.clusters[5] = 8;
        obj.clusters[7] = 8;
        obj.k =9;
        assert_eq!( obj.calc_ek(8), dist, "the distance in cluster 8 (genes 5 and 7)" );
    }
```

```{bash eval=FALSE}
make test -r
```


{{#compile_output:step3}}


This looks good, just that we can not use this library as we have not implemented the executable :-D 


