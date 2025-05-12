# 🧬 Intro to Traits in Rust (For Bioinformaticians)

In Rust, traits are like interfaces in other languages — they define a shared set of behavior that different types can implement.

Think of traits like protocols that say:

    *“If you implement this trait, you must define how these functions behave.”*

This is especially powerful in bioinformatics where you may want to define a common interface for different types of genomic data, file formats, or processing strategies.

For example, you might have different structs for processing a GTF or a BED file — but if they both implement a FeatureMatcher trait, you can treat them the same in generic code.

## 🧪 Trait Example

Let's say we want to define a trait for something that can match a read to a gene feature:

```rust

pub trait FeatureMatcher { 

/// prepare the class for ordered process_feature calls
fn init_search(&self, chr: &str, start: usize, iterator: &mut ExonIterator) -> Result<(), QueryErrors>; 

/// implements a Class specific way to postprocess multiplets
fn extract_gene_ids(&self, read_result: &Option<Vec<ReadResult>>, data: &ReadData, mapping_info: &mut MappingInfo) -> Vec<String>; 

/// the main function processing one (paired) BAM entry
fn process_feature( &self, data: &(ReadData, Option<ReadData>), mutations: &Option<MutationProcessor>, iterator: &mut ExonIterator, exp_gex: &mut SingleCellData, exp_idx: &mut IndexedGenes, mut_gex: &mut SingleCellData, mut_idx: &mut IndexedGenes, mapping_info: &mut MappingInfo, match_type: &MatchType, ); 

} 
```

Now, any struct (like GTF or BedData) that implements FeatureMatcher can be used generically:

```
 fn process_chunk<T: FeatureMatcher + Sync + Send>(...) { ... } 
```

This is powerful for writing reusable code — especially for large-scale pipelines, format conversions, or multimodal analysis tools.

### 🧵 What Are `Send` and `Sync` — and Why Do We Need Them?

When writing parallel code in Rust — which is common in bioinformatics for speeding up large data analyses — the compiler needs to **guarantee thread safety**.

Rust uses two special \"auto traits\" to track this:

#### ✅ `Send`
A type is `Send` if it’s safe to **move** it to another thread.

Think: *\"Can I pass this thing into a thread and not worry about it blowing up?\"*

#### ✅ `Sync`
A type is `Sync` if it’s safe to **share a reference** (`&T`) between threads.

Think: *\"Can multiple threads safely read this at the same time?\"*

---

### 🧪 Why It Matters for Traits

When we define a trait like `FeatureMatcher`, and we want to use it in multithreaded code (e.g., via `Arc<dyn FeatureMatcher>`), Rust needs to know:

> “Is this trait object safe to move or share between threads?”

To answer that, we add these trait bounds:

```rust
pub trait FeatureMatcher: Send + Sync {
    ...
}
```
This ensures that any struct implementing `FeatureMatcher` is guaranteed to be thread-safe — making your code safer, faster, and more parallel-friendly by default.


## 💡 Why Traits Matter in Bioinformatics

**Traits let you:**

* Design modular code where the implementation details are abstracted away
* Work with different formats using a unified API
* Enable parallelization safely with Sync + Send
* Write testable and flexible pipelines