

# Rust Program Logioc is Implemented in "main.rs"

If you compile the library as it is you will not be able to use it.
We need to implement the main.rs file that codes for the binary.

The R version of what we implement here is this:
```r

ycc <- read.delim("../Spellman_Yeast_Cell_Cycle.tsv",row.names=1,header=T,sep="\t")
ycc <- as.matrix(ycc)
ycc.01 <- t(apply(ycc,1,scale.01))




system.time(proc.time(clus <- sa.ok(ycc.01,10,25000,20,0.995)))

# plot the clusters using base R
par(mfrow=c(3,4))

for(i in 1:max(clus)){
  
  ycc.c <- ycc.01[which(clus==i),]
  plot(ycc.c[1,],ty="l",ylim=range(ycc.c))
  apply(ycc.c,1,lines)
  
}
```

But as we implement this functionality as executable it would be better to not hardcode the options:

Command line options are normally parsed using the clap crate so we should add that to our project.
This time I want a specific version:

```
cargo add clap@3.0.14 --features derive
```

We need to get the input table, the separating char for that table, the number of clusters to search for, the starting temperature, cool-down factor and max iterations to run through as well as the file we want to save the clusters to. In Rust you would do this like that:

```rust
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the input text table
    #[clap(short, long)]
    file: String,
    /// the column separator for the file
    #[clap(default_value= "\\t",short, long)]
    sep: String,
    /// the number of clusters
    #[clap(short, long)]
    clusters: usize,
    /// the starting temperature
    #[clap(default_value_t= 20.0,short, long)]
    temp: f32,
    /// the cooling factor
    #[clap(default_value_t= 0.9995,short, long)]
    cool: f32,
    ///max number of iterations
    #[clap(default_value_t= 1000*1000,short, long)]
    max_it: usize,
    /// the grouping outfile
    #[clap(short, long)]
    outfile: String,
}
```

Here's a breakdown of what it's doing:

1. **use clap::Parser;:** This imports the necessary functionality from the clap crate, which is used for parsing command-line arguments.

2. **#[derive(Parser)]:** This macro automatically implements the Parser trait for the Opts struct. This allows the struct to be used for argument parsing in the CLI.

3. **#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]:** This defines metadata for the program such as its version and author, which will be shown when the user runs the command with a --version or --help flag.

4. **struct Opts { ... }:** The Opts struct holds the various command-line arguments that the program will accept.
    Each three lines corresponds to a command-line argument with hep srting, options and variable definition.

The different clap options we are using here mean

 - **short** use the variable names first character for a short option like -f for file
 - **long** use the variable name as long option line --file for file
 - **default_value** the default String value if the user does not specify one
 - **default_value_t** used for default values that are not a String

The main function is quite simple — half of it is just calculating the time taken for the program to execute. Give it a try!

```rust
use std::time::SystemTime;
// this is specific for my package which I have called simulated_annealing_new as I had an other version, too.
use simulated_annealing::SimulatedAnnealing;

fn main() {
    let now = SystemTime::now();
    
    let opts: Opts = Opts::parse();

    let mut sep = '\t';
    if &opts.sep != "\\t"{
        //println!("I set sep to {}", opts.sep );
        sep = opts.sep.chars().next().unwrap(); 
    }

    let mut sim = SimulatedAnnealing::new( &opts.file, opts.clusters, opts.temp, sep );
    sim.scale_01();    

    //println!("Initial state: {sim}");

    let iterations = sim.run( opts.max_it, opts.cool );

    let _= sim.plot( &opts.outfile );

    //println!("Final state {sim} in {iterations} iterations");

    match sim.write_clusters( &opts.outfile, sep ){
        Ok(_) => println!("Clusters written to {}", &opts.outfile ),
        Err(e) => eprintln!("Failed to write the data to {}: {:?}", &opts.outfile, e),
    }

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            eprintln!("finished in {milli} h {min} min {sec} sec {mil} milli sec");
        },
        Err(e) => {println!("Error: {e:?}");}
    }
}

```

Now compile and test it:

```{bash}
cargo test -r
```

{{#compile_output:step4}}


## We have an Executable!!

You now can run the executable target/release/simulated_annealing like that:

```{bash}
target/release/simulated_annealing  -f tests/data/Spellman_Yeast_Cell_Cycle.tsv --clusters 8 --temp 20 --outfile /tmp/clusters.tsv --max-id 10000
```

```text

Saved: /tmp/clusters.tsv_cluster_1.png
Saved: /tmp/clusters.tsv_cluster_2.png
Saved: /tmp/clusters.tsv_cluster_3.png
Saved: /tmp/clusters.tsv_cluster_4.png
Saved: /tmp/clusters.tsv_cluster_5.png
Saved: /tmp/clusters.tsv_cluster_6.png
Saved: /tmp/clusters.tsv_cluster_7.png
Saved: /tmp/clusters.tsv_cluster_8.png
Saved: /tmp/clusters.tsv_cluster_9.png
Saved: /tmp/clusters.tsv_cluster_10.png
Clusters written to /tmp/clusters.tsv
finished in 0 h 0 min 0 sec 189 milli sec

```


## Implement a 'print' for our Class

The output from our simulation is missing key information — we want to display the system's energy before and after processing.

In an R S3 class, we would implement a custom print method like this:

```r
print.simulatedAnnealing <- function(x) {
  # Format and display info about x
}
````

This allows you to simply call `print(x)` and get a meaningful summary of the object.

Rust doesn't have classical inheritance like R's S3/S4 or R6 systems. Instead, Rust uses **traits**, which act like interfaces or capabilities. A type can implement a trait to gain specific behavior — similar to adding methods in R.

To define a custom print format in Rust, we implement the `Display` trait:

```rust
use std::fmt;

impl fmt::Display for SimulatedAnnealing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sum = 0.0_f32;
        for i in 0..self.k {
            sum += self.calc_ek(i);
        }
        sum /= self.k as f32;
        write!(
            f,
            "SimulatedAnnealing (temperature: {:.2}, total energy: {:.2} for {} clusters)",
            self.temp, sum, self.k
        )
    }
}
```

This gives the struct a user-friendly string representation, just like implementing `print()` for a class in R.


Afterwards we can change our main.rs and add the print statements:

```rust
fn main() {
    let now = SystemTime::now();
    
    let opts: Opts = Opts::parse();

    let mut sep = '\t';
    if &opts.sep != "\\t"{
        //println!("I set sep to {}", opts.sep );
        sep = opts.sep.chars().next().unwrap(); 
    }

    let mut sim = SimulatedAnnealing::new( &opts.file, opts.clusters, opts.temp, sep );
    sim.scale_01();    

    println!("Initial state: {sim}");

    let iterations = sim.run( opts.max_it, opts.cool );

    let _= sim.plot( &opts.outfile );

    println!("Final state {sim} after {iterations} iterations");

    match sim.write_clusters( &opts.outfile, sep ){
        Ok(_) => println!("Clusters written to {}", &opts.outfile ),
        Err(e) => eprintln!("Failed to write the data to {}: {:?}", &opts.outfile, e),
    }

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            eprintln!("finished in {milli} h {min} min {sec} sec {mil} milli sec");
        },
        Err(e) => {println!("Error: {e:?}");}
    }
}
```

I have not modified my examples here - so this is something you can do all for yourself.
If we have more time we could implement one more improvement to the library:

Add a store variable into the class and populate it with the row to row distances after you have scaled the data.
Then modify the calc_ek() function to use the values from the store and not calculate the distances every single time.
This should give you an additional 5x speed improvement if you store it as a Vec<Vec<f32>> again. If you use a single vector and an index function you could get an even better performance.

## Take Home Message

The implementation of the functions is significantly different from R or Python, but with the compiler and the AI assistance we can get at the moment it, is doable. Coding in Rust takes time to get into, but the speed improvements can be worth it.

This example here does not really highlight the usability as the R code also finished in a reasonable amount of time, but if you have more complicated tasks - like processing BAM files or anything else that is (1) either easy to implement in a multiprocessor system or (2) needs to process binary data, it is worth to look into Rust. It is somewhat slower than C and C++ (like 3 to 5 times?), but Rust is so much easier to program in if you come from R and Python - it is worth it.

By the way - I used a ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> for my store and gained another speed boost of factor 2. Given the fact that this takes either 60 or 30 milliseconds this is no big deal. For the sake of this tutorial I think a Vec<Vec<_>> is easier to work with. You can check out my other implementation [here](https://github.com/stela2502/simulated_annealing).


